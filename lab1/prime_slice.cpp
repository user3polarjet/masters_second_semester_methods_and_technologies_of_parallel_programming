#include <cstdint>
#include <string>
#include <vector>
#include <array>
#include <chrono>
#include <thread>

#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <string.h>
#include <pthread.h>

#define MY_STRINGIFY_IMPL(...) #__VA_ARGS__
#define MY_S(...) MY_STRINGIFY_IMPL(__VA_ARGS__)
#define MY_CONCAT_INTERNAL(a, b) a ## b
#define MY_CONCAT(a, b) MY_CONCAT_INTERNAL(a, b)
#define MY_FOR_RANGE(type, name, mmin, mmax) for(type name = (mmin); name < (mmax); name++)
#define MY_FOR_RANGE_ZERO(name, mmax) for(auto name = static_cast<typename std::remove_cv<decltype(mmax)>::type>(0); name < (mmax); name++)

#define MY_ASSERT(expr) \
    do {\
        if(!(expr)) {\
            fprintf(stderr, "%s:%d: %s: assertion `%s` failed.\n", __FILE__, __LINE__, __FUNCTION__, #expr);\
            abort();\
        }\
    } while(0)

#define MY_ASSERT_EXT(expr, ...) \
    do {\
        if(!(expr)) {\
            fprintf(stderr, "%s:%d: %s: assertion `%s` failed.\n", __FILE__, __LINE__, __FUNCTION__, #expr);\
            fprintf(stderr, __VA_ARGS__);\
            abort();\
        }\
    } while(0)

#define MY_ASSERT_NOT_LESS_ZERO(expr) \
    do {\
        if((expr) < 0) {\
            fprintf(stderr, "%s:%d: %s: errno: %d: strerror: %s: assertion `%s' failed.\n", __FILE__, __LINE__, __FUNCTION__, errno, strerror(errno), #expr);\
            abort();\
        }\
    } while(0)

#define MY_LOG_DEBUG_SINGLE(fmt) \
    do {\
        fprintf(stderr, "[%s:%d] [%s] [%ld] [" fmt "]\n", __FILE__, __LINE__, __FUNCTION__, time(nullptr));\
    } while(0)

#define MY_LOG_DEBUG(fmt, ...) \
    do {\
        fprintf(stderr, "[%s:%d] [%s] [%ld] [" fmt "]\n", __FILE__, __LINE__, __FUNCTION__, time(nullptr) __VA_OPT__(,) __VA_ARGS__);\
    } while(0)

#define MY_PRINT_EXPR_IMPL(expr, format)\
    do {\
        MY_LOG_DEBUG("`"#expr"`: `" format "`", (expr));\
    } while(0)


template <typename F>
struct defer_t {
    F f;
    ~defer_t() { f(); }
};
#define defer(code) const defer_t MY_CONCAT(_defer_, __LINE__){[&](){ code; }}

#define MY_ARRAY_COUNT(arr) (sizeof(arr) / sizeof(arr[0]))

#define MY_PTHREAD_BARRIER_WAIT(barrier)\
    do {\
        switch(pthread_barrier_wait(barrier)) {\
            case 0:\
            case PTHREAD_BARRIER_SERIAL_THREAD: {\
                break;\
            }\
            default: {\
                MY_ASSERT(false);\
                break;\
            }\
        }\
    } while(0)

template<typename T>
static std::chrono::system_clock::duration timeit(T&& func) {
    const auto start = std::chrono::system_clock::now();
    func();
    return std::chrono::system_clock::now() - start;
}

static bool is_prime(uint64_t n) {
    if(n % 2 == 0) {
        return true;
    }
    for(uint64_t d = 3; d * d <= n; d += 2) {
        if(n % d == 0) {
            return true;
        }
    }
    return false;
}

static constexpr size_t cpus_count = 12;
static constexpr size_t samples_count = 10;

#define MY_CHECKED_WRITE(fd, s) do { const auto MY_CONCAT(_my_checked_write_, __LINE__) = write(fd, s.data(), s.length()); MY_ASSERT_NOT_LESS_ZERO(MY_CONCAT(_my_checked_write_, __LINE__)); MY_ASSERT(static_cast<size_t>(MY_CONCAT(_my_checked_write_, __LINE__)) == s.length()); } while(0)

int main() {
    MY_ASSERT(sysconf(_SC_NPROCESSORS_ONLN) == static_cast<long>(cpus_count));

    const int dev_null_fd = open("/dev/null", O_WRONLY);
    MY_ASSERT_NOT_LESS_ZERO(dev_null_fd);
    defer(MY_ASSERT_NOT_LESS_ZERO(close(dev_null_fd)));

    const auto warm_up = [dev_null_fd]() {
        MY_LOG_DEBUG("start warm_up");
        const auto dur = timeit([&]() {
            constexpr uint64_t mmax = std::numeric_limits<uint32_t>::max() >> 8;
            constexpr uint64_t mmin = mmax >> 1;
            std::array<std::jthread, cpus_count> threads{};    
            for(auto& t : threads) {
                t = std::jthread([dev_null_fd]() {
                    MY_FOR_RANGE(uint64_t, i, mmin, mmax) {
                        const bool res = is_prime(i);
                        write(dev_null_fd, &res, sizeof(res));
                    }
                });
            }
        });
        const auto dur_casted = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(dur);
        MY_LOG_DEBUG("end warm_up, took %lf ms", dur_casted.count());
    };

    warm_up();

    MY_LOG_DEBUG("start single");
    {
        const auto fd = open("prime_slice_single.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "mmin,mmax,nanoseconds\n";
        MY_CHECKED_WRITE(fd, header);

        MY_FOR_RANGE(uint64_t, i, 7, 10) {
            const uint64_t mmax = std::numeric_limits<uint32_t>::max() >> i;
            const uint64_t mmin = mmax >> 1;

            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                const auto dur = timeit([dev_null_fd, mmin, mmax]() {
                    MY_FOR_RANGE(uint64_t, number, mmin, mmax) {
                        const bool res = is_prime(number);
                        write(dev_null_fd, &res, sizeof(res));
                    }
                });
                const auto line = std::to_string(mmin) + "," + std::to_string(mmax) + "," + std::to_string(dur.count()) + "\n";
                MY_CHECKED_WRITE(fd, line);
                printf("%s", line.data());
            }
        }
    }
    MY_LOG_DEBUG("end single");

    warm_up();

    MY_LOG_DEBUG("start multi");
    {
        const int fd = open("prime_slice_multi.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "threads_count,mmin,mmax,nanoseconds\n";
        MY_CHECKED_WRITE(fd, header);

        MY_FOR_RANGE(uint64_t, i, 7, 10) {
            const uint64_t mmax = std::numeric_limits<uint32_t>::max() >> i;
            const uint64_t mmin = mmax >> 1;
            MY_FOR_RANGE(size_t, threads_count, 2, cpus_count) {
                pthread_barrier_t barrier{};
                MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_init(&barrier, nullptr, static_cast<uint32_t>(threads_count + 1)));
                defer(MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_destroy(&barrier)));

                std::vector<std::jthread> threads(threads_count);

                const auto range_len = mmax - mmin;
                const auto step = range_len / threads_count;
                const auto remainder = range_len % threads_count;
                for(size_t thread_index = 0, offset = 0; thread_index < threads_count; ++thread_index) {
                    uint64_t local_mmin = 0;
                    uint64_t local_mmax = 0;
                    if(offset >= range_len) {
                        local_mmin = mmax;
                        local_mmax = mmax;
                    } else {
                        const auto local_step = thread_index < remainder ? step + 1 : step;
                        local_mmin = mmin + offset;
                        local_mmax = mmin + offset + local_step;
                        offset += local_step;
                    }
                    threads[thread_index] = std::jthread(
                        [&barrier, dev_null_fd, local_mmin, local_mmax]() {
                            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                                MY_PTHREAD_BARRIER_WAIT(&barrier);
                                MY_PTHREAD_BARRIER_WAIT(&barrier);

                                MY_FOR_RANGE(uint64_t, number, local_mmin, local_mmax) {
                                    const bool res = is_prime(number);
                                    write(dev_null_fd, &res, sizeof(res));
                                }

                                MY_PTHREAD_BARRIER_WAIT(&barrier);
                            }
                        }
                    );
                }

                MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                    MY_PTHREAD_BARRIER_WAIT(&barrier);
                    const auto dur = timeit(
                        [&barrier]() {
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                        }
                    );
                    const auto line = std::to_string(threads_count) + "," + std::to_string(mmin) + "," + std::to_string(mmax) + "," + std::to_string(dur.count()) + "\n";
                    MY_CHECKED_WRITE(fd, line);
                    printf("%s", line.data());
                }
            }
        }
    }
    MY_LOG_DEBUG("end multi");
    return 0;
}


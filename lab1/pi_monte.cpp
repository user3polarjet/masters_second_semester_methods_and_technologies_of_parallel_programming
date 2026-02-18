#include <algorithm>
#include <numeric>
#include <string>
#include <vector>
#include <array>
#include <chrono>
#include <thread>
#include <numbers>

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

#define defer(code) const auto MY_CONCAT(_defer_lambda_, __LINE__) = [&](){ code; }; defer_t<decltype(MY_CONCAT(_defer_lambda_, __LINE__))> MY_CONCAT(_defer_, __LINE__){MY_CONCAT(_defer_lambda_, __LINE__)} 

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

struct Xoroshiro128PP {
    uint64_t s[2];

    void seed(uint64_t x) {
        s[0] = splitmix64(x);
        s[1] = splitmix64(x);
    }

    static uint64_t splitmix64(uint64_t& state) {
        uint64_t z = (state += 0x9E3779B97F4A7C15);
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
        return z ^ (z >> 31);
    }

    static inline uint64_t rotl(uint64_t x, int k) {
        return (x << k) | (x >> (64 - k));
    }

    uint64_t next() {
        const uint64_t s0 = s[0];
        uint64_t s1 = s[1];
        const uint64_t result = rotl(s0 + s1, 17) + s0;
        s1 ^= s0;
        s[0] = rotl(s0, 49) ^ s1 ^ (s1 << 21);
        s[1] = rotl(s1, 28);
        return result;
    }

    double next_double() {
        return (next() >> 11) * (1.0 / 9007199254740992.0);
    }
};

static uint64_t count_points(const uint64_t points_count) {
    Xoroshiro128PP rng{};
    rng.seed(static_cast<uint64_t>(time(nullptr)));
    uint64_t points_inside_circle_count = 0;
    for(uint32_t i = 0; i < points_count; ++i) {
        const double x = rng.next_double() * 2.0 - 1.0;
        const double y = rng.next_double() * 2.0 - 1.0;
        if ((x * x + y * y) <= 1.0) {
            ++points_inside_circle_count;
        }
    }
    return points_inside_circle_count;
}

static constexpr uint64_t points_per_thread_min = 1'000'000;
static constexpr uint64_t points_per_thread_max = 10'000'000;
static constexpr uint64_t points_per_thread_step = 1'000'000;
static constexpr size_t cpus_count = 12;

int main() {
    MY_ASSERT(sysconf(_SC_NPROCESSORS_ONLN) == static_cast<long>(cpus_count));
    {
        std::vector<std::thread> threads(cpus_count);
        for(auto& thread : threads) {
            thread = std::thread([]() { count_points(1'000'000); });
        }
        for(auto& thread : threads) {
            thread.join();
        }
    }
    static constexpr size_t samples_count = 10;

    {
        std::vector<uint64_t> points_counts{};
        for(auto points_per_thread = points_per_thread_min; points_per_thread <= points_per_thread_max; points_per_thread += points_per_thread_step) {
            for(uint64_t threads_count = 1; threads_count <= static_cast<uint64_t>(cpus_count); ++threads_count) {
                points_counts.push_back(threads_count * points_per_thread);
            }
        }
        std::sort(std::begin(points_counts), std::end(points_counts));
        points_counts.erase(std::unique(std::begin(points_counts), std::end(points_counts)), std::end(points_counts));

        const auto fd = open("pi_monte_single.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "points_count,nanoseconds\n";
        MY_ASSERT_NOT_LESS_ZERO(write(fd, header.data(), header.length()));
        for(const auto points_count : points_counts) {
            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                uint64_t points_circle_count = 0;
                const auto dur = timeit(
                    [points_count, &points_circle_count]() {
                        points_circle_count = count_points(points_count);
                    });
                const double pi = 4.0 * static_cast<double>(points_circle_count) / static_cast<double>(points_count);
                MY_ASSERT(std::abs(pi - std::numbers::pi) < 0.01);

                const auto s = std::to_string(points_count) + "," + std::to_string(dur.count()) + "\n";
                MY_ASSERT_NOT_LESS_ZERO(write(fd, s.c_str(), s.length()));
                // MY_ASSERT_NOT_LESS_ZERO(fsync(fd));
            }
        }
    }
    {
        const int fd = open("pi_monte_multi.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "points_per_thread,threads_count,nanoseconds\n";
        MY_ASSERT_NOT_LESS_ZERO(write(fd, header.data(), header.length()));
        for(uint64_t points_per_thread = points_per_thread_min; points_per_thread <= points_per_thread_max; points_per_thread += points_per_thread_step) {
            for(uint32_t threads_count = 2; threads_count <= cpus_count; ++threads_count) {
                pthread_barrier_t barrier{};
                MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_init(&barrier, nullptr, threads_count + 1));
                defer(MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_destroy(&barrier)));

                std::vector<std::thread> threads(threads_count);
                std::vector<uint64_t> points_circle_counts(threads_count);

                MY_FOR_RANGE_ZERO(thread_index, threads_count) {
                    threads[thread_index] = std::thread(
                        [ &barrier, points_per_thread , &points_circle_counts, thread_index]() {
                            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                                MY_PTHREAD_BARRIER_WAIT(&barrier);
                                points_circle_counts[thread_index] = count_points(points_per_thread);
                                MY_PTHREAD_BARRIER_WAIT(&barrier);
                            }
                        }

                    );
                }
                defer(for(auto& t : threads) t.join());

                MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                    const auto dur = timeit(
                        [&barrier ,&points_circle_counts, points_per_thread, threads_count]() {
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            const double pi = 4.0 * static_cast<double>(std::accumulate(std::begin(points_circle_counts), std::end(points_circle_counts), 0)) / static_cast<double>(points_per_thread * threads_count);
                            MY_ASSERT(std::abs(pi - std::numbers::pi) < 0.01);
                        }
                    );
                    static_assert(std::is_same<std::remove_cv_t<decltype(dur)>, std::chrono::nanoseconds>::value, "");
                    const auto s = std::to_string(points_per_thread) + "," + std::to_string(threads_count) + "," + std::to_string(dur.count()) + "\n";
                    MY_ASSERT_NOT_LESS_ZERO(write(fd, s.c_str(), s.length()));
                    // MY_ASSERT_NOT_LESS_ZERO(fsync(fd));
                    // printf("%s", s.data());
                }
            }
        }
    }
}


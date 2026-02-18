#include <concepts>
#include <cstdint>
#include <string>
#include <vector>
#include <array>
#include <chrono>
#include <thread>
#include <barrier>

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

#define MY_CHECKED_WRITE(fd, s) do { const auto MY_CONCAT(_my_checked_write_, __LINE__) = write(fd, s.data(), s.length()); MY_ASSERT_NOT_LESS_ZERO(MY_CONCAT(_my_checked_write_, __LINE__)); MY_ASSERT(static_cast<size_t>(MY_CONCAT(_my_checked_write_, __LINE__)) == s.length()); } while(0)

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

static constexpr size_t cpus_count = 12;
static constexpr size_t samples_count = 10;

template<auto MMIN, auto MMAX, class F>
static constexpr void static_for(F&& f)
    requires std::same_as<decltype(MMIN), decltype(MMAX)> && std::integral<decltype(MMIN)>
{
    if constexpr (MMIN < MMAX) {
        std::forward<F>(f).template operator()<MMIN>();
        static_for<MMIN + 1, MMAX>(std::forward<F>(f));
    }
}

static void transpose(uint8_t matrix[], const size_t size) {
    MY_FOR_RANGE_ZERO(row, size - 1) {
        MY_FOR_RANGE(size_t, col, row + 1, size) {
            uint8_t temp;
            temp = matrix[row * size + col];
            matrix[row * size + col] = matrix[col * size + row];
            matrix[col * size + row] = temp;
        }
    }
}

static constexpr size_t BLOCK_SIZE = 32;

static void transpose_block_diag(uint8_t matrix[], const size_t row_start, const size_t size) {
    const size_t limit = std::min(row_start + BLOCK_SIZE, size);
    for(size_t r = row_start; r < limit; ++r) {
        for(size_t c = r + 1; c < limit; ++c) {
            std::swap(matrix[r * size + c], matrix[c * size + r]);
        }
    }
}

static void transpose_block_swap(uint8_t* matrix, size_t r_start, size_t c_start, size_t size) {
    const size_t r_limit = std::min(r_start + BLOCK_SIZE, size);
    const size_t c_limit = std::min(c_start + BLOCK_SIZE, size);
    for (size_t r = r_start; r < r_limit; ++r) {
        for (size_t c = c_start; c < c_limit; ++c) {
            std::swap(matrix[r * size + c], matrix[c * size + r]);
        }
    }
}

int main() {
    MY_ASSERT(sysconf(_SC_NPROCESSORS_ONLN) == static_cast<long>(cpus_count));

    const auto warm_up = []() {
        MY_LOG_DEBUG("start warm_up");
        const auto dur = timeit([&]() {
            constexpr uint64_t mmax = std::numeric_limits<uint32_t>::max() >> 8;
            constexpr uint64_t mmin = mmax >> 1;
            std::array<std::jthread, cpus_count> threads{};    
            for(auto& t : threads) {
                t = std::jthread([]() {
                    constexpr size_t matrix_size = 20000; 
                    std::vector<uint8_t> matrix(matrix_size * matrix_size);
                    transpose(matrix.data(), matrix_size);
                });
            }
        });
        const auto dur_casted = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(dur);
        MY_LOG_DEBUG("end warm_up, took %lf ms", dur_casted.count());
    };

    warm_up();
    static constexpr size_t matrix_size = 30000;

    {
        MY_LOG_DEBUG("start single");
        const auto fd = open("matrix_transpose_single.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "nanoseconds\n";
        MY_CHECKED_WRITE(fd, header);
        MY_FOR_RANGE_ZERO(sample_index, samples_count) {
            std::vector<uint8_t> matrix(matrix_size * matrix_size);
            const auto dur = timeit([&matrix]() { transpose(matrix.data(), matrix_size); });
            const auto line = std::to_string(dur.count()) + "\n";
            MY_CHECKED_WRITE(fd, line);
            printf("%s", line.data());
        }
        MY_LOG_DEBUG("end single");
    }
    warm_up();
    {
        MY_LOG_DEBUG("start multi");

        const int fd = open("matrix_transpose_multi.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "threads_count,nanoseconds\n";
        MY_CHECKED_WRITE(fd, header);

        MY_FOR_RANGE(size_t, threads_count, 2, cpus_count + 1) {
            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                pthread_barrier_t barrier{};
                MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_init(&barrier, nullptr, static_cast<uint32_t>(threads_count + 1)));
                defer(MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_destroy(&barrier)));

                std::vector<uint8_t> matrix(matrix_size * matrix_size);
                std::vector<std::jthread> threads(threads_count);
                MY_FOR_RANGE_ZERO(thread_index, threads_count) {
                    threads[thread_index] = std::jthread(
                        [&barrier, thread_index, threads_count, &matrix]() {
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            for(size_t row = thread_index; row < matrix_size - 1; row += threads_count) {
                                for(size_t col = row + 1; col < matrix_size; ++col) {
                                     uint8_t temp = matrix[row * matrix_size + col];
                                     matrix[row * matrix_size + col] = matrix[col * matrix_size + row];
                                     matrix[col * matrix_size + row] = temp;
                                }
                            }
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                        }
                    );
                }
                MY_PTHREAD_BARRIER_WAIT(&barrier);
                const auto dur = timeit(
                    [&barrier]() {
                        MY_PTHREAD_BARRIER_WAIT(&barrier);
                        MY_PTHREAD_BARRIER_WAIT(&barrier);
                    }
                );
                const auto line = std::to_string(threads_count) + "," + std::to_string(dur.count()) + "\n";
                MY_CHECKED_WRITE(fd, line);
                printf("%s", line.data());
            }
        }
        MY_LOG_DEBUG("end multi");
    }
    warm_up();
    {
        MY_LOG_DEBUG("start multi");

        const int fd = open("matrix_transpose_block.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "threads_count,nanoseconds\n";
        MY_CHECKED_WRITE(fd, header);

        MY_FOR_RANGE(size_t, threads_count, 2, cpus_count + 1) {
            MY_FOR_RANGE_ZERO(sample_index, samples_count) {
                pthread_barrier_t barrier{};
                MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_init(&barrier, nullptr, static_cast<uint32_t>(threads_count + 1)));
                defer(MY_ASSERT_NOT_LESS_ZERO(pthread_barrier_destroy(&barrier)));

                std::vector<uint8_t> matrix(matrix_size * matrix_size);
                std::vector<std::jthread> threads(threads_count);
                MY_FOR_RANGE_ZERO(thread_index, threads_count) {
                    threads[thread_index] = std::jthread(
                        [&barrier, thread_index, threads_count, &matrix]() {
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                            for(size_t i = thread_index * BLOCK_SIZE; i < matrix_size; i += threads_count * BLOCK_SIZE) {
                                for(size_t j = i; j < matrix_size; j += BLOCK_SIZE) {
                                    if (i == j) {
                                        transpose_block_diag(matrix.data(), i, matrix_size);
                                    } else {
                                        transpose_block_swap(matrix.data(), i, j, matrix_size);
                                    }
                                }
                            }
                            MY_PTHREAD_BARRIER_WAIT(&barrier);
                        }
                    );
                }
                MY_PTHREAD_BARRIER_WAIT(&barrier);
                const auto dur = timeit(
                    [&barrier]() {
                        MY_PTHREAD_BARRIER_WAIT(&barrier);
                        MY_PTHREAD_BARRIER_WAIT(&barrier);
                    }
                );
                const auto line = std::to_string(threads_count) + "," + std::to_string(dur.count()) + "\n";
                MY_CHECKED_WRITE(fd, line);
                printf("%s", line.data());
            }
        }
        MY_LOG_DEBUG("end multi");
    }
    return 0;
}


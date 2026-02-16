#include <cstdint>
#include <iterator>
#include <ranges>
#include <string>
#include <sys/stat.h>
#include <vector>
#include <random>
#include <limits>

#include <thread>
#include <barrier>

#include <chrono>

#include <fcntl.h>
#include <unistd.h>
#include <string.h>

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
        return (next() >> 11) * 0x1.0p-53;
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

int main() {
    constexpr uint64_t points_per_thread_min = 10000;
    constexpr uint64_t points_per_thread_max = 1000000;
    constexpr uint64_t points_per_thread_step = 10000;
    {
        std::vector<std::thread> threads(std::thread::hardware_concurrency());
        for(auto& t : threads) {
            t = std::thread([]() { count_points(1000000); });
        }
        for(auto& t : threads) {
            t.join();
        }
    }
    {
        std::vector<uint64_t> points_counts{};
        for(auto points_per_thread = points_per_thread_min; points_per_thread <= points_per_thread_max; points_per_thread += points_per_thread_step) {
            for(uint64_t threads_count = 1; threads_count <= static_cast<uint64_t>(std::thread::hardware_concurrency()); ++threads_count) {
                points_counts.push_back(threads_count * points_per_thread);
            }
        }
        std::sort(std::begin(points_counts), std::end(points_counts));
        points_counts.erase(std::unique(std::begin(points_counts), std::end(points_counts)), std::end(points_counts));

        const auto fd = open("pi_monte_single.csv", O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(fd)));
        MY_ASSERT_NOT_LESS_ZERO(fchmod(fd, 0666));
        constexpr std::string_view header = "points_count,nanoseconds,points_circle_count\n";
        MY_ASSERT_NOT_LESS_ZERO(write(fd, header.data(), header.length()));
        for(const auto points_count : points_counts) {
            uint64_t points_circle_count = 0;
            const auto dur = timeit([points_count, &points_circle_count]() {
                points_circle_count = count_points(points_count);
            });
            const auto s = std::to_string(points_count) + "," + std::to_string(dur.count()) + "," + std::to_string(points_circle_count) + "\n";
            MY_ASSERT_NOT_LESS_ZERO(write(fd, s.c_str(), s.length()));
            MY_ASSERT_NOT_LESS_ZERO(fsync(fd));
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
            for(uint64_t threads_count = 1; threads_count <= static_cast<uint64_t>(std::thread::hardware_concurrency()); ++threads_count) {
                std::vector<std::thread> threads(threads_count);
                for(auto& t : threads) {
                    t = std::thread([&sync_point]() {
                        sync_point.arrive_and_wait();
                    })
                }


                MY_FOR_RANGE_ZERO(sample_index, 10) {
                    const auto dur = timeit([&threads, points_per_thread]() {
                        for(auto& t : threads) {
                            t = std::thread([points_per_thread]() { count_points(points_per_thread); });
                        }
                        for(auto& t : threads) {
                            t.join();
                        }
                    });
                    MY_ASSERT_NOT_LESS_ZERO(fsync(fd));
                    static_assert(std::is_same_v<std::remove_cv_t<decltype(dur)>, std::chrono::nanoseconds>);
                    const auto s = std::to_string(points_per_thread) + "," + std::to_string(threads_count) + "," + std::to_string(dur.count()) + "\n";
                    MY_ASSERT_NOT_LESS_ZERO(write(fd, s.c_str(), s.length()));
                    MY_ASSERT_NOT_LESS_ZERO(fsync(fd));
                }
            }
        }
    }
}


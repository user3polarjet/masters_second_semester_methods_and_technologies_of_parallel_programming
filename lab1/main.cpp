#include <numeric>
#include <vector>
#include <array>
#include <random>
#include <cstdio>
#include <limits>
#include <ctime>
#include <thread>

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

#define MY_PRINT_EXPR_SIZE_T(expr) MY_PRINT_EXPR_IMPL(expr, "%lu")
#define MY_PRINT_EXPR_UINT(expr) MY_PRINT_EXPR_IMPL(expr, "%u")
#define MY_PRINT_EXPR_STRING(expr) MY_PRINT_EXPR_IMPL(expr, "%s")
#define MY_ASSERT_GLFW(expr) MY_ASSERT((expr) == GLFW_TRUE)

#define MY_ASSERT_VK(expr) MY_ASSERT((expr) == VK_SUCCESS)
#define MY_PRINT_EXPR_IMPL(expr, format)\
    do {\
        MY_LOG_DEBUG("`"#expr"`: `" format "`", (expr));\
    } while(0)

#define MY_PRINT_EXPR_SIZE_T(expr) MY_PRINT_EXPR_IMPL(expr, "%lu")
#define MY_PRINT_EXPR_UINT(expr) MY_PRINT_EXPR_IMPL(expr, "%u")
#define MY_PRINT_EXPR_STRING(expr) MY_PRINT_EXPR_IMPL(expr, "%s")

template <typename F>
struct defer_t {
    F f;
    ~defer_t() { f(); }
};

#define defer(code) const auto MY_CONCAT(_defer_lambda_, __LINE__) = [&](){ code; }; defer_t<decltype(MY_CONCAT(_defer_lambda_, __LINE__))> MY_CONCAT(_defer_, __LINE__){MY_CONCAT(_defer_lambda_, __LINE__)} 

#define MY_ARRAY_COUNT(arr) (sizeof(arr) / sizeof(arr[0]))

static void single() {
    static constexpr int64_t mid_point = std::numeric_limits<int>::max() / 2 + 1;
    static constexpr int64_t max_dist = mid_point * mid_point;

    std::random_device random_device{};
    std::mt19937 gen(random_device());
    std::uniform_real_distribution<double> distrib(-1.0, 1.0);

    const int points_count = 1000000;
    int points_inside_circle_count = 0;
    for(int i = 0; i < points_count; ++i) {
        const double x = distrib(gen);
        const double y = distrib(gen);
        if ((x * x + y * y) <= 1.0) {
            ++points_inside_circle_count;
        }
    }
    const double pi = 4.0 * static_cast<double>(points_inside_circle_count) / points_count;
    MY_LOG_DEBUG("PI: %lf", pi);
}

static void multi() {
    static constexpr int64_t mid_point = std::numeric_limits<int>::max() / 2 + 1;
    static constexpr int64_t max_dist = mid_point * mid_point;

    constexpr size_t threads_count = 6;
    constexpr size_t points_count_per_thread = 10000000;
    constexpr size_t points_count = points_count_per_thread * threads_count;

    std::array<std::thread, threads_count> threads{};
    std::array<int, threads.size()> points_inside_circle_counts{};

    MY_FOR_RANGE_ZERO(thread_index, threads_count) {
        threads[thread_index] = std::thread([thread_index, &points_inside_circle_counts]() {
            std::random_device random_device{};
            std::mt19937 gen(random_device());
            std::uniform_real_distribution<double> distrib(-1.0, 1.0);
            int points_inside_circle_count = 0;
            MY_FOR_RANGE_ZERO(point_index, points_count_per_thread) {
                const double x = distrib(gen);
                const double y = distrib(gen);
                if ((x * x + y * y) <= 1.0) {
                    ++points_inside_circle_count;
                }
            }
            points_inside_circle_counts[thread_index] = points_inside_circle_count;
        });
    }
    for(auto& t : threads) {
        t.join();
    }
    const int points_inside_circle_count = std::accumulate(std::begin(points_inside_circle_counts), std::end(points_inside_circle_counts), 0);
    const double pi = 4.0 * static_cast<double>(points_inside_circle_count) / static_cast<double>(points_count);
    MY_LOG_DEBUG("PI: %lf", pi);
}

int main() {
    multi();
}


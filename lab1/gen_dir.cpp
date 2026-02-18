#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <string.h>
#include <pthread.h>
#include <sys/mman.h>

#include <filesystem>

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

int main() {
    Xoroshiro128PP xoroshiro{};
    xoroshiro.seed(static_cast<uint64_t>(time(NULL)));

    const auto current_file = std::filesystem::path(__FILE__);     
    const auto project_dir = current_file.parent_path();     
    const auto build_dir = project_dir / "build";
    const auto files_dir = build_dir / "test_dir";

    std::filesystem::remove_all(files_dir);
    MY_ASSERT_NOT_LESS_ZERO(mkdir(files_dir.c_str(), 0700));

    MY_FOR_RANGE_ZERO(file_index, 10000) {
        const auto file_path = files_dir / std::to_string(file_index);
        const int file_fd = open(file_path.c_str(), O_RDWR | O_CREAT | O_TRUNC);
        MY_ASSERT_NOT_LESS_ZERO(file_fd);
        defer(MY_ASSERT_NOT_LESS_ZERO(close(file_fd)));
        MY_ASSERT_NOT_LESS_ZERO(chmod(file_path.c_str(), 0600));

        constexpr size_t words_count = 6000;
        constexpr size_t word_len = 50;
        constexpr size_t file_size = word_len * words_count;
        MY_ASSERT_NOT_LESS_ZERO(ftruncate(file_fd, file_size));
        struct stat st;
        MY_ASSERT_NOT_LESS_ZERO(fstat(file_fd, &st));
        MY_ASSERT(static_cast<size_t>(st.st_size) == file_size);

        char* const ptr = reinterpret_cast<char*>(mmap(nullptr, static_cast<size_t>(st.st_size), PROT_WRITE | PROT_READ, MAP_SHARED, file_fd, 0));
        MY_ASSERT(ptr != MAP_FAILED);
        defer(MY_ASSERT_NOT_LESS_ZERO(munmap(ptr, static_cast<size_t>(st.st_size))));

        size_t index = 0;
        MY_FOR_RANGE_ZERO(word_index, words_count) {
            MY_FOR_RANGE_ZERO(letter_index, word_len - 1) {
                constexpr char rng_mod = 'z' - 'a';
                ptr[index++] = 'A' + (xoroshiro.next() % rng_mod);
            }
            ptr[index++] = '\n';
        }
    }
    return 0;
}


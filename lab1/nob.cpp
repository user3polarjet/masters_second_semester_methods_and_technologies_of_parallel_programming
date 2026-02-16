#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <filesystem>

#include <unistd.h>
#include <sys/stat.h>
#include <sys/time.h>

// #include <cassert>
// #define ASSERT_NOT_MINUS_ONE(expr) assert((expr) != -1)

#define NOB_REBUILD_URSELF(binary_path, source_path) "clang", "-x", "c++", "-lstdc++", "-o", binary_path, source_path
#define NOB_IMPLEMENTATION
#include "nob.h"


// static bool operator>(const timespec& lhs, const timespec& rhs) {
//     return lhs.tv_sec == rhs.tv_sec ? lhs.tv_nsec > rhs.tv_nsec : lhs.tv_sec > rhs.tv_sec;
// }
//
// template<typename ...Args>
// static bool needs_rebuild(const char input_path[], Args... args) {
//     static_assert((std::is_same<Args, const char*>::value && ...), "");
//     struct stat input_st;
//     int res = stat(input_path, &input_st);
//     if(res == ENOENT) {
//         return true;
//     }
//     struct stat st;
//     return ((ASSERT_NOT_MINUS_ONE(stat(args, &st)), st.st_mtim > input_st.st_mtim) && ...);
// }
//
// static void rebuild_yourself() {
//     const auto file = std::filesystem::path(__FILE__);     
//     const auto file_old = file.filename().string() + ".old";
//     if(needs_rebuild())
//     struct stat st_old;
//     ASSERT_NOT_MINUS_ONE(stat(file_old.c_str(), &st));
//
//     std::filesystem::status(file)
//     if()
//
//     // execvp()
// } 

int main(int argc, char **argv) {
    NOB_GO_REBUILD_URSELF(argc, argv);
    const auto current_file = std::filesystem::path(__FILE__);     
    const auto build_dir = current_file.parent_path() / "build";
    if(not std::filesystem::exists(build_dir)) {
        std::filesystem::create_directory(build_dir);
    }
    nob_mkdir_if_not_exists(build_dir.c_str());
    const auto main_exec = build_dir / "main";
    Nob_Cmd cmd = {0};
    nob_cmd_append(
        &cmd,
        "clang",
        "-std=c++20",
        "-Weverything",
        "-Wall",
        "-Wextra",
        "-Werror",

        "-O3",

        "-lstdc++",
        "-lm",
        "-fno-rtti",
        "-fno-exceptions",
        "-fsanitize=address",

        "-Wno-c++98-compat",
        "-Wno-c++98-compat-pedantic",
        "-Wno-unused-macros",
        "-Wno-unused",
        "-Wno-unsafe-buffer-usage-in-libc-call",
        "-Wno-disabled-macro-expansion",
        "-Wno-padded",
        "-Wno-unreachable-code-loop-increment",

        "-o", 
        main_exec.c_str(),
        "main.cpp"
    );
    if (!nob_cmd_run(&cmd)) return 1;
    return 0;
}

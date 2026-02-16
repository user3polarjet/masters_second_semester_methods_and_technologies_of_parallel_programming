#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <filesystem>

#include <unistd.h>
#include <sys/stat.h>
#include <sys/time.h>

#define NOB_REBUILD_URSELF(binary_path, source_path) "clang", "-x", "c++", "-lstdc++", "-o", binary_path, source_path
#define NOB_IMPLEMENTATION
#include "nob.h"

int main(int argc, char **argv) {
    NOB_GO_REBUILD_URSELF(argc, argv);
    const auto current_file = std::filesystem::path(__FILE__);     
    const auto build_dir = current_file.parent_path() / "build";
    if(not std::filesystem::exists(build_dir)) {
        std::filesystem::create_directory(build_dir);
    }
    nob_mkdir_if_not_exists(build_dir.c_str());

    Nob_Cmd cmd = {0};

    const auto std_includes = current_file.parent_path() / "std_includes.hpp";
    const auto std_includes_pch = build_dir / (std_includes.filename().string() + ".pch");
    const char* std_includes_cstr = std_includes.c_str();
    if(nob_needs_rebuild(std_includes_pch.c_str(), &std_includes_cstr, 1)) {
        nob_cmd_append(&cmd, "clang", "-std=c++20", "-fno-rtti", "-fno-exceptions", "-O3", "-fsanitize=address", "-x", "c++-header", std_includes.c_str(), "-o", std_includes_pch.c_str());
        if (!nob_cmd_run(&cmd)) return 1;
    }

    const auto main_exec = build_dir / "main";

    if(nob_needs_rebuild1(main_exec.c_str(), "main.cpp")) {
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
            "-include-pch",
            std_includes_pch.c_str(),

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
    }
    return 0;
}

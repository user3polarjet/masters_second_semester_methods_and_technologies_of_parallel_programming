#include <filesystem>

#define NOB_REBUILD_URSELF(binary_path, source_path) "clang", "-x", "c++", "-lstdc++", "-o", binary_path, source_path
#define NOB_IMPLEMENTATION
#include "nob.h"

int main(int argc, char **argv) {
    NOB_GO_REBUILD_URSELF(argc, argv);
    const auto current_file = std::filesystem::path(__FILE__);     
    const auto build_dir = current_file.parent_path() / "build";
    nob_mkdir_if_not_exists(build_dir.c_str());
    const auto main_exec = build_dir / "main";
    Nob_Cmd cmd = {0};
    nob_cmd_append(
        &cmd,
        "clang",
        "-std=c++11",
        "-Weverything",
        "-Wall",
        "-Wextra",
        "-Werror",

        "-lstdc++",
        "-lm",
        "-fno-rtti",
        "-fno-exceptions",

        "-Wno-c++98-compat",
        "-Wno-c++98-compat-pedantic",
        "-Wno-unused-macros",
        "-Wno-unused",
        "-o", 
        main_exec.c_str(),
        "main.cpp"
    );
    if (!nob_cmd_run(&cmd)) return 1;
    return 0;
}

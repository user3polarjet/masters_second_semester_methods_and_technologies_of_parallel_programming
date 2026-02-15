#include <filesystem>

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
    const auto main_exec = build_dir / "main";
    Nob_Cmd cmd = {0};
    nob_cmd_append(&cmd, "clang", "-Weverything", "-Wall", "-Wextra", "-lstdc++", "-fno-rtti", "-fno-exceptions", "-Wno-c++98-compat", "-o", main_exec.c_str(), "main.cpp");
    if (!nob_cmd_run(&cmd)) return 1;
    return 0;
}

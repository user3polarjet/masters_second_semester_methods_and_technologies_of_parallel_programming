#include <filesystem>

#define NOB_REBUILD_URSELF(binary_path, source_path) "clang", "-O0", "-x", "c++", "-lstdc++", "-o", binary_path, source_path
#define NOB_IMPLEMENTATION
#include "nob.h"

int main(int argc, char **argv) {
    NOB_GO_REBUILD_URSELF(argc, argv);
    const auto current_file = std::filesystem::path(__FILE__);     
    const auto project_dir = current_file.parent_path();     
    const auto build_dir = project_dir / "build";

    nob_mkdir_if_not_exists(build_dir.c_str());

    Nob_Cmd cmd = {0};

    const auto exec_sources = {
        "pi_monte",
        "prime_slice"
    };

    for(const auto exec_source : exec_sources) {
        const auto exec_source_path = project_dir / (std::string(exec_source) + ".cpp");
        const auto exec_path = build_dir / exec_source;
        if(nob_needs_rebuild1(exec_path.c_str(), exec_source_path.c_str())) {
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
                "-Wno-unused-template",

                "-o", 
                exec_path.c_str(),
                exec_source_path.c_str()
            );
            if (!nob_cmd_run(&cmd)) return 1;
        }
    }
    return 0;
}

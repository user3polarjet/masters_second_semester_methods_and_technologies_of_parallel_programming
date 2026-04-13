use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cimgui"); 
    println!("cargo:rerun-if-changed=cimplot");
    println!("cargo:rustc-link-lib=vulkan");

    let target = env::var("TARGET").unwrap();
    let is_cross_compiling = target == "aarch64-unknown-linux-gnu";
    
    let sysroot_path = "/home/user/work/mount";

    let mut clang_args = vec![
        "-E".to_string(), 
        "-P".to_string(), 
        "bindings.h".to_string(),
        "-Icimgui".to_string(),
        "-Icimgui/imgui".to_string(),
        "-Icimgui/imgui/backends".to_string(),
        "-I../glfw/include".to_string(),
        "-Icimplot/".to_string(),
        "-Icimplot/implot/".to_string(),

    ];

    if is_cross_compiling {
        clang_args.push(format!("--target={}", target));
        clang_args.push(format!("--sysroot={}", sysroot_path));
    }

    let preproc_status = Command::new("clang")
        .args(&clang_args)
        .output()
        .expect("Failed to execute clang preprocessor.");

    if !preproc_status.status.success() {
        let stderr = String::from_utf8_lossy(&preproc_status.stderr);
        panic!("Clang preprocessor failed with error:\n{}", stderr);
    }

    let preprocessed_code = String::from_utf8_lossy(&preproc_status.stdout);

    let mut flag_bits = Vec::new();
    let mut flags = Vec::new();

    for line in preprocessed_code.lines() {
        let line = line.trim();
        if line.starts_with("} ") && (line.ends_with("FlagBits;") || line.ends_with("FlagBitsEXT;")) {
            let name = &line[2..line.len() - 1]; 
            flag_bits.push(name.to_string());
        }
        if line.starts_with("typedef VkFlags ") {
            let name = &line["typedef VkFlags ".len()..line.len() - 1];
            flags.push(name.to_string());
        }
    }

    let mut builder = bindgen::Builder::default()
        .header("bindings.h")
        .clang_arg("-Icimgui")
        .clang_arg("-Icimgui/imgui")
        .clang_arg("-Icimgui/imgui/backends")
        .clang_arg("-I../glfw/include")
        .clang_arg("-Icimplot")
        .clang_arg("-Icimplot/implot")
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        .bitfield_enum(".*((Bits)|(Flags)).*")
        .newtype_enum(".*")

        .blocklist_item("VK_WHOLE_SIZE")
        .blocklist_item("VK_QUEUE_FAMILY_IGNORED")
        .blocklist_item("VK_QUEUE_FAMILY_EXTERNAL")
        .blocklist_item("VK_SUBPASS_EXTERNAL")

        .raw_line("pub const VK_WHOLE_SIZE: u64 = !0;")
        .raw_line("pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;")
        .raw_line("pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !0 - 1;")
        .raw_line("pub const VK_SUBPASS_EXTERNAL: u32 = !0;");

    if is_cross_compiling {
        builder = builder.clang_arg(format!("--sysroot={}", sysroot_path));
    }

    for flag in flags {
        if let Some(flag_bit) = flag_bits.iter().find(|&bit| bit.replace("Bits", "s") == flag) {
            builder = builder
                .blocklist_type(&flag)
                .raw_line(format!("pub type {} = {};", flag, flag_bit));
        }
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file_path = out_path.join("bindings.rs");

    let mut code = bindings.to_string();

    let re = regex::Regex::new(r"pub struct (ImGui[A-Za-z0-9_]+_)\(pub ::core::ffi::c_uint\);")
        .expect("Failed to compile regex");
    
    code = re.replace_all(&code, "pub struct ${1}(pub ::core::ffi::c_int);").to_string();

    let re = regex::Regex::new(r"pub struct (ImPlot[A-Za-z0-9_]+_)\(pub ::core::ffi::c_uint\);")
        .expect("Failed to compile regex");
    
    code = re.replace_all(&code, "pub struct ${1}(pub ::core::ffi::c_int);").to_string();

    std::fs::write(&bindings_file_path, code)
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();

    build.cpp(true)
        .define("CIMGUI_USE_VULKAN", None)
        .define("CIMGUI_USE_GLFW", None)
        .define("IMGUI_IMPL_API", "extern \"C\"")
        // .cpp_link_stdlib(None)
        .flag_if_supported("-fno-rtti") 
        .flag_if_supported("-fno-exceptions") 
        .flag_if_supported("-fno-threadsafe-statics")
        .opt_level(3)
        .file("cimgui/imgui/imgui.cpp")
        .file("cimgui/imgui/imgui_draw.cpp")
        .file("cimgui/imgui/imgui_tables.cpp")
        .file("cimgui/imgui/imgui_widgets.cpp")
        .file("cimgui/imgui/imgui_demo.cpp")
        .file("cimgui/imgui/backends/imgui_impl_glfw.cpp")
        .file("cimgui/imgui/backends/imgui_impl_vulkan.cpp")
        .file("cimgui/cimgui.cpp")
        .file("cimgui/cimgui_impl.cpp")
        .include("cimgui/imgui")
        .include("cimgui/imgui/backends")
        .include("cimgui")
        .file("cimplot/implot/implot_demo.cpp")
        .file("cimplot/implot/implot_items.cpp")
        .file("cimplot/implot/implot.cpp")
        .file("cimplot/cimplot.cpp")
        .include("cimplot/implot")
        .include("cimplot")
        .include("../glfw/include");

    if is_cross_compiling {
        build.flag(&format!("--sysroot={}", sysroot_path));
        build.include(format!("{}/usr/include", sysroot_path));
    }

    build.compile("dcimgui");
}

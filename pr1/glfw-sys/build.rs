fn main() {
    println!("cargo:rerun-if-changed=../glfw");

    let dst = cmake::Config::new("../glfw")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .build();

    let lib_path = dst.join("lib"); 
    let search_path = if dst.join("lib64").exists() {
        dst.join("lib64")
    } else {
        lib_path
    };
    println!("cargo:rustc-link-search=native={}", search_path.display());
    println!("cargo:rustc-link-lib=static=glfw3");
}

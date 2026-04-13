use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=linux.h");

    let bindings = bindgen::Builder::default()
        .header("linux.h")
        // FIX: This solves the "extern blocks must be unsafe" error
        .wrap_unsafe_ops(true)
        // .wrap_static_fns(true) 
        // Force bindgen to output 'unsafe extern "C"'
        // .generate_extern_crate(true) 
        
        
        // Use this to ensure compatibility with modern Rust editions
        .formatter(bindgen::Formatter::Rustfmt)
        
        // These match your original shell command
        .derive_default(true)
        .layout_tests(false)
        .newtype_enum(".*")
        
        // Help Clang find the right definitions for V4L2
        // .clang_arg("-D__EXPORTED_HEADERS__")
        
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use spirv_builder::SpirvBuilder;

fn main() {
    println!("cargo:rerun-if-changed=../my_shaders/src");
    println!("cargo:rerun-if-changed=../my_shaders/Cargo.toml");

    let result = SpirvBuilder::new("../my_shaders", "spirv-unknown-vulkan1.3")
        .capability(spirv_builder::Capability::SampledBuffer)
        .capability(spirv_builder::Capability::ImageBuffer)
        .capability(spirv_builder::Capability::VulkanMemoryModelDeviceScope)
        .build()
        .expect("Failed to build shader");

    let shader_path = result.module.unwrap_single();
    println!("cargo:rustc-env=SHADERS_PATH={}", shader_path.display());
}

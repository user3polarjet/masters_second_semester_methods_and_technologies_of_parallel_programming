#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate glfw_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    variant << 29u32 | major << 22u32 | minor << 12u32 | patch
}
pub const fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    major << 22u32 | minor << 12u32 | patch
}
pub const fn vk_api_version_variant(version: u32) -> u32 {
    version >> 29
}
pub const fn vk_api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7F
}
pub const fn vk_api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}
pub const fn vk_api_version_patch(version: u32) -> u32 {
    version & 0xFFF
}
pub const fn vk_version_major(version: u32) -> u32 {
    version >> 22
}
pub const fn vk_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}
pub const fn vk_version_patch(version: u32) -> u32 {
    version & 0xFFF
}

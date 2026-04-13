#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(dead_code)]

use my::kcf;
use nix;
use vk;

use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd};

struct Defer<T: FnMut()> {
    func: T,
}

impl<T: FnMut()> Defer<T> {
    fn new(func: T) -> Self {
        Self { func }
    }
}

impl<T: FnMut()> Drop for Defer<T> {
    fn drop(&mut self) {
        (self.func)();
    }
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const std::ffi::c_char
    };
}

macro_rules! defer {
    ($($item:tt)*) => {
        let _defer = Defer {
            func: || { $($item)* }
        };
    }
}

macro_rules! contains_bits {
    ($target:expr, $required:expr) => {
        ($target & $required) == $required
    };
}

macro_rules! vk_enumerate {
    ($func:expr) => {
        vk_enumerate!($func,)
    };
    ($func:expr, $( $arg:expr ),*) => {
        {
            let mut count = 0;
            unsafe {
                $func($($arg,)* &mut count, std::ptr::null_mut());

                let mut data = Vec::with_capacity(count as usize);
                $func($($arg,)* &mut count, data.as_mut_ptr());

                data.set_len(count as usize);
                data
            }
        }
    };
}

macro_rules! vk_create {
    ($func:expr, $( $arg:expr ),*) => {
        {
            let mut value = Default::default();
            unsafe { vk_assert!($func($($arg,)* &mut value)); }
            value
        }
    };
}

macro_rules! vk_assert {
    ($e:expr) => {
        assert_eq!($e, vk::VkResult::VK_SUCCESS)
    };
}

macro_rules! assert_not_less_zero {
    ($e:expr) => {
        assert!($e >= 0)
    };
}

unsafe extern "C" fn my_glfw_error_callback(error: i32, description: *const std::ffi::c_char) {
    panic!("GLFW Error {}: {}", error, unsafe {
        std::ffi::CStr::from_ptr(description).to_str().unwrap()
    });
}

struct GlfwContext<'a>(&'a mut vk::GLFWwindow);
impl<'a> GlfwContext<'a> {
    fn new() -> Self {
        unsafe { vk::glfwSetErrorCallback(Some(my_glfw_error_callback)) };
        assert_eq!(unsafe { vk::glfwInit() }, vk::GLFW_TRUE as i32);
        unsafe {
            vk::glfwWindowHint(vk::GLFW_CLIENT_API as i32, vk::GLFW_NO_API as i32);
        }
        unsafe {
            vk::glfwWindowHint(vk::GLFW_RESIZABLE as i32, vk::GLFW_FALSE as i32);
        }
        let glfw_window = unsafe { vk::glfwCreateWindow(640, 480, c_str!("Demo"), std::ptr::null_mut(), std::ptr::null_mut()) };
        Self(unsafe { std::ptr::NonNull::new(glfw_window).unwrap().as_mut() })
    }
}

impl<'a> Drop for GlfwContext<'a> {
    fn drop(&mut self) {
        unsafe {
            vk::glfwDestroyWindow(self.0);
        }
        unsafe {
            vk::glfwTerminate();
        }
    }
}

fn create_instance() -> vk::VkInstance {
    let vk_application_info = vk::VkApplicationInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pApplicationName: c_str!("Demo"),
        applicationVersion: vk::vk_make_version(1u32, 0u32, 0u32),
        pEngineName: c_str!("Demo Engine"),
        engineVersion: vk::vk_make_version(1u32, 0u32, 0u32),
        apiVersion: vk::vk_make_api_version(0, 1, 3, 0),
        ..Default::default()
    };

    let mut enabled_extension_names = vec![
        c_str!("VK_EXT_debug_utils"),
        c_str!("VK_KHR_get_physical_device_properties2"),
        c_str!("VK_KHR_external_memory_capabilities"),
    ];

    let mut glfw_required_instance_extensions_count: u32 = 0;
    let glfw_required_instance_extensions = unsafe { vk::glfwGetRequiredInstanceExtensions(&mut glfw_required_instance_extensions_count) };
    let glfw_required_instance_extensions =
        unsafe { std::slice::from_raw_parts(glfw_required_instance_extensions, glfw_required_instance_extensions_count as usize) };
    enabled_extension_names.extend(glfw_required_instance_extensions.iter());

    let enabled_layer_names = [c_str!("VK_LAYER_KHRONOS_validation")];
    {
        let vk_layer_properties_arr = vk_enumerate!(vk::vkEnumerateInstanceLayerProperties);
        for layer_name in enabled_layer_names {
            let c_layer_name = unsafe { std::ffi::CStr::from_ptr(layer_name) };
            vk_layer_properties_arr
                .iter()
                .find(|el| unsafe { std::ffi::CStr::from_ptr(el.layerName.as_ptr()) == c_layer_name })
                .unwrap();
        }
    }

    for ext in &enabled_extension_names {
        println!("ext: {:?}", unsafe { std::ffi::CStr::from_ptr(*ext) })
    }

    for ext in &enabled_layer_names {
        println!("layer: {:?}", unsafe { std::ffi::CStr::from_ptr(*ext) })
    }

    let vk_instance_create_info = vk::VkInstanceCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pApplicationInfo: &vk_application_info,
        ppEnabledExtensionNames: enabled_extension_names.as_ptr() as *const *const _,
        enabledExtensionCount: enabled_extension_names.len() as u32,
        ppEnabledLayerNames: enabled_layer_names.as_ptr() as *const *const _,
        enabledLayerCount: enabled_layer_names.len() as u32,
        ..Default::default()
    };
    let mut vk_instance: vk::VkInstance = std::ptr::null_mut();
    unsafe {
        vk_assert!(vk::vkCreateInstance(&vk_instance_create_info, std::ptr::null(), &mut vk_instance));
    }
    return vk_instance;
}

fn pick_physical_device(vk_instance: vk::VkInstance) -> vk::VkPhysicalDevice {
    let physical_devices = vk_enumerate!(vk::vkEnumeratePhysicalDevices, vk_instance);
    for (i, physical_device) in physical_devices.iter().enumerate() {
        let mut vk_physical_device_properties: vk::VkPhysicalDeviceProperties = Default::default();
        unsafe {
            vk::vkGetPhysicalDeviceProperties(physical_devices[i], &mut vk_physical_device_properties);
        }
        println!(
            "vk_physical_devices: {}, apiVersion: {}, driverVersion: {}, vendorID: {}, deviceID: {}, deviceType: {:?}, deviceName: {:?}",
            i,
            vk_physical_device_properties.apiVersion,
            vk_physical_device_properties.driverVersion,
            vk_physical_device_properties.vendorID,
            vk_physical_device_properties.deviceID,
            vk_physical_device_properties.deviceType,
            unsafe { std::ffi::CStr::from_ptr(vk_physical_device_properties.deviceName.as_ptr()) }
        );
    }
    for physical_device in physical_devices {
        let mut vk_physical_device_properties: vk::VkPhysicalDeviceProperties = Default::default();
        unsafe {
            vk::vkGetPhysicalDeviceProperties(physical_device, &mut vk_physical_device_properties);
        }
        if vk_physical_device_properties.deviceType == vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU {
            return physical_device;
        }
    }
    assert!(false);
    std::ptr::null_mut()
}

unsafe extern "C" fn debug_callback(
    messageSeverity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: vk::VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const vk::VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut ::core::ffi::c_void,
) -> vk::VkBool32 {
    println!("validation layer message: {:?}", unsafe {
        std::ffi::CStr::from_ptr((*pCallbackData).pMessage)
    });
    if (messageSeverity
        & (vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
            | vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT))
        .0
        != 0
    {
        assert!(false);
    } else {
    }
    return vk::VK_FALSE;
}

unsafe fn create_debug_utils_messenger_ext(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const vk::VkAllocationCallbacks,
    pMessenger: *mut vk::VkDebugUtilsMessengerEXT,
) -> vk::VkResult {
    let func: vk::PFN_vkCreateDebugUtilsMessengerEXT =
        unsafe { std::mem::transmute(vk::vkGetInstanceProcAddr(instance, c_str!("vkCreateDebugUtilsMessengerEXT"))) };
    let func = func.unwrap();
    unsafe { func(instance, pCreateInfo, pAllocator, pMessenger) }
}

unsafe fn destroy_debug_utils_messenger_ext(
    instance: vk::VkInstance,
    messenger: vk::VkDebugUtilsMessengerEXT,
    pAllocator: *const vk::VkAllocationCallbacks,
) {
    let func: vk::PFN_vkDestroyDebugUtilsMessengerEXT =
        unsafe { std::mem::transmute(vk::vkGetInstanceProcAddr(instance, c_str!("vkDestroyDebugUtilsMessengerEXT"))) };
    let func = func.unwrap();
    unsafe { func(instance, messenger, pAllocator) }
}

fn create_debug_utils_messenger(vk_instance: vk::VkInstance) -> vk::VkDebugUtilsMessengerEXT {
    let vk_debug_utils_messenger_create_info_ext = vk::VkDebugUtilsMessengerCreateInfoEXT {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        messageSeverity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT
            | vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
            | vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
        messageType: vk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
            | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
            | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        pfnUserCallback: Some(debug_callback),
        ..Default::default()
    };
    let mut vk_debug_utils_messenger_ext: vk::VkDebugUtilsMessengerEXT = Default::default();
    unsafe {
        vk_assert!(create_debug_utils_messenger_ext(
            vk_instance,
            &vk_debug_utils_messenger_create_info_ext,
            std::ptr::null(),
            &mut vk_debug_utils_messenger_ext
        ))
    };
    return vk_debug_utils_messenger_ext;
}

fn pick_graphics_presentation_compute_queue_family(vk_physical_device: vk::VkPhysicalDevice, vk_surface_khr: vk::VkSurfaceKHR) -> u32 {
    let vk_queue_family_properties_arr = vk_enumerate!(vk::vkGetPhysicalDeviceQueueFamilyProperties, vk_physical_device);
    vk_queue_family_properties_arr
        .iter()
        .enumerate()
        .find(|(index, prop)| {
            let mut presentation_family_supported = vk::VK_FALSE;
            unsafe {
                vk_assert!(vk::vkGetPhysicalDeviceSurfaceSupportKHR(
                    vk_physical_device,
                    *index as u32,
                    vk_surface_khr,
                    &mut presentation_family_supported
                ));
            }
            presentation_family_supported == vk::VK_TRUE
                && contains_bits!(prop.queueFlags, vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT)
                && contains_bits!(prop.queueFlags, vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT)
        })
        .unwrap()
        .0 as u32
}

fn create_device(vk_physical_device: vk::VkPhysicalDevice, queue_family_index: u32) -> vk::VkDevice {
    let vk_extension_properties_arr = vk_enumerate!(vk::vkEnumerateDeviceExtensionProperties, vk_physical_device, std::ptr::null());
    let vk_required_extension_names: [*const std::ffi::c_char; 6] = [
        c_str!("VK_KHR_swapchain"),
        c_str!("VK_KHR_external_memory"),
        c_str!("VK_KHR_external_memory_fd"),
        c_str!("VK_EXT_external_memory_dma_buf"),
        c_str!("VK_KHR_external_fence"),
        c_str!("VK_KHR_external_fence_fd"),
    ];
    for extension_name in &vk_required_extension_names {
        let requested_name = unsafe { std::ffi::CStr::from_ptr(*extension_name) };
        let _ = vk_extension_properties_arr
            .iter()
            .find(|el| {
                let el_name = unsafe { std::ffi::CStr::from_ptr(el.extensionName.as_ptr()) };
                el_name == requested_name
            })
            .unwrap();
    }

    let mut vk_13_features = vk::VkPhysicalDeviceVulkan13Features {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
        synchronization2: 1,
        ..Default::default()
    };
    let vk_12_features = vk::VkPhysicalDeviceVulkan12Features {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
        vulkanMemoryModel: 1,
        pNext: &mut vk_13_features as *mut _ as *mut std::ffi::c_void,
        ..Default::default()
    };

    let mut vk_physical_device_features: vk::VkPhysicalDeviceFeatures = Default::default();
    vk_physical_device_features.wideLines = 1;

    let queue_priority: f32 = 1.0;
    let vk_device_queue_create_info = vk::VkDeviceQueueCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        queueFamilyIndex: queue_family_index,
        queueCount: 1,
        pQueuePriorities: &queue_priority,
        ..Default::default()
    };

    // 2. Chain the 1.2 features into the creation info using pNext
    let vk_device_create_info = vk::VkDeviceCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: &vk_12_features as *const _ as *const std::ffi::c_void, // Chain the struct here
        pQueueCreateInfos: &vk_device_queue_create_info,
        queueCreateInfoCount: 1,
        ppEnabledExtensionNames: vk_required_extension_names.as_ptr(),
        enabledExtensionCount: vk_required_extension_names.len() as u32,
        pEnabledFeatures: &vk_physical_device_features,
        ..Default::default()
    };

    let mut vk_device: vk::VkDevice = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateDevice(
            vk_physical_device,
            &vk_device_create_info,
            std::ptr::null(),
            &mut vk_device
        ));
    }
    return vk_device;
}

fn create_command_pool(vk_device: vk::VkDevice, queue_family_index: u32) -> vk::VkCommandPool {
    let vk_command_pool_create_info = vk::VkCommandPoolCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        flags: vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
            | vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT,
        queueFamilyIndex: queue_family_index,
        ..Default::default()
    };
    let mut vk_command_pool: vk::VkCommandPool = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateCommandPool(
            vk_device,
            &vk_command_pool_create_info,
            std::ptr::null(),
            &mut vk_command_pool
        ))
    };
    return vk_command_pool;
}

const IMAGE_PIXEL_WIDTH: u32 = 640;
const IMAGE_PIXEL_HEIGHT: u32 = 480;
const IMAGE_BUFFER_SIZE: u64 = (IMAGE_PIXEL_WIDTH * IMAGE_PIXEL_HEIGHT * 2) as u64;

fn align_up(m: vk::VkDeviceSize, k: vk::VkDeviceSize) -> vk::VkDeviceSize {
    // Standard alignment check: k must be a power of two
    assert!(k > 0 && (k & (k - 1)) == 0);
    (m + k - 1) & !(k - 1)
}

trait MemoryRequirements {
    fn get_memory_requirements(&self, vk_device: vk::VkDevice) -> vk::VkMemoryRequirements;
    fn bind_memory(&self, vk_device: vk::VkDevice, memory: vk::VkDeviceMemory, offset: vk::VkDeviceSize);
}

impl MemoryRequirements for vk::VkBuffer {
    fn get_memory_requirements(&self, vk_device: vk::VkDevice) -> vk::VkMemoryRequirements {
        let mut vk_memory_requirements = vk::VkMemoryRequirements::default();
        unsafe { vk::vkGetBufferMemoryRequirements(vk_device, *self, &mut vk_memory_requirements) };
        vk_memory_requirements
    }
    fn bind_memory(&self, vk_device: vk::VkDevice, memory: vk::VkDeviceMemory, offset: vk::VkDeviceSize) {
        vk_assert!(unsafe { vk::vkBindBufferMemory(vk_device, *self, memory, offset) });
    }
}

impl MemoryRequirements for vk::VkImage {
    fn get_memory_requirements(&self, vk_device: vk::VkDevice) -> vk::VkMemoryRequirements {
        let mut vk_memory_requirements = vk::VkMemoryRequirements::default();
        unsafe { vk::vkGetImageMemoryRequirements(vk_device, *self, &mut vk_memory_requirements) };
        vk_memory_requirements
    }
    fn bind_memory(&self, vk_device: vk::VkDevice, memory: vk::VkDeviceMemory, offset: vk::VkDeviceSize) {
        vk_assert!(unsafe { vk::vkBindImageMemory(vk_device, *self, memory, offset) });
    }
}

fn find_memory_type_index(
    vk_physical_device: vk::VkPhysicalDevice,
    memory_type_bits: u32,
    vk_memory_property_flag_bits: vk::VkMemoryPropertyFlagBits,
) -> u32 {
    let mut props = vk::VkPhysicalDeviceMemoryProperties::default();
    unsafe {
        vk::vkGetPhysicalDeviceMemoryProperties(vk_physical_device, &mut props);
    }
    let memory_types = &props.memoryTypes[..(props.memoryTypeCount as usize)];
    for (i, mem_type) in memory_types.iter().enumerate() {
        if (memory_type_bits & (1 << (i as u32))) != 0 && (mem_type.propertyFlags & vk_memory_property_flag_bits).0 != 0 {
            return i as u32;
        }
    }
    panic!()
}

fn memory_offset_iterator<'a, I>(vk_device: &'a vk::VkDevice, iter: I) -> impl Iterator<Item = u64> + use<'a, I>
where
    I: Iterator<Item = &'a dyn MemoryRequirements>,
{
    iter.scan(0, |offset, el| {
        let reqs = el.get_memory_requirements(*vk_device);
        *offset = align_up(*offset, reqs.alignment);
        let current_resource_start = *offset;
        *offset += reqs.size;
        Some(current_resource_start)
    })
}

fn calculate_memory_size<'a, I>(vk_device: &'a vk::VkDevice, resources: I) -> vk::VkDeviceSize
where
    I: Iterator<Item = &'a dyn MemoryRequirements>,
{
    resources.fold(0u64, |acc, resource| {
        let reqs = resource.get_memory_requirements(*vk_device);
        align_up(acc, reqs.alignment) + reqs.size
    })
}

fn allocate_bind_memory<'a, I>(
    vk_physical_device: vk::VkPhysicalDevice,
    vk_device: &'a vk::VkDevice,
    resources: I,
    vk_memory_property_flag_bits: vk::VkMemoryPropertyFlagBits,
) -> vk::VkDeviceMemory
where
    I: Iterator<Item = &'a dyn MemoryRequirements> + Clone,
{
    let total_size = calculate_memory_size(vk_device, resources.clone());
    let memory_type_bits = resources.clone().fold(u32::MAX, |acc, resource| {
        acc & resource.get_memory_requirements(*vk_device).memoryTypeBits
    });

    let vk_memory_allocate_info = vk::VkMemoryAllocateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        allocationSize: total_size,
        memoryTypeIndex: find_memory_type_index(vk_physical_device, memory_type_bits, vk_memory_property_flag_bits),
        ..Default::default()
    };
    let memory = vk_create!(vk::vkAllocateMemory, *vk_device, &vk_memory_allocate_info, std::ptr::null());
    for (resource, offset) in resources.clone().zip(memory_offset_iterator(vk_device, resources)) {
        resource.bind_memory(*vk_device, memory, offset);
    }
    memory
}

const fn array_map_refs<T, const N: usize>(arr: &[T; N]) -> [&T; N] {
    let mut res: [*const T; N] = [std::ptr::null(); N];
    let mut i = 0;
    while i < N {
        res[i] = &arr[i] as *const T;
        i += 1;
    }
    unsafe { std::mem::transmute_copy::<[*const T; N], [&T; N]>(&res) }
}

macro_rules! map_to_refs_as {
    ($arr:expr) => {
        array_map_refs($arr).map(|el| el as &_)
    };
}

const STAGING_BUFFERS_COUNT: usize = 2;
const SETS_COUNT: usize = STAGING_BUFFERS_COUNT;

const VK_DESCRIPTOR_SET_LAYOUT_BINDINGS: [vk::VkDescriptorSetLayoutBinding; 3] = [
    vk::VkDescriptorSetLayoutBinding {
        binding: 0,
        descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        descriptorCount: 1,
        stageFlags: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
        pImmutableSamplers: std::ptr::null(),
    },
    vk::VkDescriptorSetLayoutBinding {
        binding: 1,
        descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
        descriptorCount: 1,
        stageFlags: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT,
        pImmutableSamplers: std::ptr::null(),
    },
    vk::VkDescriptorSetLayoutBinding {
        binding: 2,
        descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
        descriptorCount: 1,
        stageFlags: vk::VkShaderStageFlagBits(
            vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT.0 | vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT.0,
        ),
        pImmutableSamplers: std::ptr::null(),
    },
];

fn create_descriptor_set_layout(vk_device: vk::VkDevice) -> vk::VkDescriptorSetLayout {
    let vk_descriptor_set_layout_create_info = vk::VkDescriptorSetLayoutCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        pBindings: VK_DESCRIPTOR_SET_LAYOUT_BINDINGS.as_ptr(),
        bindingCount: VK_DESCRIPTOR_SET_LAYOUT_BINDINGS.len() as u32,
        ..Default::default()
    };
    let mut vk_descriptor_set_layout = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateDescriptorSetLayout(
            vk_device,
            &vk_descriptor_set_layout_create_info,
            std::ptr::null(),
            &mut vk_descriptor_set_layout
        ));
    }
    vk_descriptor_set_layout
}

fn create_descriptor_pool(vk_device: vk::VkDevice) -> vk::VkDescriptorPool {
    let pool_sizes = VK_DESCRIPTOR_SET_LAYOUT_BINDINGS.map(|binding| vk::VkDescriptorPoolSize {
        type_: binding.descriptorType,
        descriptorCount: SETS_COUNT as u32,
    });

    let vk_descriptor_pool_create_info = vk::VkDescriptorPoolCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        pPoolSizes: pool_sizes.as_ptr(),
        poolSizeCount: pool_sizes.len() as u32,
        maxSets: SETS_COUNT as u32,
        ..Default::default()
    };

    let mut vk_descriptor_pool = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateDescriptorPool(
            vk_device,
            &vk_descriptor_pool_create_info,
            std::ptr::null(),
            &mut vk_descriptor_pool
        ));
    }
    vk_descriptor_pool
}

fn create_descriptor_sets(
    vk_device: vk::VkDevice,
    vk_descriptor_pool: vk::VkDescriptorPool,
    vk_descriptor_set_layout: vk::VkDescriptorSetLayout,
    vk_hist_buffer: vk::VkBuffer,
    vk_staging_buffers: &[vk::VkBuffer; SETS_COUNT],
    vk_video_image_view: vk::VkImageView,
    vk_video_sampler: vk::VkSampler,
) -> [vk::VkDescriptorSet; SETS_COUNT] {
    let mut vk_descriptor_sets: [vk::VkDescriptorSet; SETS_COUNT] = Default::default();
    {
        let layouts = [vk_descriptor_set_layout; SETS_COUNT];
        let vk_descriptor_set_allocate_info = vk::VkDescriptorSetAllocateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            descriptorPool: vk_descriptor_pool,
            descriptorSetCount: SETS_COUNT as u32,
            pSetLayouts: layouts.as_ptr(),
            ..Default::default()
        };
        unsafe {
            vk_assert!(vk::vkAllocateDescriptorSets(
                vk_device,
                &vk_descriptor_set_allocate_info,
                vk_descriptor_sets.as_mut_ptr() as *mut vk::VkDescriptorSet
            ));
        }
    }

    for (staging_buffer, set) in vk_staging_buffers.iter().zip(&vk_descriptor_sets) {
        let vk_descriptor_image_info = vk::VkDescriptorImageInfo {
            imageLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            imageView: vk_video_image_view,
            sampler: vk_video_sampler,
        };

        let vk_write_descriptor_set_0 = vk::VkWriteDescriptorSet {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            dstSet: *set,
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            descriptorCount: 1,
            pImageInfo: &vk_descriptor_image_info,
            ..Default::default()
        };

        let vk_descriptor_buffer_info_1 = vk::VkDescriptorBufferInfo {
            buffer: *staging_buffer,
            offset: 0,
            range: vk::VK_WHOLE_SIZE,
        };

        let vk_write_descriptor_set_1 = vk::VkWriteDescriptorSet {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            dstSet: *set,
            dstBinding: 1,
            dstArrayElement: 0,
            descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            descriptorCount: 1,
            pBufferInfo: &vk_descriptor_buffer_info_1,
            ..Default::default()
        };

        let vk_descriptor_buffer_info_2 = vk::VkDescriptorBufferInfo {
            buffer: vk_hist_buffer,
            offset: 0,
            range: vk::VK_WHOLE_SIZE,
        };

        let vk_write_descriptor_set_2 = vk::VkWriteDescriptorSet {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            dstSet: *set,
            dstBinding: 2,
            dstArrayElement: 0,
            descriptorType: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            descriptorCount: 1,
            pBufferInfo: &vk_descriptor_buffer_info_2,
            ..Default::default()
        };

        let writes = [vk_write_descriptor_set_0, vk_write_descriptor_set_1, vk_write_descriptor_set_2];

        unsafe {
            vk::vkUpdateDescriptorSets(vk_device, writes.len() as u32, writes.as_ptr(), 0, std::ptr::null());
        }
    }
    vk_descriptor_sets
}

const REQUIRED_SURFACE_FORMAT: vk::VkSurfaceFormatKHR = vk::VkSurfaceFormatKHR {
    format: vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM,
    colorSpace: vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
};
const REQUIRED_PRESENT_MODE: vk::VkPresentModeKHR = vk::VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR;

fn recreate_swapchain(
    vk_swapchain_khr: &mut vk::VkSwapchainKHR,
    glfw_window: *mut vk::GLFWwindow,
    vk_physical_device: vk::VkPhysicalDevice,
    vk_surface_khr: vk::VkSurfaceKHR,
    vk_device: vk::VkDevice,
) -> vk::VkExtent2D {
    let mut vk_surface_capabilities_khr = vk::VkSurfaceCapabilitiesKHR::default();
    unsafe {
        vk_assert!(vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            vk_physical_device,
            vk_surface_khr,
            &mut vk_surface_capabilities_khr
        ));
    }

    let vk_surface_format_khrs = vk_enumerate!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR, vk_physical_device, vk_surface_khr);

    for format in &vk_surface_format_khrs {
        println!("Supported format: {:?}", format.format);
    }

    assert!(
        vk_surface_format_khrs
            .iter()
            .any(|f| { f.format == REQUIRED_SURFACE_FORMAT.format && f.colorSpace == REQUIRED_SURFACE_FORMAT.colorSpace })
    );

    let vk_present_modes_khrs = vk_enumerate!(vk::vkGetPhysicalDeviceSurfacePresentModesKHR, vk_physical_device, vk_surface_khr);

    for mode in &vk_present_modes_khrs {
        println!("Supported present mode: {:?}", mode);
    }

    assert!(vk_present_modes_khrs.iter().any(|&mode| mode == REQUIRED_PRESENT_MODE));

    let vk_swapchain_extent = unsafe {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        vk::glfwGetFramebufferSize(glfw_window, &mut width, &mut height);
        vk::VkExtent2D {
            width: (width as u32).clamp(
                vk_surface_capabilities_khr.minImageExtent.width,
                vk_surface_capabilities_khr.maxImageExtent.width,
            ),
            height: (height as u32).clamp(
                vk_surface_capabilities_khr.minImageExtent.height,
                vk_surface_capabilities_khr.maxImageExtent.height,
            ),
        }
    };

    // Assert that there is no limit to max image count (0 means unlimited in Vulkan)
    assert_eq!(vk_surface_capabilities_khr.maxImageCount, 0);

    let old_swapchain = *vk_swapchain_khr;

    let vk_swapchain_create_info_khr = vk::VkSwapchainCreateInfoKHR {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        surface: vk_surface_khr,
        minImageCount: vk_surface_capabilities_khr.minImageCount + 1,
        imageFormat: REQUIRED_SURFACE_FORMAT.format,
        imageColorSpace: REQUIRED_SURFACE_FORMAT.colorSpace,
        imageExtent: vk_swapchain_extent,
        imageArrayLayers: 1,
        imageUsage: vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT,
        imageSharingMode: vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
        preTransform: vk_surface_capabilities_khr.currentTransform,
        compositeAlpha: vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        presentMode: REQUIRED_PRESENT_MODE,
        clipped: vk::VK_TRUE,
        oldSwapchain: old_swapchain,
        ..Default::default()
    };

    unsafe {
        vk_assert!(vk::vkCreateSwapchainKHR(
            vk_device,
            &vk_swapchain_create_info_khr,
            std::ptr::null(),
            vk_swapchain_khr
        ));

        if old_swapchain != std::ptr::null_mut() {
            vk::vkDestroySwapchainKHR(vk_device, old_swapchain, std::ptr::null());
        }
    }
    vk_swapchain_extent
}

macro_rules! include_spirv {
    ($path:expr) => {{
        const BYTES: &[u8] = include_bytes!(env!($path));

        // Static Assert: Ensure length is a multiple of 4
        const _: () = assert!(BYTES.len() % 4 == 0, "SPIR-V binary must be 32-bit aligned");

        // Transmute bytes to u32 at compile time
        // Note: This requires the bytes to be aligned in the binary.
        // include_bytes! usually aligns to 1, so we use a union to force alignment.
        #[repr(C)]
        union Transmuter {
            bytes: [u8; BYTES.len()],
            words: [u32; BYTES.len() / 4],
        }

        const TRANSMUTED: &[u32] = unsafe {
            &Transmuter {
                bytes: *include_bytes!(env!($path)),
            }
            .words
        };

        TRANSMUTED
    }};
}

const SHADER_CODE: &[u32] = include_spirv!("SHADERS_PATH");

fn create_shader_module(vk_device: vk::VkDevice) -> vk::VkShaderModule {
    let vk_shader_module_create_info = vk::VkShaderModuleCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        codeSize: SHADER_CODE.len() as usize * 4usize,
        pCode: SHADER_CODE.as_ptr(),
        ..Default::default()
    };
    let mut vk_shader_module = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateShaderModule(
            vk_device,
            &vk_shader_module_create_info,
            std::ptr::null(),
            &mut vk_shader_module
        ));
    }
    vk_shader_module
}

fn create_compute_pipeline(
    vk_device: vk::VkDevice,
    vk_pipeline_layout: vk::VkPipelineLayout,
    vk_shader_module: vk::VkShaderModule,
    entry_point: &str,
) -> vk::VkPipeline {
    let entry_point = std::ffi::CString::new(entry_point).unwrap();
    let mut vk_pipeline: vk::VkPipeline = Default::default();
    let vk_compute_pipeline_create_info = vk::VkComputePipelineCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
        stage: vk::VkPipelineShaderStageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT,
            module: vk_shader_module,
            pName: entry_point.as_ptr(),
            ..Default::default()
        },
        layout: vk_pipeline_layout,
        ..Default::default()
    };
    unsafe {
        vk_assert!(vk::vkCreateComputePipelines(
            vk_device,
            std::ptr::null_mut(),
            1,
            &vk_compute_pipeline_create_info,
            std::ptr::null(),
            &mut vk_pipeline
        ));
    }
    vk_pipeline
}

fn create_render_pass(vk_device: vk::VkDevice) -> vk::VkRenderPass {
    let vk_attachment_description = vk::VkAttachmentDescription {
        format: REQUIRED_SURFACE_FORMAT.format,
        samples: vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
        loadOp: vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        storeOp: vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE,
        stencilLoadOp: vk::VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        stencilStoreOp: vk::VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
        initialLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
        finalLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        ..Default::default()
    };

    let vk_attachment_reference = vk::VkAttachmentReference {
        attachment: 0,
        layout: vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    };

    let vk_subpass_description = vk::VkSubpassDescription {
        pipelineBindPoint: vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
        colorAttachmentCount: 1,
        pColorAttachments: &vk_attachment_reference,
        ..Default::default()
    };

    let vk_subpass_dependency = vk::VkSubpassDependency {
        srcSubpass: vk::VK_SUBPASS_EXTERNAL,
        dstSubpass: 0,
        srcStageMask: vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        srcAccessMask: vk::VkAccessFlagBits(0),
        dstStageMask: vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
        dependencyFlags: vk::VkDependencyFlagBits(0),
    };

    let vk_render_pass_create_info = vk::VkRenderPassCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        attachmentCount: 1,
        pAttachments: &vk_attachment_description,
        subpassCount: 1,
        pSubpasses: &vk_subpass_description,
        dependencyCount: 1,
        pDependencies: &vk_subpass_dependency,
        ..Default::default()
    };

    let mut vk_render_pass = Default::default();
    unsafe {
        vk_assert!(vk::vkCreateRenderPass(
            vk_device,
            &vk_render_pass_create_info,
            std::ptr::null(),
            &mut vk_render_pass
        ));
    }
    vk_render_pass
}

type VkGetFenceFdKHR =
    unsafe extern "C" fn(device: vk::VkDevice, pGetFdInfo: *const vk::VkFenceGetFdInfoKHR, pFd: *mut ::core::ffi::c_int) -> vk::VkResult;

struct FenceFd<'fd> {
    fd: std::os::fd::OwnedFd,
    epoll: &'fd nix::sys::epoll::Epoll,
}

impl<'fd> Drop for FenceFd<'fd> {
    fn drop(&mut self) {
        self.epoll.delete(&self.fd).unwrap();
    }
}

impl<'fd> FenceFd<'fd> {
    pub fn new(
        epoll: &'fd nix::sys::epoll::Epoll,
        vk_device: vk::VkDevice,
        vk_fence_get_fd_info: &vk::VkFenceGetFdInfoKHR,
        vk_get_fence_fd_khr: VkGetFenceFdKHR,
    ) -> Self {
        let mut fd = i32::default();
        vk_assert!(unsafe { vk_get_fence_fd_khr(vk_device, vk_fence_get_fd_info as *const _, &mut fd) });
        let fd = unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) };
        epoll
            .add(
                &fd,
                nix::sys::epoll::EpollEvent::new(nix::sys::epoll::EpollFlags::EPOLLIN, fd.as_raw_fd() as u64),
            )
            .unwrap();
        Self { fd, epoll }
    }
}

fn create_graphics_pipeline(
    vk_device: vk::VkDevice,
    vk_render_pass: vk::VkRenderPass,
    vk_swapchain_extent: &vk::VkExtent2D,
    vk_pipeline_layout: vk::VkPipelineLayout,
    vk_shader_module: vk::VkShaderModule,
    vertex_shader_entry_point: &str,
    fragment_shader_entry_point: &str,
) -> vk::VkPipeline {
    let vertex_shader_entry_point = std::ffi::CString::new(vertex_shader_entry_point).unwrap();
    let fragment_shader_entry_point = std::ffi::CString::new(fragment_shader_entry_point).unwrap();
    let shader_stages = [
        vk::VkPipelineShaderStageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
            module: vk_shader_module,
            pName: vertex_shader_entry_point.as_ptr(),
            ..Default::default()
        },
        vk::VkPipelineShaderStageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
            module: vk_shader_module,
            pName: fragment_shader_entry_point.as_ptr(),
            ..Default::default()
        },
    ];

    let vertex_input_info = vk::VkPipelineVertexInputStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        ..Default::default()
    };

    let input_assembly_info = vk::VkPipelineInputAssemblyStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vk::VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        primitiveRestartEnable: vk::VK_FALSE,
        ..Default::default()
    };

    let viewport = vk::VkViewport {
        x: 0.0,
        y: 0.0,
        width: vk_swapchain_extent.width as f32,
        height: vk_swapchain_extent.height as f32,
        minDepth: 0.0,
        maxDepth: 1.0,
    };

    let scissor = vk::VkRect2D {
        offset: vk::VkOffset2D { x: 0, y: 0 },
        extent: *vk_swapchain_extent,
    };

    let viewport_state_info = vk::VkPipelineViewportStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewportCount: 1,
        pViewports: &viewport,
        scissorCount: 1,
        pScissors: &scissor,
        ..Default::default()
    };

    let rasterizer_info = vk::VkPipelineRasterizationStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        depthClampEnable: vk::VK_FALSE,
        rasterizerDiscardEnable: vk::VK_FALSE,
        polygonMode: vk::VkPolygonMode::VK_POLYGON_MODE_FILL,
        lineWidth: 1.0,
        cullMode: vk::VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT,
        frontFace: vk::VkFrontFace::VK_FRONT_FACE_CLOCKWISE,
        depthBiasEnable: vk::VK_FALSE,
        depthBiasConstantFactor: 0.0,
        depthBiasClamp: 0.0,
        depthBiasSlopeFactor: 0.0,
        ..Default::default()
    };

    let multisample_info = vk::VkPipelineMultisampleStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        sampleShadingEnable: vk::VK_FALSE,
        rasterizationSamples: vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
        minSampleShading: 1.0,
        pSampleMask: std::ptr::null(),
        alphaToCoverageEnable: vk::VK_FALSE,
        alphaToOneEnable: vk::VK_FALSE,
        ..Default::default()
    };

    let color_blend_attachment = vk::VkPipelineColorBlendAttachmentState {
        colorWriteMask: vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT,
        blendEnable: vk::VK_FALSE,
        srcColorBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ONE,
        dstColorBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ZERO,
        colorBlendOp: vk::VkBlendOp::VK_BLEND_OP_ADD,
        srcAlphaBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ZERO,
        alphaBlendOp: vk::VkBlendOp::VK_BLEND_OP_ADD,
    };

    let color_blend_info = vk::VkPipelineColorBlendStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        logicOpEnable: vk::VK_FALSE,
        logicOp: vk::VkLogicOp::VK_LOGIC_OP_COPY,
        attachmentCount: 1,
        pAttachments: &color_blend_attachment,
        blendConstants: [0.0, 0.0, 0.0, 0.0],
        ..Default::default()
    };

    let pipeline_info = vk::VkGraphicsPipelineCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        stageCount: shader_stages.len() as u32,
        pStages: shader_stages.as_ptr(),
        pVertexInputState: &vertex_input_info,
        pInputAssemblyState: &input_assembly_info,
        pViewportState: &viewport_state_info,
        pRasterizationState: &rasterizer_info,
        pMultisampleState: &multisample_info,
        pDepthStencilState: std::ptr::null(),
        pColorBlendState: &color_blend_info,
        layout: vk_pipeline_layout,
        renderPass: vk_render_pass,
        subpass: 0,
        basePipelineHandle: std::ptr::null_mut(),
        basePipelineIndex: -1,
        ..Default::default()
    };
    let mut vk_graphics_pipeline = vk::VkPipeline::default();
    unsafe {
        vk_assert!(vk::vkCreateGraphicsPipelines(
            vk_device,
            std::ptr::null_mut(),
            1,
            &pipeline_info,
            std::ptr::null(),
            &mut vk_graphics_pipeline
        ));
    }
    vk_graphics_pipeline
}

fn create_graphics_line_topology_pipeline(
    vk_device: vk::VkDevice,
    vk_render_pass: vk::VkRenderPass,
    vk_swapchain_extent: &vk::VkExtent2D,
    vk_pipeline_layout: vk::VkPipelineLayout,
    vk_shader_module: vk::VkShaderModule,
    line_width: f32,
    vertex_shader_entry_point: &str,
    fragment_shader_entry_point: &str,
) -> vk::VkPipeline {
    let vertex_shader_entry_point = std::ffi::CString::new(vertex_shader_entry_point).unwrap();
    let fragment_shader_entry_point = std::ffi::CString::new(fragment_shader_entry_point).unwrap();
    let shader_stages = [
        vk::VkPipelineShaderStageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
            module: vk_shader_module,
            pName: vertex_shader_entry_point.as_ptr(),
            ..Default::default()
        },
        vk::VkPipelineShaderStageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
            module: vk_shader_module,
            pName: fragment_shader_entry_point.as_ptr(),
            ..Default::default()
        },
    ];

    let vertex_input_info = vk::VkPipelineVertexInputStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        ..Default::default()
    };

    let input_assembly_info = vk::VkPipelineInputAssemblyStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vk::VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
        primitiveRestartEnable: vk::VK_FALSE,
        ..Default::default()
    };

    let viewport = vk::VkViewport {
        x: 0.0,
        y: 0.0,
        width: vk_swapchain_extent.width as f32,
        height: vk_swapchain_extent.height as f32,
        minDepth: 0.0,
        maxDepth: 1.0,
    };

    let scissor = vk::VkRect2D {
        offset: vk::VkOffset2D { x: 0, y: 0 },
        extent: *vk_swapchain_extent,
    };

    let viewport_state_info = vk::VkPipelineViewportStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewportCount: 1,
        pViewports: &viewport,
        scissorCount: 1,
        pScissors: &scissor,
        ..Default::default()
    };

    let rasterizer_info = vk::VkPipelineRasterizationStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        depthClampEnable: vk::VK_FALSE,
        rasterizerDiscardEnable: vk::VK_FALSE,
        polygonMode: vk::VkPolygonMode::VK_POLYGON_MODE_FILL,
        lineWidth: line_width,
        cullMode: vk::VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT,
        frontFace: vk::VkFrontFace::VK_FRONT_FACE_CLOCKWISE,
        depthBiasEnable: vk::VK_FALSE,
        depthBiasConstantFactor: 0.0,
        depthBiasClamp: 0.0,
        depthBiasSlopeFactor: 0.0,
        ..Default::default()
    };

    let multisample_info = vk::VkPipelineMultisampleStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        sampleShadingEnable: vk::VK_FALSE,
        rasterizationSamples: vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
        minSampleShading: 1.0,
        pSampleMask: std::ptr::null(),
        alphaToCoverageEnable: vk::VK_FALSE,
        alphaToOneEnable: vk::VK_FALSE,
        ..Default::default()
    };

    let color_blend_attachment = vk::VkPipelineColorBlendAttachmentState {
        colorWriteMask: vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT
            | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT,
        blendEnable: vk::VK_FALSE,
        srcColorBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ONE,
        dstColorBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ZERO,
        colorBlendOp: vk::VkBlendOp::VK_BLEND_OP_ADD,
        srcAlphaBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: vk::VkBlendFactor::VK_BLEND_FACTOR_ZERO,
        alphaBlendOp: vk::VkBlendOp::VK_BLEND_OP_ADD,
    };

    let color_blend_info = vk::VkPipelineColorBlendStateCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        logicOpEnable: vk::VK_FALSE,
        logicOp: vk::VkLogicOp::VK_LOGIC_OP_COPY,
        attachmentCount: 1,
        pAttachments: &color_blend_attachment,
        blendConstants: [0.0, 0.0, 0.0, 0.0],
        ..Default::default()
    };

    let pipeline_info = vk::VkGraphicsPipelineCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        stageCount: shader_stages.len() as u32,
        pStages: shader_stages.as_ptr(),
        pVertexInputState: &vertex_input_info,
        pInputAssemblyState: &input_assembly_info,
        pViewportState: &viewport_state_info,
        pRasterizationState: &rasterizer_info,
        pMultisampleState: &multisample_info,
        pDepthStencilState: std::ptr::null(),
        pColorBlendState: &color_blend_info,
        layout: vk_pipeline_layout,
        renderPass: vk_render_pass,
        subpass: 0,
        basePipelineHandle: std::ptr::null_mut(),
        basePipelineIndex: -1,
        ..Default::default()
    };
    let mut vk_graphics_pipeline = vk::VkPipeline::default();
    unsafe {
        vk_assert!(vk::vkCreateGraphicsPipelines(
            vk_device,
            std::ptr::null_mut(),
            1,
            &pipeline_info,
            std::ptr::null(),
            &mut vk_graphics_pipeline
        ));
    }
    vk_graphics_pipeline
}

fn create_pipeline_layout(vk_device: vk::VkDevice, vk_descriptor_set_layout: vk::VkDescriptorSetLayout) -> vk::VkPipelineLayout {
    let mut vk_pipeline_layout: vk::VkPipelineLayout = Default::default();
    let push_constant_range = [vk::VkPushConstantRange {
        stageFlags: vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT | vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
        offset: 0,
        size: std::mem::size_of::<my_shaders::player::PushConstants>() as u32,
    }];

    let vk_pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
        pSetLayouts: &vk_descriptor_set_layout,
        setLayoutCount: 1,
        // Inject the push constant here
        pPushConstantRanges: push_constant_range.as_ptr(),
        pushConstantRangeCount: push_constant_range.len() as u32,
        ..Default::default()
    };
    unsafe {
        vk_assert!(vk::vkCreatePipelineLayout(
            vk_device,
            &vk_pipeline_layout_create_info,
            std::ptr::null(),
            &mut vk_pipeline_layout
        ));
    }
    vk_pipeline_layout
}

fn single_time_command<F: Fn(vk::VkCommandBuffer)>(vk_device: vk::VkDevice, vk_command_pool: vk::VkCommandPool, vk_queue: vk::VkQueue, func: F) {
    let mut vk_command_buffer = vk::VkCommandBuffer::default();
    let vk_command_buffer_allocate_info = vk::VkCommandBufferAllocateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        commandBufferCount: 1,
        commandPool: vk_command_pool,
        level: vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        ..Default::default()
    };
    unsafe {
        vk_assert!(vk::vkAllocateCommandBuffers(
            vk_device,
            &vk_command_buffer_allocate_info,
            &mut vk_command_buffer
        ));
    }
    {
        let vk_command_buffer_begin_info = vk::VkCommandBufferBeginInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::VkCommandBufferUsageFlagBits::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            ..Default::default()
        };
        unsafe {
            vk_assert!(vk::vkBeginCommandBuffer(vk_command_buffer, &vk_command_buffer_begin_info));
        }
        func(vk_command_buffer);
        unsafe {
            vk_assert!(vk::vkEndCommandBuffer(vk_command_buffer));
        }
    }
    let vk_submit_info = vk::VkSubmitInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
        commandBufferCount: 1,
        pCommandBuffers: &vk_command_buffer,
        ..Default::default()
    };
    unsafe {
        vk_assert!(vk::vkQueueSubmit(vk_queue, 1, &vk_submit_info, std::ptr::null_mut()));
    }
    unsafe {
        vk_assert!(vk::vkQueueWaitIdle(vk_queue));
    }
    unsafe {
        vk::vkFreeCommandBuffers(vk_device, vk_command_pool, 1, &mut vk_command_buffer);
    }
}

unsafe extern "C" fn imgui_check_vk_result_fn(vk_result: vk::VkResult) {
    vk_assert!(vk_result);
}

struct ImGuiContext {
    imgui_context: *mut vk::ImGuiContext,
    implot_context: *mut vk::ImPlotContext,
    vk_descriptor_pool: vk::VkDescriptorPool,
    vk_device: vk::VkDevice,
}

impl Drop for ImGuiContext {
    fn drop(&mut self) {
        unsafe {
            vk::ImGui_ImplVulkan_Shutdown();
            vk::ImGui_ImplGlfw_Shutdown();
            vk::ImPlot_DestroyContext(self.implot_context);
            vk::igDestroyContext(self.imgui_context);
            vk::vkDestroyDescriptorPool(self.vk_device, self.vk_descriptor_pool, std::ptr::null());
        }
    }
}

impl ImGuiContext {
    fn new(
        glfw_window: &mut vk::GLFWwindow,
        vk_instance: vk::VkInstance,
        vk_physical_device: vk::VkPhysicalDevice,
        vk_surface_khr: vk::VkSurfaceKHR,
        vk_queue_family_index: u32,
        vk_queue: vk::VkQueue,
        vk_device: vk::VkDevice,
        vk_render_pass: vk::VkRenderPass,
        vk_framebuffers_count: u32,
    ) -> Self {
        const IMGUI_IMPL_VULKAN_MINIMUM_IMAGE_SAMPLER_POOL_SIZE: u32 = 8;
        let vk_descriptor_pool = {
            let vk_descriptor_pool_sizes = [vk::VkDescriptorPoolSize {
                type_: vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                descriptorCount: IMGUI_IMPL_VULKAN_MINIMUM_IMAGE_SAMPLER_POOL_SIZE,
            }];
            let vk_descriptor_pool_create_info = vk::VkDescriptorPoolCreateInfo {
                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                flags: vk::VkDescriptorPoolCreateFlagBits::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
                maxSets: IMGUI_IMPL_VULKAN_MINIMUM_IMAGE_SAMPLER_POOL_SIZE,
                poolSizeCount: vk_descriptor_pool_sizes.len() as u32,
                pPoolSizes: vk_descriptor_pool_sizes.as_ptr(),
                ..Default::default()
            };
            vk_create!(vk::vkCreateDescriptorPool, vk_device, &vk_descriptor_pool_create_info, std::ptr::null())
        };

        let imgui_context = unsafe { vk::igCreateContext(std::ptr::null_mut()).as_mut().unwrap() };
        let io = unsafe { vk::igGetIO_Nil().as_mut().unwrap() };
        io.ConfigFlags |= vk::ImGuiConfigFlags_::ImGuiConfigFlags_NavEnableKeyboard.0 as i32;

        let implot_context = unsafe { vk::ImPlot_CreateContext() };

        unsafe {
            assert!(vk::ImGui_ImplGlfw_InitForVulkan(glfw_window, true));
        }

        {
            let mut imgui_impl_vulkan_init_info = vk::ImGui_ImplVulkan_InitInfo {
                ApiVersion: vk::vk_make_api_version(0, 1, 3, 0),
                Instance: vk_instance,
                PhysicalDevice: vk_physical_device,
                Device: vk_device,
                QueueFamily: vk_queue_family_index,
                Queue: vk_queue,
                DescriptorPool: vk_descriptor_pool,
                ..Default::default()
            };

            let mut vk_surface_capabilities_khr = vk::VkSurfaceCapabilitiesKHR::default();
            unsafe {
                vk_assert!(vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                    vk_physical_device,
                    vk_surface_khr,
                    &mut vk_surface_capabilities_khr
                ));
            }

            imgui_impl_vulkan_init_info.MinImageCount = vk_surface_capabilities_khr.minImageCount;
            imgui_impl_vulkan_init_info.ImageCount = vk_framebuffers_count;
            imgui_impl_vulkan_init_info.PipelineInfoMain.RenderPass = vk_render_pass;
            imgui_impl_vulkan_init_info.PipelineInfoMain.MSAASamples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
            imgui_impl_vulkan_init_info.CheckVkResultFn = Some(imgui_check_vk_result_fn);
            unsafe {
                assert!(vk::ImGui_ImplVulkan_Init(&mut imgui_impl_vulkan_init_info));
            }
        }
        Self {
            imgui_context,
            implot_context,
            vk_descriptor_pool,
            vk_device,
        }
    }
}

pub const IMPLOT_AUTO_COL: vk::ImVec4 = vk::ImVec4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: -1.0,
};

// ImPlot defines IMPLOT_AUTO as -1
pub const IMPLOT_AUTO: i32 = -1;

pub const IMPLOT_SPEC_DEFAULT: vk::ImPlotSpec = vk::ImPlotSpec {
    LineColor: IMPLOT_AUTO_COL,
    LineWeight: 1.0,
    FillColor: IMPLOT_AUTO_COL,
    FillAlpha: 1.0,

    // Note: Depending on how bindgen parsed the enum fields, you might
    // need to append `.0` to these enum variants (e.g. `vk::ImPlotMarker_::ImPlotMarker_None.0`)
    // if the struct field is typed as an i32 instead of the newtype enum.
    Marker: vk::ImPlotMarker_::ImPlotMarker_None.0,
    MarkerSize: 4.0,
    MarkerLineColor: IMPLOT_AUTO_COL,
    MarkerFillColor: IMPLOT_AUTO_COL,
    Size: 4.0,
    Offset: 0,
    Stride: IMPLOT_AUTO,
    Flags: vk::ImPlotItemFlags_::ImPlotItemFlags_None.0,
};

fn imgui_ui(target_pos: &mut Option<[f32; 2]>) {
    unsafe {
        vk::ImGui_ImplVulkan_NewFrame();
        vk::ImGui_ImplGlfw_NewFrame();
        vk::igNewFrame();
    }
    unsafe {
        let main_viewport = vk::igGetMainViewport();
        vk::igSetNextWindowPos(vk::ImVec2::default(), vk::ImGuiCond_::ImGuiCond_Always.0, vk::ImVec2_c::default());
        vk::igSetNextWindowSize((*main_viewport).WorkSize, vk::ImGuiCond_::ImGuiCond_Always.0);
    }
    let mut open = true;

    {
        unsafe {
            vk::igPushStyleVarX(vk::ImGuiStyleVar_::ImGuiStyleVar_WindowPadding.0, 0.0);
            vk::igPushStyleVarY(vk::ImGuiStyleVar_::ImGuiStyleVar_WindowPadding.0, 0.0);
        }
        defer!(unsafe {
            vk::igPopStyleVar(2);
            vk::igEnd();
        });

        if unsafe {
            vk::igBegin(
                c_str!("imgui_window"),
                &mut open,
                (vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoTitleBar
                    | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoMove
                    | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoResize
                    | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoDecoration
                    | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoBackground)
                    .0,
            )
        } {
            let available_space = unsafe { vk::igGetContentRegionAvail() };
            if unsafe {
                vk::igBeginChild_Str(
                    c_str!("empty_area"),
                    vk::ImVec2 {
                        x: available_space.x,
                        y: available_space.y - 70f32,
                    },
                    vk::ImGuiChildFlags_::ImGuiChildFlags_Borders.0,
                    (vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoResize
                        | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoMove
                        | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoBackground)
                        .0,
                )
            } {
                // Check if the child window is hovered and left-clicked (0 = Left Mouse Button)
                if unsafe { vk::igIsWindowHovered(0) } && unsafe { vk::igIsMouseClicked_Bool(0, false) } {
                    let mouse_pos = unsafe { vk::igGetMousePos() };

                    match target_pos {
                        None => *target_pos = Some([mouse_pos.x, mouse_pos.y]),
                        Some(_) => *target_pos = None,
                    }

                    println!("Updated target position: {:?}", target_pos);
                }

                // --- NEW: Draw the target cross/dot ---
                if let Some([px, py]) = *target_pos {
                    unsafe {
                        let draw_list = vk::igGetWindowDrawList();

                        let cross_size = 12.0;
                        let cross_thickness = 2.0;

                        // Colors in ImGui are typically u32 in ABGR format: 0xAABBGGRR
                        let color_red = 0xFF_00_00_FF; // Opaque Red
                        let color_green = 0xFF_00_FF_00; // Opaque Green

                        // Draw Horizontal Line
                        vk::ImDrawList_AddLine(
                            draw_list,
                            vk::ImVec2 { x: px - cross_size, y: py },
                            vk::ImVec2 { x: px + cross_size, y: py },
                            color_red,
                            cross_thickness,
                        );

                        // Draw Vertical Line
                        vk::ImDrawList_AddLine(
                            draw_list,
                            vk::ImVec2 { x: px, y: py - cross_size },
                            vk::ImVec2 { x: px, y: py + cross_size },
                            color_red,
                            cross_thickness,
                        );

                        // Draw Center Dot
                        vk::ImDrawList_AddCircleFilled(
                            draw_list,
                            vk::ImVec2 { x: px, y: py },
                            3.0,         // Radius
                            color_green, // Color
                            0,           // Segments (0 = auto)
                        );
                    }
                }
                // --------------------------------------
            }
            unsafe {
                vk::igEndChild();
            }

            // unsafe {
            //     vk::igPushStyleColor_Vec4(
            //         vk::ImGuiCol_::ImGuiCol_ChildBg.0,
            //         vk::ImVec4 {
            //             w: 0.5f32,
            //             ..Default::default()
            //         },
            //     );
            // }
            // defer!(unsafe {
            //     vk::igPopStyleColor(1);
            // });
            // let available_space = unsafe { vk::igGetContentRegionAvail() };
            // if unsafe {
            //     vk::igBeginChild_Str(
            //         c_str!("player"),
            //         available_space,
            //         vk::ImGuiChildFlags_::ImGuiChildFlags_Borders.0,
            //         (vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoResize | vk::ImGuiWindowFlags_::ImGuiWindowFlags_NoMove).0,
            //     )
            // } {
            //     unsafe { vk::igButton(c_str!("play"), vk::ImVec2::default()) };
            //     unsafe {
            //         vk::igSameLine(0.0, 0.0);
            //     }

            //     let available_space = unsafe { vk::igGetContentRegionAvail() };
            //     unsafe {
            //         vk::igPushItemWidth(available_space.x);
            //     }
            //     defer! { unsafe { vk::igPopItemWidth(); }}

            //     let mut value = 0f32;
            //     unsafe {
            //         vk::igSliderFloat(c_str!("##state"), &mut value, 0f32, 1f32, std::ptr::null(), 0);
            //     }
            // }
            // unsafe {
            //     vk::igEndChild();
            // }
        }
    }

    unsafe {
        vk::igRender();
    }
}

fn main() {
    let glfw_context = GlfwContext::new();
    let vk_instance = create_instance();
    defer! { unsafe {vk::vkDestroyInstance(vk_instance, std::ptr::null())} }
    let vk_physical_device = pick_physical_device(vk_instance);
    let vk_debug_utils_messenger_ext = create_debug_utils_messenger(vk_instance);
    defer! {
        unsafe { destroy_debug_utils_messenger_ext(vk_instance, vk_debug_utils_messenger_ext, std::ptr::null()); }
    }
    let mut vk_surface_khr: vk::VkSurfaceKHR = std::ptr::null_mut();
    assert_eq!(
        unsafe {
            vk::glfwCreateWindowSurface(
                std::mem::transmute(vk_instance),
                glfw_context.0,
                std::ptr::null(),
                std::mem::transmute(&mut vk_surface_khr),
            )
            .0
        },
        vk::VkResult::VK_SUCCESS.0
    );
    defer! {
        unsafe { vk::vkDestroySurfaceKHR(vk_instance, vk_surface_khr, std::ptr::null()); }
    }
    let queue_family_index = pick_graphics_presentation_compute_queue_family(vk_physical_device, vk_surface_khr);

    let vk_device = create_device(vk_physical_device, queue_family_index);
    defer! { unsafe { vk::vkDestroyDevice(vk_device, std::ptr::null()); }}

    let mut vk_queue: vk::VkQueue = Default::default();
    unsafe {
        vk::vkGetDeviceQueue(vk_device, queue_family_index, 0, &mut vk_queue);
    }

    let vk_command_pool = create_command_pool(vk_device, queue_family_index);
    defer! {unsafe{vk::vkDestroyCommandPool(vk_device, vk_command_pool, std::ptr::null())}};

    let mut vk_command_buffer = Default::default();
    {
        let vk_command_buffer_allocate_info = vk::VkCommandBufferAllocateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            commandBufferCount: 1,
            commandPool: vk_command_pool,
            level: vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            ..Default::default()
        };
        unsafe {
            vk_assert!(vk::vkAllocateCommandBuffers(
                vk_device,
                &vk_command_buffer_allocate_info,
                &mut vk_command_buffer
            ))
        };
    }
    defer! {unsafe{vk::vkFreeCommandBuffers(vk_device, vk_command_pool, 1, &vk_command_buffer);}}

    let vk_staging_buffers: [_; SETS_COUNT] = std::array::from_fn(|index| {
        let create_info = vk::VkBufferCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            usage: vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT,
            size: IMAGE_BUFFER_SIZE,
            sharingMode: vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            ..Default::default()
        };
        vk_create!(vk::vkCreateBuffer, vk_device, &create_info, std::ptr::null())
    });
    defer! {
        for buffer in &vk_staging_buffers {
            unsafe { vk::vkDestroyBuffer(vk_device, *buffer, std::ptr::null()); }
        }
    }
    let host_local_memory = allocate_bind_memory(
        vk_physical_device,
        &vk_device,
        map_to_refs_as!(&vk_staging_buffers).into_iter(),
        vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
    );
    defer! { unsafe { vk::vkFreeMemory(vk_device, host_local_memory, std::ptr::null()); } }
    let staging_buffer_slices = {
        let host_local_memory_mapped = {
            let size = calculate_memory_size(&vk_device, map_to_refs_as!(&vk_staging_buffers).into_iter());
            let ptr = vk_create!(vk::vkMapMemory, vk_device, host_local_memory, 0, size, vk::VkMemoryMapFlagBits(0));
            unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, size as usize) }
        };
        let second_buffer_offset = memory_offset_iterator(&vk_device, map_to_refs_as!(&vk_staging_buffers).into_iter())
            .last()
            .unwrap();
        let (first, second) = host_local_memory_mapped.split_at_mut(second_buffer_offset as usize);
        [first, second]
    };
    defer! {
        unsafe { vk::vkUnmapMemory(vk_device, host_local_memory); }
    }

    let vk_video_image = {
        let create_info = vk::VkImageCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            imageType: vk::VkImageType::VK_IMAGE_TYPE_2D,
            extent: vk::VkExtent3D {
                width: IMAGE_PIXEL_WIDTH,
                height: IMAGE_PIXEL_HEIGHT,
                depth: 1,
            },
            mipLevels: 1,
            arrayLayers: 1,
            format: vk::VkFormat::VK_FORMAT_R16_UNORM,
            tiling: vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
            initialLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
            usage: vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT,
            sharingMode: vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            samples: vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            ..Default::default()
        };
        vk_create!(vk::vkCreateImage, vk_device, &create_info, std::ptr::null())
    };
    defer! { unsafe { vk::vkDestroyImage(vk_device, vk_video_image, std::ptr::null()); } }

    let vk_hist_buffer = {
        let create_info = vk::VkBufferCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            usage: vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT,
            size: (256 * std::mem::size_of::<i32>()) as u64,
            sharingMode: vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            ..Default::default()
        };
        vk_create!(vk::vkCreateBuffer, vk_device, &create_info, std::ptr::null())
    };
    defer! { unsafe { vk::vkDestroyBuffer(vk_device, vk_hist_buffer, std::ptr::null()); } }
    let device_local_memory = allocate_bind_memory(
        vk_physical_device,
        &vk_device,
        [&vk_hist_buffer as &dyn MemoryRequirements, &vk_video_image as &dyn MemoryRequirements].into_iter(),
        vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
    );
    defer! { unsafe { vk::vkFreeMemory(vk_device, device_local_memory, std::ptr::null()); } }

    let vk_video_image_view = {
        let create_info = vk::VkImageViewCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            image: vk_video_image,
            viewType: vk::VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
            format: vk::VkFormat::VK_FORMAT_R16_UNORM,
            components: vk::VkComponentMapping {
                r: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                g: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                b: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                a: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
            },
            subresourceRange: vk::VkImageSubresourceRange {
                aspectMask: vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
            ..Default::default()
        };
        vk_create!(vk::vkCreateImageView, vk_device, &create_info, std::ptr::null())
    };
    defer! {
        unsafe { vk::vkDestroyImageView(vk_device, vk_video_image_view, std::ptr::null()); }
    }
    let vk_video_sampler = {
        let create_info = vk::VkSamplerCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            magFilter: vk::VkFilter::VK_FILTER_NEAREST,
            minFilter: vk::VkFilter::VK_FILTER_NEAREST,
            addressModeU: vk::VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            addressModeV: vk::VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            addressModeW: vk::VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            anisotropyEnable: vk::VK_FALSE,
            maxAnisotropy: 1.0,
            borderColor: vk::VkBorderColor::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
            unnormalizedCoordinates: vk::VK_FALSE,
            compareEnable: vk::VK_FALSE,
            compareOp: vk::VkCompareOp::VK_COMPARE_OP_ALWAYS,
            mipmapMode: vk::VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_LINEAR,
            mipLodBias: 0.0,
            minLod: 0.0,
            maxLod: 0.0,
            ..Default::default()
        };
        vk_create!(vk::vkCreateSampler, vk_device, &create_info, std::ptr::null())
    };
    defer! { unsafe { vk::vkDestroySampler(vk_device, vk_video_sampler, std::ptr::null()); } }

    let vk_descriptor_pool = create_descriptor_pool(vk_device);
    defer! { unsafe { vk::vkDestroyDescriptorPool(vk_device, vk_descriptor_pool, std::ptr::null()); } }

    let vk_descriptor_set_layout = create_descriptor_set_layout(vk_device);
    defer! { unsafe { vk::vkDestroyDescriptorSetLayout(vk_device, vk_descriptor_set_layout, std::ptr::null()); } }

    let vk_descriptor_sets = create_descriptor_sets(
        vk_device,
        vk_descriptor_pool,
        vk_descriptor_set_layout,
        vk_hist_buffer,
        &vk_staging_buffers,
        vk_video_image_view,
        vk_video_sampler,
    );

    let vk_pipeline_layout = create_pipeline_layout(vk_device, vk_descriptor_set_layout);
    defer! {
        unsafe { vk::vkDestroyPipelineLayout(vk_device, vk_pipeline_layout, std::ptr::null()) };
    }

    let shader_module = create_shader_module(vk_device);
    defer! { unsafe { vk::vkDestroyShaderModule(vk_device, shader_module, std::ptr::null()); } }

    let vk_hist_pipeline = create_compute_pipeline(vk_device, vk_pipeline_layout, shader_module, "calculate_histogram");
    defer! { unsafe { vk::vkDestroyPipeline(vk_device, vk_hist_pipeline, std::ptr::null()) }; }

    let vk_cumulative_sum_pipeline = create_compute_pipeline(vk_device, vk_pipeline_layout, shader_module, "calculate_cumulative_sum");
    defer! { unsafe { vk::vkDestroyPipeline(vk_device, vk_cumulative_sum_pipeline, std::ptr::null()) }; }

    let vk_clean_buffer_pipeline = create_compute_pipeline(vk_device, vk_pipeline_layout, shader_module, "clean_buffer");
    defer! { unsafe { vk::vkDestroyPipeline(vk_device, vk_clean_buffer_pipeline, std::ptr::null()) }; }

    let vk_render_pass = create_render_pass(vk_device);
    defer! { unsafe { vk::vkDestroyRenderPass(vk_device, vk_render_pass, std::ptr::null()); } }

    let epoll = nix::sys::epoll::Epoll::new(nix::sys::epoll::EpollCreateFlags::empty()).unwrap();
    let mut epoll_events = [nix::sys::epoll::EpollEvent::empty(); 32];
    epoll
        .add(
            &video_timer_fd,
            nix::sys::epoll::EpollEvent::new(nix::sys::epoll::EpollFlags::EPOLLIN, video_timer_fd.as_fd().as_raw_fd() as u64),
        )
        .unwrap();

    let mut staging_buffer_index_owned_by_vulkan = false;
    let mut new_staging_buffer_available = false;

    let mut vk_swapchain_khr: vk::VkSwapchainKHR = std::ptr::null_mut();

    'swapchain_loop: loop {
        let vk_present_complete_semaphore = {
            let vk_semaphore_create_info = vk::VkSemaphoreCreateInfo {
                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
                ..Default::default()
            };
            vk_create!(vk::vkCreateSemaphore, vk_device, &vk_semaphore_create_info, std::ptr::null())
        };
        defer! { unsafe { vk::vkDestroySemaphore(vk_device, vk_present_complete_semaphore, std::ptr::null()); } }

        let vk_fence = {
            let vk_export_fence_create_info = vk::VkExportFenceCreateInfo {
                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO,
                handleTypes: vk::VkExternalFenceHandleTypeFlagBits::VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT,
                ..Default::default()
            };
            let vk_fence_create_info = vk::VkFenceCreateInfo {
                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                flags: vk::VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT,
                pNext: &vk_export_fence_create_info as *const _ as *const std::ffi::c_void,
                ..Default::default()
            };
            let mut fence = vk::VkFence::default();
            unsafe {
                vk_assert!(vk::vkCreateFence(vk_device, &vk_fence_create_info, std::ptr::null(), &mut fence));
            }
            fence
        };
        defer! {
            unsafe { vk::vkDestroyFence(vk_device, vk_fence, std::ptr::null()); }
        }

        let get_fence_fd_khr: vk::PFN_vkGetFenceFdKHR = unsafe { std::mem::transmute(vk::vkGetDeviceProcAddr(vk_device, c_str!("vkGetFenceFdKHR"))) };
        let get_fence_fd_khr = get_fence_fd_khr.unwrap();

        let vk_fence_get_fd_info_khr = {
            vk::VkFenceGetFdInfoKHR {
                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR,
                fence: vk_fence,
                handleType: vk::VkExternalFenceHandleTypeFlagBits::VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT,
                ..Default::default()
            }
        };
        let mut vk_fence_fd = FenceFd::new(&epoll, vk_device, &vk_fence_get_fd_info_khr, get_fence_fd_khr);

        let vk_swapchain_extent = recreate_swapchain(&mut vk_swapchain_khr, glfw_context.0, vk_physical_device, vk_surface_khr, vk_device);
        println!("vk_swapchain_extent: {:?}", vk_swapchain_extent);

        let vk_swapchain_images = vk_enumerate!(vk::vkGetSwapchainImagesKHR, vk_device, vk_swapchain_khr);

        let vk_swapchain_image_views: Vec<vk::VkImageView> = vk_swapchain_images
            .iter()
            .map(|image| {
                let vk_image_create_info = vk::VkImageViewCreateInfo {
                    sType: vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                    image: *image,
                    viewType: vk::VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
                    format: REQUIRED_SURFACE_FORMAT.format,
                    components: vk::VkComponentMapping {
                        r: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                        g: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                        b: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                        a: vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                    },
                    subresourceRange: vk::VkImageSubresourceRange {
                        aspectMask: vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: 1,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    ..Default::default()
                };
                let mut swapchain_image_view = vk::VkImageView::default();
                unsafe {
                    vk_assert!(vk::vkCreateImageView(
                        vk_device,
                        &vk_image_create_info,
                        std::ptr::null(),
                        &mut swapchain_image_view
                    ));
                }
                swapchain_image_view
            })
            .collect();
        defer! {
            for vk_image_view in &vk_swapchain_image_views {
                unsafe { vk::vkDestroyImageView(vk_device, *vk_image_view, std::ptr::null()); }
            }
        }

        let vk_framebuffers: Vec<vk::VkFramebuffer> = vk_swapchain_image_views
            .iter()
            .map(|view| {
                let vk_framebuffer_create_info = vk::VkFramebufferCreateInfo {
                    sType: vk::VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                    renderPass: vk_render_pass,
                    attachmentCount: 1,
                    pAttachments: view as *const _,
                    width: vk_swapchain_extent.width,
                    height: vk_swapchain_extent.height,
                    layers: 1,
                    ..Default::default()
                };
                let mut framebuffer = vk::VkFramebuffer::default();
                unsafe {
                    vk_assert!(vk::vkCreateFramebuffer(
                        vk_device,
                        &vk_framebuffer_create_info,
                        std::ptr::null(),
                        &mut framebuffer
                    ));
                }
                framebuffer
            })
            .collect();
        defer! {
            for vk_framebuffer in &vk_framebuffers {
                unsafe { vk::vkDestroyFramebuffer(vk_device, *vk_framebuffer, std::ptr::null()); }
            }
        }

        let vk_graphics_pipeline = create_graphics_pipeline(
            vk_device,
            vk_render_pass,
            &vk_swapchain_extent,
            vk_pipeline_layout,
            shader_module,
            "player::fullscreen_vs",
            "player::vdr_fs",
        );
        defer! { unsafe { vk::vkDestroyPipeline(vk_device, vk_graphics_pipeline, std::ptr::null()); } }

        let vk_render_complete_semaphores: Vec<vk::VkSemaphore> = (0..vk_framebuffers.len())
            .map(|_| {
                let vk_semaphore_create_info = vk::VkSemaphoreCreateInfo {
                    sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
                    ..Default::default()
                };
                let mut semaphore = vk::VkSemaphore::default();
                unsafe {
                    vk_assert!(vk::vkCreateSemaphore(
                        vk_device,
                        &vk_semaphore_create_info,
                        std::ptr::null(),
                        &mut semaphore
                    ));
                }
                semaphore
            })
            .collect();

        defer! {
            for semaphore in &vk_render_complete_semaphores {
                unsafe { vk::vkDestroySemaphore(vk_device, *semaphore, std::ptr::null()); }
            }
        }

        defer! { unsafe { vk_assert!(vk::vkDeviceWaitIdle(vk_device)); } }

        let imgui_context = ImGuiContext::new(
            glfw_context.0,
            vk_instance,
            vk_physical_device,
            vk_surface_khr,
            queue_family_index,
            vk_queue,
            vk_device,
            vk_render_pass,
            vk_framebuffers.len() as u32,
        );

        use crate::kcf;
        let params = kcf::KCFParams {
            // learn_rate: 0.2,
            // label_sigma: 1.5,
            peak_func: kcf::make_subpix_func::<5, 64>(kcf::peak_parabola),
            ..Default::default()
        };
        let mut tracker = kcf::KCFTracker::new(params);
        let mut target_pos: Option<[f32; 2]> = None;

        let mut log_count = 0u64;

        'render_loop: loop {
            if unsafe { vk::glfwWindowShouldClose(glfw_context.0) } == vk::GLFW_TRUE as i32 {
                break 'swapchain_loop;
            }
            unsafe {
                vk::glfwPollEvents();
            }

            let events_count = epoll.wait(&mut epoll_events, nix::sys::epoll::EpollTimeout::MAX).unwrap();
            'event_loop: for event in &epoll_events[0..events_count] {
                let fd = event.data() as i32;
                if fd == video_timer_fd.as_fd().as_raw_fd() {
                    let mut expirations = [0u8; 8];
                    let _ = nix::unistd::read(&video_timer_fd, &mut expirations);

                    new_staging_buffer_available = true;
                    let frame_slice = video_frame_iterator.next().unwrap();

                    let staging_idx = !staging_buffer_index_owned_by_vulkan as usize;

                    let src_len = frame_slice.len();
                    staging_buffer_slices[staging_idx][..src_len].copy_from_slice(frame_slice);
                    // staging_buffer_slices[staging_idx].copy_from_slice(frame_slice);

                    match target_pos {
                        None => tracker.reset(),
                        Some([x, y]) => {
                            let u16_slice = unsafe {
                                std::slice::from_raw_parts(
                                    staging_buffer_slices[staging_idx].as_ptr() as *const u16,
                                    (IMAGE_PIXEL_WIDTH * IMAGE_PIXEL_HEIGHT) as usize,
                                )
                            };

                            // Replaced ndarray with our new zero-overhead image view
                            let frame_view = kcf::ImageViewU16 {
                                data: u16_slice,
                                cols: IMAGE_PIXEL_WIDTH as usize,
                                rows: IMAGE_PIXEL_HEIGHT as usize,
                            };

                            let result = if log_count % 30 == 0 {
                                let start = std::time::Instant::now();
                                let res = tracker.update(frame_view, kcf::F32x2 { x, y });
                                let end = std::time::Instant::now();
                                println!("Update took: {:?}", end - start);
                                res
                            } else {
                                tracker.update(frame_view, kcf::F32x2 { x, y })
                            };

                            log_count += 1;

                            if result.confidence < 0.5 {
                                target_pos = None;
                            } else {
                                target_pos = Some([result.position.x, result.position.y]);
                            }
                            // println!("Updated target position: {:?}", target_pos);
                            // println!("Last Confidence: {:?}", result.confidence);
                        }
                    }
                    continue 'event_loop;
                }
                if fd == vk_fence_fd.fd.as_raw_fd() {
                    unsafe {
                        vk_assert!(vk::vkResetFences(vk_device, 1, &vk_fence));
                    }

                    let mut image_index: u32 = 0;
                    let mut vk_result = unsafe {
                        vk::vkAcquireNextImageKHR(
                            vk_device,
                            vk_swapchain_khr,
                            u64::MAX,
                            vk_present_complete_semaphore,
                            std::ptr::null_mut(),
                            &mut image_index,
                        )
                    };

                    match vk_result {
                        vk::VkResult::VK_ERROR_OUT_OF_DATE_KHR | vk::VkResult::VK_SUBOPTIMAL_KHR => continue 'swapchain_loop,
                        other => vk_assert!(vk_result),
                    }
                    let vk_render_complete_semaphore = vk_render_complete_semaphores[image_index as usize];
                    let wait_stages = [vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];

                    imgui_ui(&mut target_pos);
                    let imgui_draw_data = unsafe { vk::igGetDrawData() };

                    {
                        let mut copy_new_buffer = false;
                        if new_staging_buffer_available {
                            staging_buffer_index_owned_by_vulkan = !staging_buffer_index_owned_by_vulkan;
                            new_staging_buffer_available = false;
                            copy_new_buffer = true;
                        }
                        let descriptor_set = vk_descriptor_sets[staging_buffer_index_owned_by_vulkan as usize];

                        let vk_command_buffer_begin_info = vk::VkCommandBufferBeginInfo {
                            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
                            flags: vk::VkCommandBufferUsageFlagBits::VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
                            ..Default::default()
                        };

                        unsafe {
                            vk_assert!(vk::vkBeginCommandBuffer(vk_command_buffer, &vk_command_buffer_begin_info));
                        }
                        unsafe {
                            vk::vkCmdBindDescriptorSets(
                                vk_command_buffer,
                                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                                vk_pipeline_layout,
                                0,
                                1,
                                &descriptor_set,
                                0,
                                std::ptr::null(),
                            );
                        }
                        unsafe {
                            vk::vkCmdBindDescriptorSets(
                                vk_command_buffer,
                                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                                vk_pipeline_layout,
                                0,
                                1,
                                &descriptor_set,
                                0,
                                std::ptr::null(),
                            );
                        }
                        {
                            let image_transition_barrier = vk::VkImageMemoryBarrier {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                                oldLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                                newLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                                srcQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                dstQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                image: vk_video_image,
                                subresourceRange: vk::VkImageSubresourceRange {
                                    aspectMask: vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT,
                                    baseMipLevel: 0,
                                    levelCount: 1,
                                    baseArrayLayer: 0,
                                    layerCount: 1,
                                },
                                srcAccessMask: vk::VkAccessFlagBits::VK_ACCESS_NONE,
                                dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT,
                                ..Default::default()
                            };
                            unsafe {
                                vk::vkCmdPipelineBarrier(
                                    vk_command_buffer,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT,
                                    vk::VkDependencyFlagBits(0),
                                    0,
                                    std::ptr::null(),
                                    0,
                                    std::ptr::null(),
                                    1,
                                    &image_transition_barrier,
                                );
                            }
                        }
                        if copy_new_buffer {
                            let region = vk::VkBufferImageCopy {
                                imageSubresource: vk::VkImageSubresourceLayers {
                                    aspectMask: vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT,
                                    layerCount: 1,
                                    ..Default::default()
                                },
                                imageExtent: vk::VkExtent3D {
                                    width: IMAGE_PIXEL_WIDTH,
                                    height: IMAGE_PIXEL_HEIGHT,
                                    depth: 1,
                                },
                                ..Default::default()
                            };
                            unsafe {
                                vk::vkCmdCopyBufferToImage(
                                    vk_command_buffer,
                                    vk_staging_buffers[staging_buffer_index_owned_by_vulkan as usize],
                                    vk_video_image,
                                    vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                                    1,
                                    &region,
                                );
                            }
                        }
                        unsafe {
                            vk::vkCmdBindPipeline(
                                vk_command_buffer,
                                vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                                vk_clean_buffer_pipeline,
                            );
                            vk::vkCmdDispatch(vk_command_buffer, 1, 1, 1);
                        }
                        {
                            let clean_to_hist_barrier = vk::VkBufferMemoryBarrier {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
                                srcAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT,
                                dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT | vk::VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT,
                                buffer: vk_hist_buffer,
                                size: vk::VK_WHOLE_SIZE,
                                ..Default::default()
                            };
                            unsafe {
                                vk::vkCmdPipelineBarrier(
                                    vk_command_buffer,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT, // Wait for Clean Compute
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT, // Before Hist Compute starts
                                    vk::VkDependencyFlagBits(0),
                                    0,
                                    std::ptr::null(),
                                    1,
                                    &clean_to_hist_barrier,
                                    0,
                                    std::ptr::null(),
                                );
                            }
                            unsafe {
                                vk::vkCmdBindPipeline(
                                    vk_command_buffer,
                                    vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                                    vk_hist_pipeline,
                                );
                                let workgroups_count = (IMAGE_PIXEL_WIDTH * IMAGE_PIXEL_HEIGHT) / (3 * 2 * 256);
                                vk::vkCmdDispatch(vk_command_buffer, workgroups_count, 1, 1);
                            }
                        }
                        {
                            let hist_to_cumulative_sum_barrier = vk::VkBufferMemoryBarrier {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
                                srcAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT,
                                dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT | vk::VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT,
                                srcQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                dstQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                buffer: vk_hist_buffer,
                                size: vk::VK_WHOLE_SIZE,
                                ..Default::default()
                            };
                            unsafe {
                                vk::vkCmdPipelineBarrier(
                                    vk_command_buffer,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
                                    vk::VkDependencyFlagBits(0),
                                    0,
                                    std::ptr::null(),
                                    1,
                                    &hist_to_cumulative_sum_barrier,
                                    0,
                                    std::ptr::null(),
                                );
                                vk::vkCmdBindPipeline(
                                    vk_command_buffer,
                                    vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                                    vk_cumulative_sum_pipeline,
                                );
                                vk::vkCmdDispatch(vk_command_buffer, 1, 1, 1);
                            }
                        }
                        {
                            let vk_image_memory_barrier = vk::VkImageMemoryBarrier {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                                oldLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                                newLayout: vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                                image: vk_video_image,
                                subresourceRange: vk::VkImageSubresourceRange {
                                    aspectMask: vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT,
                                    levelCount: 1,
                                    layerCount: 1,
                                    ..Default::default()
                                },
                                srcAccessMask: vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT,
                                dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT,
                                ..Default::default()
                            };

                            let cumulative_sum_to_graphics_barrier = vk::VkBufferMemoryBarrier {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
                                srcAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT,
                                dstAccessMask: vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT,
                                srcQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                dstQueueFamilyIndex: vk::VK_QUEUE_FAMILY_IGNORED,
                                buffer: vk_hist_buffer,
                                size: vk::VK_WHOLE_SIZE,
                                ..Default::default()
                            };

                            unsafe {
                                vk::vkCmdPipelineBarrier(
                                    vk_command_buffer,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT
                                        | vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
                                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
                                    vk::VkDependencyFlagBits(0),
                                    0,
                                    std::ptr::null(),
                                    1,
                                    &cumulative_sum_to_graphics_barrier,
                                    1,
                                    &vk_image_memory_barrier,
                                );
                            }
                        }
                        {
                            let vk_render_pass_begin_info = vk::VkRenderPassBeginInfo {
                                sType: vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
                                renderPass: vk_render_pass,
                                framebuffer: vk_framebuffers[image_index as usize],
                                renderArea: vk::VkRect2D {
                                    offset: vk::VkOffset2D { x: 0, y: 0 },
                                    extent: vk_swapchain_extent,
                                },
                                ..Default::default()
                            };
                            unsafe {
                                vk::vkCmdBeginRenderPass(
                                    vk_command_buffer,
                                    &vk_render_pass_begin_info,
                                    vk::VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE,
                                );

                                vk::vkCmdBindPipeline(
                                    vk_command_buffer,
                                    vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
                                    vk_graphics_pipeline,
                                );
                                vk::vkCmdDraw(vk_command_buffer, 6, 1, 0, 0);

                                vk::ImGui_ImplVulkan_RenderDrawData(imgui_draw_data, vk_command_buffer, std::ptr::null_mut());
                                vk::vkCmdEndRenderPass(vk_command_buffer);
                            }
                        }
                        unsafe {
                            vk::vkEndCommandBuffer(vk_command_buffer);
                        }
                    }

                    let vk_submit_info = vk::VkSubmitInfo {
                        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
                        waitSemaphoreCount: 1,
                        pWaitSemaphores: &vk_present_complete_semaphore,
                        pWaitDstStageMask: wait_stages.as_ptr(),
                        commandBufferCount: 1,
                        pCommandBuffers: &vk_command_buffer as *const _,
                        signalSemaphoreCount: 1,
                        pSignalSemaphores: &vk_render_complete_semaphore,
                        ..Default::default()
                    };

                    unsafe {
                        vk_assert!(vk::vkQueueSubmit(vk_queue, 1, &vk_submit_info, vk_fence));
                    }

                    vk_fence_fd = FenceFd::new(&epoll, vk_device, &vk_fence_get_fd_info_khr, get_fence_fd_khr);

                    let vk_present_info_khr = vk::VkPresentInfoKHR {
                        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
                        waitSemaphoreCount: 1,
                        pWaitSemaphores: &vk_render_complete_semaphore,
                        swapchainCount: 1,
                        pSwapchains: &vk_swapchain_khr,
                        pImageIndices: &image_index,
                        ..Default::default()
                    };

                    unsafe {
                        vk_result = vk::vkQueuePresentKHR(vk_queue, &vk_present_info_khr);
                    }

                    match vk_result {
                        vk::VkResult::VK_ERROR_OUT_OF_DATE_KHR | vk::VkResult::VK_SUBOPTIMAL_KHR => continue 'swapchain_loop,
                        other => vk_assert!(vk_result),
                    }
                    continue 'event_loop;
                }
            }
        }
    }
    unsafe {
        vk::vkDestroySwapchainKHR(vk_device, vk_swapchain_khr, std::ptr::null());
    }
}

// struct Xoroshiro128PP {
//     uint64_t s[2];

//     void seed(uint64_t x) {
//         s[0] = splitmix64(x);
//         s[1] = splitmix64(x);
//     }

//     static uint64_t splitmix64(uint64_t& state) {
//         uint64_t z = (state += 0x9E3779B97F4A7C15);
//         z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
//         z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
//         return z ^ (z >> 31);
//     }

//     static inline uint64_t rotl(uint64_t x, int k) {
//         return (x << k) | (x >> (64 - k));
//     }

//     uint64_t next() {
//         const uint64_t s0 = s[0];
//         uint64_t s1 = s[1];
//         const uint64_t result = rotl(s0 + s1, 17) + s0;
//         s1 ^= s0;
//         s[0] = rotl(s0, 49) ^ s1 ^ (s1 << 21);
//         s[1] = rotl(s1, 28);
//         return result;
//     }

//     double next_double() {
//         return (next() >> 11) * (1.0 / 9007199254740992.0);
//     }
// };

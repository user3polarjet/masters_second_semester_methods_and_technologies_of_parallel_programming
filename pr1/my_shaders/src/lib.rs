#![no_std]
#![feature(asm_experimental_arch)]
// use descriptor_gen::shader_template;
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[spirv(compute(threads(256)))]
pub fn clean_buffer(
    #[spirv(global_invocation_id)] id: spirv_std::glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_counts: &mut [u32; 256],
) {
    let tid = id.x as usize;
    // Since threads(256) is fixed, we simply clear the index
    // corresponding to the invocation ID.
    if tid < 256 {
        global_counts[tid] = 0;
    }
}


// pub mod imx462;
// pub mod oct640m;
// pub mod player;

// #[shader_template(orig_w, orig_h, target_w, target_h)]
// #[spirv(vertex)]
// pub fn main_image_vs(
//     #[spirv(vertex_index)] vert_id: i32,
//     #[spirv(position)] out_pos: &mut spirv_std::glam::Vec4,
//     #[spirv(location = 0)] out_tex_coord: &mut spirv_std::glam::Vec2,
//     #[spirv(push_constant)] constants: &PushConstants,
// ) {
//     // These become standard Rust constants.
//     // The macro replaces the idents with the literals you provide.
//     const AO: f32 = orig_w as f32 / orig_h as f32;
//     const AT: f32 = target_w as f32 / target_h as f32;

//     // Standard square quad: x,y in [-1, 1], u,v in [0, 1]
//     let positions: [spirv_std::glam::Vec4; 6] = [
//         spirv_std::glam::Vec4::new(-1.0, -1.0, 0.0, 0.0),
//         spirv_std::glam::Vec4::new(1.0, -1.0, 1.0, 0.0),
//         spirv_std::glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
//         spirv_std::glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
//         spirv_std::glam::Vec4::new(-1.0, 1.0, 0.0, 1.0),
//         spirv_std::glam::Vec4::new(-1.0, -1.0, 0.0, 0.0),
//     ];

//     let pos = positions[vert_id as usize];

//     // --- 1. GEOMETRY TRANSFORMS ---

//     // A. Scale standard quad to match the Original Image's physical shape
//     let base_x = pos.x * AO;
//     let base_y = pos.y;

//     // B. Apply Zoom
//     let zoomed_x = base_x * constants.zoom;
//     let zoomed_y = base_y * constants.zoom;

//     // C. Apply Rotation
//     let cos_a = constants.angle_cos;
//     let sin_a = constants.angle_sin;

//     let rot_x = zoomed_x * cos_a - zoomed_y * sin_a;
//     let rot_y = zoomed_x * sin_a + zoomed_y * cos_a;

//     // D. Map to NDC (Pre-squish X by dividing by Target Aspect Ratio)
//     let final_x = rot_x / AT;
//     let final_y = rot_y;

//     // Output the physically rotated and scaled quad
//     *out_pos = spirv_std::glam::Vec4::new(final_x, final_y, 0.0, 1.0);

//     // --- 2. TEXTURE COORDS ---
//     *out_tex_coord = spirv_std::glam::Vec2::new(pos.z, pos.w);
// }

// #[shader_template(width, height)]
// #[spirv(vertex)]
// pub fn pipa_vs(
//     #[spirv(vertex_index)] vert_id: i32,
//     #[spirv(position)] out_pos: &mut spirv_std::glam::Vec4,
//     #[spirv(location = 0)] out_tex_coord: &mut spirv_std::glam::Vec2,
// ) {
//     // Constants calculated at compile-time via template substitution
//     const PERCENTAGE: f32 = 0.3;
//     const P_WIDTH_PX: f32 = width as f32 * PERCENTAGE;
//     const P_HEIGHT_PX: f32 = height as f32 * PERCENTAGE;

//     // NDC scale math (Division by half-resolution)
//     const P_W: f32 = P_WIDTH_PX / ((width / 2) as f32);
//     const P_H: f32 = P_HEIGHT_PX / ((height / 2) as f32);

//     // Anchor Point: Top Right (1.0, -1.0)
//     const RIGHT: f32 = 1.0;
//     const TOP: f32 = -1.0;
//     const LEFT: f32 = RIGHT - P_W;
//     const BOTTOM: f32 = TOP + P_H;

//     // Position and TexCoord data: [x, y, u, v]
//     const POSITIONS: [spirv_std::glam::Vec4; 6] = [
//         spirv_std::glam::Vec4::new(LEFT, BOTTOM, 0.0, 0.0),
//         spirv_std::glam::Vec4::new(RIGHT, TOP, 1.0, 1.0),
//         spirv_std::glam::Vec4::new(RIGHT, BOTTOM, 1.0, 0.0),
//         spirv_std::glam::Vec4::new(LEFT, BOTTOM, 0.0, 0.0),
//         spirv_std::glam::Vec4::new(LEFT, TOP, 0.0, 1.0),
//         spirv_std::glam::Vec4::new(RIGHT, TOP, 1.0, 1.0),
//     ];

//     let pos = POSITIONS[vert_id as usize];
//     *out_pos = spirv_std::glam::Vec4::new(pos.x, pos.y, 0.0, 1.0);
//     *out_tex_coord = spirv_std::glam::Vec2::new(pos.z, pos.w);
// }

// #[repr(C)]
// #[derive(Copy, Clone, Default)]
// pub struct PushConstants {
//     // pub diamond_x: f32,
//     // pub diamond_y: f32,

//     // pub target_x: f32,
//     // pub target_y: f32,
//     // pub target_extent_x: f32,
//     // pub target_extent_y: f32,
//     pub angle_cos: f32,
//     pub angle_sin: f32,
//     pub zoom: f32,
// }

// #[shader_template(hist_buffer_binding)]
// #[spirv(compute(threads(256)))]
// pub fn clean_buffer(
//     #[spirv(global_invocation_id)] id: spirv_std::glam::UVec3,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = hist_buffer_binding)] global_counts: &mut [u32; 256],
// ) {
//     let tid = id.x as usize;
//     // Since threads(256) is fixed, we simply clear the index
//     // corresponding to the invocation ID.
//     if tid < 256 {
//         global_counts[tid] = 0;
//     }
// }

// #[shader_template(thermal_image_sampler_binding, hist_buffer_binding, width, height)]
// #[spirv(fragment)]
// pub fn thermal_fs(
//     #[spirv(location = 0)] in_tex_coord: spirv_std::glam::Vec2,
//     #[spirv(descriptor_set = 0, binding = thermal_image_sampler_binding)] tex_sampler: &spirv_std::image::SampledImage<spirv_std::image::Image2d>,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = hist_buffer_binding)] cumulative_counts: &[u32; 256],
//     out_color: &mut spirv_std::glam::Vec4,
// ) {
//     let pixel: spirv_std::glam::Vec4 = tex_sampler.sample(in_tex_coord);
//     let absolute_color = pixel.x * 65535.0;
//     let bin = (absolute_color as u32).clamp(0, 65535) >> 8;
//     let color_scaled_inside_bin = (absolute_color - (bin as f32) * 256.0) / 256.0;
//     let cdf_min = if bin == 0 { 0.0 } else { cumulative_counts[(bin - 1) as usize] as f32 };
//     let cdf_max = cumulative_counts[bin as usize] as f32;

//     let total_pixels = (width * height) as f32;
//     let cdf_normalized_x = cdf_min / total_pixels;
//     let cdf_normalized_y = cdf_max / total_pixels;

//     let y = cdf_normalized_x * (1.0 - color_scaled_inside_bin) + cdf_normalized_y * color_scaled_inside_bin;
//     *out_color = spirv_std::glam::Vec4::new(y, y, y, 1.0);
// }

// #[shader_template(color_image_sampler_binding)]
// #[spirv(fragment)]
// pub fn color_fs(
//     #[spirv(location = 0)] in_tex_coord: spirv_std::glam::Vec2,
//     #[spirv(descriptor_set = 0, binding = color_image_sampler_binding)] tex_sampler: &spirv_std::image::SampledImage<spirv_std::image::Image2d>,
//     out_color: &mut spirv_std::glam::Vec4,
// ) {
//     let sampled = tex_sampler.sample(in_tex_coord);
//     *out_color = sampled;
//     out_color.w = 1.0;
// }

// #[shader_template(thermal_image_buffer_binding, hist_buffer_binding, width, height, elements_per_thread, pixels_per_element)]
// #[spirv(compute(threads(256)))]
// pub fn calculate_histogram(
//     #[spirv(workgroup_id)] workgroup_id: spirv_std::glam::UVec3,
//     #[spirv(local_invocation_id)] local_id: spirv_std::glam::UVec3,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = thermal_image_buffer_binding)] image_src: &[u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = hist_buffer_binding)] global_counts: &mut [u32; 256],
//     #[spirv(workgroup)] local_counts: &mut [u32; 256],
// ) {
//     let tid = local_id.x as usize;

//     // Initialize local histogram in workgroup memory
//     local_counts[tid] = 0;
//     spirv_std::arch::workgroup_memory_barrier_with_group_sync();

//     // The attribute macro replaces these identifiers with the passed literals
//     let items_per_group = 256 * elements_per_thread;
//     let base_index = (workgroup_id.x * items_per_group) + (local_id.x * elements_per_thread);
//     let max_index = (width * height) / pixels_per_element;

//     let bits_per_pixel = 32 / pixels_per_element;
//     let shift_offset = bits_per_pixel - 8;

//     for i in 0..elements_per_thread {
//         let current_index = base_index + i;

//         if current_index < max_index {
//             let bixel = image_src[current_index as usize];

//             for p in 0..pixels_per_element {
//                 let shift = (p * bits_per_pixel) + shift_offset;
//                 let bin = ((bixel >> shift) & 0xFF) as usize;

//                 unsafe {
//                     spirv_std::arch::atomic_i_add::<u32, { spirv_std::memory::Scope::Workgroup as u32 }, 0>(&mut local_counts[bin], 1);
//                 }
//             }
//         }
//     }

//     // Synchronize and aggregate results to global memory
//     unsafe {
//         spirv_std::arch::workgroup_memory_barrier_with_group_sync();
//         spirv_std::arch::atomic_i_add::<u32, { spirv_std::memory::Scope::QueueFamily as u32 }, 0>(&mut global_counts[tid], local_counts[tid]);
//     }
// }

// #[shader_template(hist_buffer_binding)]
// #[spirv(compute(threads(256)))]
// pub fn calculate_cumulative_sum(
//     #[spirv(local_invocation_id)] local_id: spirv_std::glam::UVec3,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = hist_buffer_binding)] global_counts: &mut [u32; 256],
//     #[spirv(workgroup)] ping: &mut [u32; 256],
//     #[spirv(workgroup)] pong: &mut [u32; 256],
// ) {
//     let tid = local_id.x as usize;

//     // 1. Load initial data into Workgroup memory
//     ping[tid] = global_counts[tid];

//     spirv_std::arch::workgroup_memory_barrier_with_group_sync();

//     let mut offset = 1;
//     let mut use_ping = true;

//     // 2. Hillis-Steele parallel prefix sum (8 steps for 256 elements)
//     for _ in 0..8 {
//         let mut val = 0;
//         if tid >= offset {
//             if use_ping {
//                 val = ping[tid - offset];
//             } else {
//                 val = pong[tid - offset];
//             }
//         }

//         if use_ping {
//             pong[tid] = ping[tid] + val;
//         } else {
//             ping[tid] = pong[tid] + val;
//         }

//         use_ping = !use_ping;
//         offset *= 2;

//         unsafe {
//             spirv_std::arch::workgroup_memory_barrier_with_group_sync();
//         }
//     }

//     // 3. Final result is in ping because loop count is even (8 iterations)
//     global_counts[tid] = ping[tid];
// }

// #[inline(always)]
// fn bayer_read_pair(
//     in_bayer: &spirv_std::image::Image!(buffer, format = r32ui, sampled = false),
//     base_row_word: usize,
//     col_word: usize,
//     shift: usize,
// ) -> (u32, u32) {
//     let idx = (base_row_word + col_word) as i32;

//     let w0 = in_bayer.read(idx);
//     let w1 = in_bayer.read(idx + 1);

//     let lower = w0 >> shift;
//     let upper = if shift > 0 { w1 << (32 - shift) } else { 0 };
//     let combined = (lower | upper) & 0x00FFFFFF;

//     let b0 = combined & 0xFF;
//     let b1 = (combined >> 8) & 0xFF;
//     let b2 = (combined >> 16) & 0xFF;

//     let p0 = (b0 << 4) | (b2 & 0x0F);
//     let p1 = (b1 << 4) | (b2 >> 4);

//     (p0, p1)
// }

// #[shader_template(color_image_texel_buffer_binding, color_image_binding, width, height)]
// #[spirv(compute(threads(16, 16)))]
// pub fn bayer_convert_bilinear(
//     #[spirv(global_invocation_id)] id: spirv_std::glam::UVec3,
//     #[spirv(descriptor_set = 0, binding = color_image_texel_buffer_binding)] in_bayer: &spirv_std::image::Image!(
//         buffer,
//         format = r32ui,
//         sampled = false
//     ),
//     #[spirv(descriptor_set = 0, binding = color_image_binding)] out_image: &spirv_std::image::Image!(2D, format = rgba8, sampled = false),
// ) {
//     let id_x = id.x as usize;
//     let id_y = id.y as usize;

//     // Standard Rust constants. The attribute macro will replace
//     // 'width' with the literal value provided in the decl! call.
//     const ROW_WORDS: usize = (width * 3) / 8;

//     let y_m1 = if id_y > 0 { (id_y << 1) - 1 } else { 0 };
//     let y_0 = id_y << 1;
//     let y_1 = y_0 + 1;
//     let y_2 = y_0 + 2;

//     let r_m1 = y_m1 * ROW_WORDS;
//     let r_0 = y_0 * ROW_WORDS;
//     let r_1 = y_1 * ROW_WORDS;
//     let r_2 = y_2 * ROW_WORDS;

//     let c_byte = id_x * 3;
//     let c_word = c_byte >> 2;
//     let c_shift = (c_byte & 3) << 3;

//     let l_byte = if id_x > 0 { (id_x - 1) * 3 } else { 0 };
//     let l_word = l_byte >> 2;
//     let l_shift = (l_byte & 3) << 3;

//     let r_byte = (id_x + 1) * 3;
//     let r_word = r_byte >> 2;
//     let r_shift = (r_byte & 3) << 3;

//     // Neighbors for Bilinear Interpolation
//     let (p0m1, p1m1) = bayer_read_pair(in_bayer, r_m1, c_word, c_shift);
//     let (p00, p10) = bayer_read_pair(in_bayer, r_0, c_word, c_shift);
//     let (p01, p11) = bayer_read_pair(in_bayer, r_1, c_word, c_shift);
//     let (p02, p12) = bayer_read_pair(in_bayer, r_2, c_word, c_shift);

//     let pm1m1 = if id_x > 0 {
//         bayer_read_pair(in_bayer, r_m1, l_word, l_shift).1
//     } else {
//         p0m1
//     };
//     let pm10 = if id_x > 0 {
//         bayer_read_pair(in_bayer, r_0, l_word, l_shift).1
//     } else {
//         p00
//     };
//     let pm11 = if id_x > 0 {
//         bayer_read_pair(in_bayer, r_1, l_word, l_shift).1
//     } else {
//         p01
//     };

//     let p20 = bayer_read_pair(in_bayer, r_0, r_word, r_shift).0;
//     let p21 = bayer_read_pair(in_bayer, r_1, r_word, r_shift).0;
//     let p22 = bayer_read_pair(in_bayer, r_2, r_word, r_shift).0;

//     // Bilinear Demosaicing (RGGB pattern)
//     let c00_r = p00;
//     let c00_g = (p0m1 + p01 + pm10 + p10) >> 2;
//     let c00_b = (pm1m1 + p1m1 + pm11 + p11) >> 2;

//     let c10_r = (p00 + p20) >> 1;
//     let c10_g = p10;
//     let c10_b = (p1m1 + p11) >> 1;

//     let c01_r = (p00 + p02) >> 1;
//     let c01_g = p01;
//     let c01_b = (pm11 + p11) >> 1;

//     let c11_r = (p00 + p20 + p02 + p22) >> 2;
//     let c11_g = (p10 + p12 + p01 + p21) >> 2;
//     let c11_b = p11;

//     let scale = 1.0 / 4095.0;

//     let out00 = spirv_std::glam::Vec4::new(c00_r as f32 * scale, c00_g as f32 * scale, c00_b as f32 * scale, 1.0);
//     let out10 = spirv_std::glam::Vec4::new(c10_r as f32 * scale, c10_g as f32 * scale, c10_b as f32 * scale, 1.0);
//     let out01 = spirv_std::glam::Vec4::new(c01_r as f32 * scale, c01_g as f32 * scale, c01_b as f32 * scale, 1.0);
//     let out11 = spirv_std::glam::Vec4::new(c11_r as f32 * scale, c11_g as f32 * scale, c11_b as f32 * scale, 1.0);

//     let x0_u32 = id.x << 1;
//     let y0_u32 = id.y << 1;

//     unsafe {
//         out_image.write(spirv_std::glam::UVec2::new(x0_u32, y0_u32), out00);
//         out_image.write(spirv_std::glam::UVec2::new(x0_u32 + 1, y0_u32), out10);
//         out_image.write(spirv_std::glam::UVec2::new(x0_u32, y0_u32 + 1), out01);
//         out_image.write(spirv_std::glam::UVec2::new(x0_u32 + 1, y0_u32 + 1), out11);
//     }
// }

// pub const COLOR_IMAGE_TEXEL_BUFFER_BINDING: u32 = 0;
// pub const COLOR_IMAGE_STORAGE_BINDING: u32 = 1;
// pub const COLOR_IMAGE_SAMPLER_BINDING: u32 = 2;
// pub const THERMAL_IMAGE_SAMPLER_BINDING: u32 = 3;
// pub const THERMAL_IMAGE_BUFFER_BINDING: u32 = 4;
// pub const HIST_BUFFER_BINDING: u32 = 5;

// calculate_histogram_decl!(thermal_image_buffer_binding: 4, hist_buffer_binding: 5, width: 640, height: 480, elements_per_thread: 3, pixels_per_element: 2);
// calculate_histogram_decl!(thermal_image_buffer_binding: 4, hist_buffer_binding: 5, width: 1280, height: 720, elements_per_thread: 3, pixels_per_element: 4);
// calculate_cumulative_sum_decl!(hist_buffer_binding: 5);
// clean_buffer_decl!(hist_buffer_binding: 5);
// bayer_convert_bilinear_decl!(color_image_texel_buffer_binding: 0, color_image_binding: 1, width: 1920, height: 1080);
// main_image_vs_decl!(orig_w: 1920, orig_h: 1080, target_w: 640, target_h: 480);
// main_image_vs_decl!(orig_w: 1920, orig_h: 1080, target_w: 800, target_h: 600);
// main_image_vs_decl!(orig_w: 1920, orig_h: 1080, target_w: 1280, target_h: 720);
// main_image_vs_decl!(orig_w: 640, orig_h: 480, target_w: 640, target_h: 480);
// main_image_vs_decl!(orig_w: 640, orig_h: 480, target_w: 640, target_h: 360);
// pipa_vs_decl!(width: 640, height: 480);
// pipa_vs_decl!(width: 1280, height: 720);
// pipa_vs_decl!(width: 1920, height: 1080);
// color_fs_decl!(color_image_sampler_binding: 2);
// thermal_fs_decl!(thermal_image_sampler_binding: 3, hist_buffer_binding: 5, width: 640, height: 480);
// thermal_fs_decl!(thermal_image_sampler_binding: 3, hist_buffer_binding: 5, width: 1280, height: 720);

// const fn vec2_array_map_sub<const N: usize>(array: [spirv_std::glam::Vec2; N], point: spirv_std::glam::Vec2) -> [spirv_std::glam::Vec2; N] {
//     let mut res = [spirv_std::glam::Vec2 { x: 0.0, y: 0.0 }; N];
//     let mut i = 0usize;
//     while i < N {
//         res[i] = spirv_std::glam::Vec2 {
//             x: array[i].x - point.x,
//             y: array[i].y - point.y,
//         };
//         i += 1;
//     }
//     res
// }

// const fn vec2_sub(lhs: spirv_std::glam::Vec2, rhs: spirv_std::glam::Vec2) -> spirv_std::glam::Vec2 {
//     spirv_std::glam::Vec2 {
//         x: lhs.x - rhs.x,
//         y: lhs.y - rhs.y,
//     }
// }
// const fn vec2_add(lhs: spirv_std::glam::Vec2, rhs: spirv_std::glam::Vec2) -> spirv_std::glam::Vec2 {
//     spirv_std::glam::Vec2 {
//         x: lhs.x + rhs.x,
//         y: lhs.y + rhs.y,
//     }
// }
// const fn const_slice<const FROM: usize, const TO: usize, const TARGET_SIZE: usize, const N: usize>(
//     arr: [spirv_std::glam::Vec2; N],
// ) -> [spirv_std::glam::Vec2; TARGET_SIZE] {
//     let mut res = [spirv_std::glam::Vec2::ZERO; TARGET_SIZE];
//     let mut i = FROM;
//     let mut j = 0usize;

//     while i < TO && j < TARGET_SIZE {
//         res[j] = arr[i];
//         i += 1;
//         j += 1;
//     }
//     res
// }

// const PIPA_SHIFT: f32 = 0.2;
// const PIPA_HOR_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: PIPA_SHIFT * PIPA_WIDTH,
//     y: 0.0,
// };
// const PIPA_VERT_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: 0.0,
//     y: PIPA_SHIFT * PIPA_HEIGHT,
// };
// pub const PIPA_LINE_POINTS: [spirv_std::glam::Vec2; 16] = vec2_array_map_sub(
//     [
//         PIPA_LEFT_TOP,
//         vec2_add(PIPA_LEFT_TOP, PIPA_HOR_SHIFT),
//         PIPA_LEFT_TOP,
//         vec2_add(PIPA_LEFT_TOP, PIPA_VERT_SHIFT),
//         PIPA_RIGHT_TOP,
//         vec2_sub(PIPA_RIGHT_TOP, PIPA_HOR_SHIFT),
//         PIPA_RIGHT_TOP,
//         vec2_add(PIPA_RIGHT_TOP, PIPA_VERT_SHIFT),
//         PIPA_RIGHT_BOTTOM,
//         vec2_sub(PIPA_RIGHT_BOTTOM, PIPA_HOR_SHIFT),
//         PIPA_RIGHT_BOTTOM,
//         vec2_sub(PIPA_RIGHT_BOTTOM, PIPA_VERT_SHIFT),
//         PIPA_LEFT_BOTTOM,
//         vec2_add(PIPA_LEFT_BOTTOM, PIPA_HOR_SHIFT),
//         PIPA_LEFT_BOTTOM,
//         vec2_sub(PIPA_LEFT_BOTTOM, PIPA_VERT_SHIFT),
//     ],
//     PIPA_CENTER,
// );

// const IMAGE_CENTER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: (IMAGE_WIDTH / 2) as f32,
//     y: (IMAGE_HEIGHT / 2) as f32,
// };

// const DIAMOND_PIXEL_WIDTH: f32 = 10f32;
// const DIAMOND_SHIFT: f32 = 7f32;
// const DIAMOND_HOR_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: DIAMOND_SHIFT, y: 0.0 };
// const DIAMOND_VERT_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: 0.0, y: DIAMOND_SHIFT };
// const DIAMONT_RIGHT_TOP: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: DIAMOND_PIXEL_WIDTH,
//     y: -DIAMOND_PIXEL_WIDTH,
// };
// const DIAMONT_RIGHT_BOTTOM: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: DIAMOND_PIXEL_WIDTH,
//     y: DIAMOND_PIXEL_WIDTH,
// };
// const DIAMONT_LEFT_BOTTOM: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: -DIAMOND_PIXEL_WIDTH,
//     y: DIAMOND_PIXEL_WIDTH,
// };
// const DIAMONT_LEFT_TOP: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: -DIAMOND_PIXEL_WIDTH,
//     y: -DIAMOND_PIXEL_WIDTH,
// };

// const DIAMOND_LINE_POINTS: [spirv_std::glam::Vec2; 8] = vec2_array_map_scale_aspect_ratio([
//     vec2_sub(DIAMONT_RIGHT_TOP, DIAMOND_HOR_SHIFT),
//     vec2_add(DIAMONT_RIGHT_TOP, DIAMOND_VERT_SHIFT),
//     vec2_sub(DIAMONT_RIGHT_BOTTOM, DIAMOND_HOR_SHIFT),
//     vec2_sub(DIAMONT_RIGHT_BOTTOM, DIAMOND_VERT_SHIFT),
//     vec2_add(DIAMONT_LEFT_BOTTOM, DIAMOND_HOR_SHIFT),
//     vec2_sub(DIAMONT_LEFT_BOTTOM, DIAMOND_VERT_SHIFT),
//     vec2_add(DIAMONT_LEFT_TOP, DIAMOND_HOR_SHIFT),
//     vec2_add(DIAMONT_LEFT_TOP, DIAMOND_VERT_SHIFT),
// ]);

// const CROSS_PIXEL_SHIFT: f32 = 3f32;
// const CROSS_PIXEL_SIZE: f32 = 10f32;

// const CROSS_HOR_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: CROSS_PIXEL_SHIFT,
//     y: 0.0,
// };
// const CROSS_VERT_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: 0.0,
//     y: CROSS_PIXEL_SHIFT,
// };

// const CROSS_TOP_OUTER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: 0.0,
//     y: -CROSS_PIXEL_SIZE,
// };
// const CROSS_BOTTOM_OUTER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: 0.0, y: CROSS_PIXEL_SIZE };
// const CROSS_LEFT_OUTER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: -CROSS_PIXEL_SIZE,
//     y: 0.0,
// };
// const CROSS_RIGHT_OUTER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: CROSS_PIXEL_SIZE, y: 0.0 };

// const CROSS_TOP_INNER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: 0.0,
//     y: -CROSS_PIXEL_SHIFT,
// };
// const CROSS_BOTTOM_INNER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: 0.0,
//     y: CROSS_PIXEL_SHIFT,
// };
// const CROSS_LEFT_INNER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: -CROSS_PIXEL_SHIFT,
//     y: 0.0,
// };
// const CROSS_RIGHT_INNER: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 {
//     x: CROSS_PIXEL_SHIFT,
//     y: 0.0,
// };

// const CROSS_LINE_POINTS: [spirv_std::glam::Vec2; 8] = vec2_array_map_scale_aspect_ratio([
//     CROSS_TOP_OUTER,
//     CROSS_TOP_INNER,
//     CROSS_BOTTOM_OUTER,
//     CROSS_BOTTOM_INNER,
//     CROSS_LEFT_OUTER,
//     CROSS_LEFT_INNER,
//     CROSS_RIGHT_OUTER,
//     CROSS_RIGHT_INNER,
// ]);

// const fn vec2_array_map_scale_aspect_ratio<const N: usize>(array: [spirv_std::glam::Vec2; N]) -> [spirv_std::glam::Vec2; N] {
//     let mut res = [spirv_std::glam::Vec2 { x: 0.0, y: 0.0 }; N];
//     let mut i = 0usize;
//     while i < N {
//         res[i] = spirv_std::glam::Vec2 {
//             x: array[i].x / ((IMAGE_WIDTH / 2) as f32),
//             y: array[i].y / ((IMAGE_HEIGHT / 2) as f32),
//         };
//         i += 1;
//     }
//     res
// }

// const TARGET_SHIFT: f32 = 0.4;
// const TARGET_HOR_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: TARGET_SHIFT, y: 0.0 };
// const TARGET_VERT_SHIFT: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: 0.0, y: TARGET_SHIFT };
// const TARGET_RECT_LEFT_TOP: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: -1.0, y: -1.0 };
// const TARGET_RECT_LEFT_BOTTOM: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: -1.0, y: 1.0 };
// const TARGET_RECT_RIGHT_BOTTOM: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: 1.0, y: 1.0 };
// const TARGET_RECT_RIGHT_TOP: spirv_std::glam::Vec2 = spirv_std::glam::Vec2 { x: 1.0, y: -1.0 };

// pub const TARGET_RECT_POINTS: [spirv_std::glam::Vec2; 16] = [
//     TARGET_RECT_LEFT_TOP,
//     vec2_add(TARGET_RECT_LEFT_TOP, TARGET_HOR_SHIFT),
//     TARGET_RECT_LEFT_TOP,
//     vec2_add(TARGET_RECT_LEFT_TOP, TARGET_VERT_SHIFT),
//     TARGET_RECT_RIGHT_TOP,
//     vec2_sub(TARGET_RECT_RIGHT_TOP, TARGET_HOR_SHIFT),
//     TARGET_RECT_RIGHT_TOP,
//     vec2_add(TARGET_RECT_RIGHT_TOP, TARGET_VERT_SHIFT),
//     TARGET_RECT_RIGHT_BOTTOM,
//     vec2_sub(TARGET_RECT_RIGHT_BOTTOM, TARGET_HOR_SHIFT),
//     TARGET_RECT_RIGHT_BOTTOM,
//     vec2_sub(TARGET_RECT_RIGHT_BOTTOM, TARGET_VERT_SHIFT),
//     TARGET_RECT_LEFT_BOTTOM,
//     vec2_add(TARGET_RECT_LEFT_BOTTOM, TARGET_HOR_SHIFT),
//     TARGET_RECT_LEFT_BOTTOM,
//     vec2_sub(TARGET_RECT_LEFT_BOTTOM, TARGET_VERT_SHIFT),
// ];

// pub const LINES: [[spirv_std::glam::Vec2; 8]; 7] = [
//     DIAMOND_LINE_POINTS,
//     CROSS_LINE_POINTS,
//     PIPA_BOUND_RECT_POINTS,
//     const_slice::<0, 8, 8, 16>(TARGET_RECT_POINTS),
//     const_slice::<8, 16, 8, 16>(TARGET_RECT_POINTS),
//     const_slice::<0, 8, 8, 16>(PIPA_LINE_POINTS),
//     const_slice::<8, 16, 8, 16>(PIPA_LINE_POINTS),
// ];

// #[spirv(vertex)]
// pub fn lines_vs(
//     #[spirv(vertex_index)] vert_id: i32,
//     #[spirv(instance_index)] instance_id: i32,
//     #[spirv(position)] out_pos: &mut spirv_std::glam::Vec4,
//     #[spirv(push_constant)] constants: &PushConstants,
// ) {
//     let pos = match instance_id {
//         0 => LINES[0][vert_id as usize] + (spirv_std::glam::Vec2::new(constants.diamond_x, constants.diamond_y) - IMAGE_CENTER) / IMAGE_CENTER,
//         1 => LINES[1][vert_id as usize] + (spirv_std::glam::Vec2::new(constants.target_x, constants.target_y) - IMAGE_CENTER) / IMAGE_CENTER,
//         2 => LINES[2][vert_id as usize],
//         3 => {
//             ((LINES[3][vert_id as usize] * spirv_std::glam::Vec2::new(constants.target_extent_x, constants.target_extent_y)
//                 + spirv_std::glam::Vec2::new(constants.target_x, constants.target_y))
//                 - IMAGE_CENTER)
//                 / IMAGE_CENTER
//         }
//         4 => {
//             ((LINES[4][vert_id as usize] * spirv_std::glam::Vec2::new(constants.target_extent_x, constants.target_extent_y)
//                 + spirv_std::glam::Vec2::new(constants.target_x, constants.target_y))
//                 - IMAGE_CENTER)
//                 / IMAGE_CENTER
//         }
//         // DIVIDE by zoom here so the PiP brackets shrink as you zoom in
//         _ => PIPA_CENTER + LINES[instance_id as usize][vert_id as usize] / spirv_std::glam::Vec2::new(constants.zoom, constants.zoom),
//     };
//     *out_pos = spirv_std::glam::Vec4::new(pos.x, pos.y, 0.0, 1.0);
// }

// #[spirv(fragment)]
// pub fn white_color_fs(out_color: &mut spirv_std::glam::Vec4) {
//     *out_color = spirv_std::glam::Vec4::new(1f32, 1f32, 1f32, 1f32);
// }

// #[spirv(fragment)]
// pub fn black_color_fs(out_color: &mut spirv_std::glam::Vec4) {
//     *out_color = spirv_std::glam::Vec4::new(0f32, 0f32, 0f32, 1f32);
// }

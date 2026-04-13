#![no_std]

use spirv_std::glam::{UVec3, Vec2, Vec4};
use spirv_std::spirv;

#[derive(Clone, Copy)]
pub struct Xorshift32 {
    pub state: u32,
}

impl Xorshift32 {
    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos_x: u32,
    pub pos_y: u32,
    pub rng: Xorshift32,
    pub _padding: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PushConstants {
    pub grid_width: u32,
    pub grid_height: u32,
    pub num_particles: u32,
    pub max_density: u32,
}

// --- COMPUTE 1: CLEAN PASS ---
#[spirv(compute(threads(256)))]
pub fn clean_buffer(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] grid: &mut [u32],
    #[spirv(push_constant)] constants: &PushConstants,
) {
    let idx = id.x as usize;
    if idx < (constants.grid_width * constants.grid_height) as usize {
        grid[idx] = 0;
    }
}

#[spirv(compute(threads(256)))]
pub fn brownian_step(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] particles: &mut [Particle],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] grid: &mut [u32],
    #[spirv(push_constant)] constants: &PushConstants,
) {
    let idx = id.x as usize;
    if idx >= constants.num_particles as usize {
        return;
    }

    let mut p = particles[idx];

    let dir = p.rng.next() >> 30;

    let mut x = p.pos_x as i32;
    let mut y = p.pos_y as i32;

    match dir {
        0 => y -= 1,
        1 => x += 1,
        2 => y += 1,
        _ => x -= 1,
    }

    let max_x = (constants.grid_width - 1) as i32;
    let max_y = (constants.grid_height - 1) as i32;

    if x < 0 {
        x = 1;
    }
    if x > max_x {
        x = max_x - 1;
    }
    if y < 0 {
        y = 1;
    }
    if y > max_y {
        y = max_y - 1;
    }

    p.pos_x = x as u32;
    p.pos_y = y as u32;
    particles[idx] = p;

    let grid_idx = (p.pos_y * constants.grid_width + p.pos_x) as usize;
    unsafe {
        spirv_std::arch::atomic_i_add::<u32, 1, 0>(&mut grid[grid_idx], 1);
    }
}

#[spirv(vertex)]
pub fn fullscreen_vs(#[spirv(vertex_index)] vert_id: i32, #[spirv(position)] out_pos: &mut Vec4, #[spirv(location = 0)] out_uv: &mut Vec2) {
    const POSITIONS: [Vec4; 6] = [
        Vec4::new(-1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, -1.0, 1.0, 0.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Vec4::new(-1.0, 1.0, 0.0, 1.0),
        Vec4::new(-1.0, -1.0, 0.0, 0.0),
        Vec4::new(1.0, -1.0, 1.0, 0.0),
    ];
    let pos = POSITIONS[vert_id as usize];
    *out_pos = Vec4::new(pos.x, pos.y, 0.0, 1.0);
    *out_uv = Vec2::new(pos.z, pos.w);
}

#[spirv(fragment)]
pub fn grid_fs(
    #[spirv(location = 0)] in_uv: Vec2,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] grid: &[u32],
    #[spirv(push_constant)] constants: &PushConstants,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let x = ((in_uv.x * constants.grid_width as f32) as u32).min(constants.grid_width - 1);
    let y = ((in_uv.y * constants.grid_height as f32) as u32).min(constants.grid_height - 1);

    let grid_idx = (y * constants.grid_width + x) as usize;
    let density = grid[grid_idx];
    let intensity = (density as f32 / constants.max_density as f32).clamp(0.0, 1.0);

    *out_color = Vec4::new(intensity, 0.0, 0.0, 1.0);
}

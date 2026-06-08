use fastnoise_lite::FastNoiseLite;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use wgpu::{
    Buffer, BufferUsages, Device, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    renderer::vertex::Vertex,
    terrain::{CHUNK_SIZE, TEXTURE_SCALE},
};

pub(crate) struct Chunk {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Chunk {
    pub fn new(device: &Device, noises: &[(FastNoiseLite, f32)], position: (i32, i32)) -> Self {
        let height_map = create_heightmap(noises, position);
        let vertices = get_vertices(position, &height_map);
        let indices = get_indices();
        let (vertex_buffer, index_buffer) = create_mesh(device, &vertices, &indices);

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(CHUNK_SIZE.pow(2) * 6) as u32, 0, 0..1);
    }
}

fn create_heightmap(noises: &[(FastNoiseLite, f32)], position: (i32, i32)) -> Vec<f32> {
    let size = CHUNK_SIZE + 1;
    let mut height_map = vec![0.0f32; size * size];

    height_map.par_iter_mut().enumerate().for_each(|(i, tile)| {
        let x = i / size;
        let z = i % size;
        let sample_x = (position.0 as f32) * (CHUNK_SIZE as f32) + (x as f32);
        let sample_z = (position.1 as f32) * (CHUNK_SIZE as f32) + (z as f32);
        *tile = sample_noises(noises, sample_x, sample_z);
    });
    height_map
}

fn sample_noises(noises: &[(FastNoiseLite, f32)], sample_x: f32, sample_z: f32) -> f32 {
    noises.iter().fold(0.0, |current, (noise, amplitude)| {
        current + sample_noise(noise, sample_x, sample_z) * amplitude
    })
}

fn sample_noise(noise: &FastNoiseLite, sample_x: f32, sample_z: f32) -> f32 {
    noise.get_noise_2d(sample_x, sample_z)
}

fn compute_normal(height_map: &[f32], x_index: usize, z_index: usize) -> [f32; 3] {
    let size = CHUNK_SIZE + 1;
    let idx = |x: usize, z: usize| x * size + z;

    let slope_x = if x_index == 0 {
        height_map[idx(1, z_index)] - height_map[idx(0, z_index)]
    } else if x_index == CHUNK_SIZE {
        height_map[idx(CHUNK_SIZE, z_index)] - height_map[idx(CHUNK_SIZE - 1, z_index)]
    } else {
        (height_map[idx(x_index + 1, z_index)] - height_map[idx(x_index - 1, z_index)]) / 2.0
    };

    let slope_z = if z_index == 0 {
        height_map[idx(x_index, 1)] - height_map[idx(x_index, 0)]
    } else if z_index == CHUNK_SIZE {
        height_map[idx(x_index, CHUNK_SIZE)] - height_map[idx(x_index, CHUNK_SIZE - 1)]
    } else {
        (height_map[idx(x_index, z_index + 1)] - height_map[idx(x_index, z_index - 1)]) / 2.0
    };

    let length = (slope_x * slope_x + slope_z * slope_z + 1.0).sqrt();
    [-slope_x / length, 1.0 / length, -slope_z / length]
}

fn get_vertices(position: (i32, i32), height_map: &[f32]) -> Vec<Vertex> {
    let size = CHUNK_SIZE + 1;
    let mut vertices = vec![Vertex::default(); size * size];
    for (x, row) in height_map.chunks(size).enumerate() {
        for (z, tile) in row.iter().enumerate() {
            let pos_x = (position.0 as f32) * (CHUNK_SIZE as f32) + (x as f32);
            let pos_z = (position.1 as f32) * (CHUNK_SIZE as f32) + (z as f32);
            let pos_y = *tile;
            let u = pos_x / (TEXTURE_SCALE * CHUNK_SIZE as f32);
            let v = pos_z / (TEXTURE_SCALE * CHUNK_SIZE as f32);
            let normal = compute_normal(height_map, x, z);

            vertices[x + z * size] = Vertex {
                position: [pos_x, pos_y, pos_z],
                tex_coords: [u, v],
                normal,
            }
        }
    }
    vertices
}

fn get_indices() -> Vec<u16> {
    let mut indices = vec![0u16; CHUNK_SIZE.pow(2) * 6];
    let mut tile_index = 0;
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let top_left = (x + z * (CHUNK_SIZE + 1)) as u16;
            let top_right = top_left + 1;
            let bottom_left = top_left + (CHUNK_SIZE + 1) as u16;
            let bottom_right = bottom_left + 1;

            indices[tile_index] = top_left;
            indices[tile_index + 1] = bottom_left;
            indices[tile_index + 2] = bottom_right;
            indices[tile_index + 3] = top_left;
            indices[tile_index + 4] = bottom_right;
            indices[tile_index + 5] = top_right;

            tile_index += 6;
        }
    }
    indices
}

fn create_mesh(device: &Device, vertices: &[Vertex], indices: &[u16]) -> (Buffer, Buffer) {
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Chunk Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Chunk Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
    });
    (vertex_buffer, index_buffer)
}

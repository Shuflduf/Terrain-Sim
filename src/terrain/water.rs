use std::collections::HashSet;

use wgpu::{Buffer, BufferUsages, Device, RenderPass, util::DeviceExt};

use crate::{
    renderer::vertex::{InstanceData, Vertex},
    terrain::{CHUNK_SIZE, IndicesArr, VerticesArr},
};

const WATER_LEVEL: f32 = -5.0;

pub struct Water {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    instance_count: u32,
}

impl Water {
    pub fn new(device: &Device) -> Self {
        let vertices = Self::create_vertices();
        let indices = Self::create_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });
        let instance_buffer = device.create_buffer(&wgpu::wgt::BufferDescriptor {
            label: Some("Water Instance buffer"),
            size: 1,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instance_count: 0,
        }
    }

    fn create_vertices() -> VerticesArr {
        let mut vertices = [Vertex::default(); (CHUNK_SIZE + 1).pow(2)];
        for x in 0..(CHUNK_SIZE + 1) {
            for z in 0..(CHUNK_SIZE + 1) {
                vertices[x + z * (CHUNK_SIZE + 1)] = Vertex {
                    position: [x as f32, WATER_LEVEL, z as f32],
                    tex_coords: [x as f32 / CHUNK_SIZE as f32, z as f32 / CHUNK_SIZE as f32],
                    normal: [0.0, 1.0, 0.0],
                }
            }
        }
        vertices
    }

    fn create_indices() -> IndicesArr {
        let mut indices = [0u16; CHUNK_SIZE.pow(2) * 6];
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

    pub fn update_instance_buffer(
        &mut self,
        device: &Device,
        chunk_positions: &HashSet<(i32, i32)>,
    ) {
        let instances: Vec<InstanceData> = chunk_positions
            .iter()
            .map(|&(x, z)| InstanceData {
                translation: [x as f32 * CHUNK_SIZE as f32, z as f32 * CHUNK_SIZE as f32],
            })
            .collect();
        self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: BufferUsages::VERTEX,
        });
        self.instance_count = instances.len() as u32;
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..(CHUNK_SIZE * CHUNK_SIZE * 6) as u32,
            0,
            0..self.instance_count,
        );
    }
}

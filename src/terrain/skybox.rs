use wgpu::{Buffer, BufferUsages, Device, RenderPass, util::DeviceExt};

use crate::renderer::vertex::Vertex;

pub struct Skybox {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl Skybox {
    pub fn new(device: &Device) -> Self {
        let vertices = Self::create_vertices();
        let indices = Self::create_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    fn create_vertices() -> [Vertex; 8] {
        [
            Vertex {
                position: [-1.0, -1.0, 1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                tex_coords: [0.0; 2],
                normal: [0.0; 3],
            },
        ]
    }

    fn create_indices() -> [u16; 36] {
        [
            0, 1, 2, 0, 2, 3, //
            5, 4, 7, 5, 7, 6, //
            3, 2, 6, 3, 6, 7, //
            4, 5, 1, 4, 1, 0, //
            1, 5, 6, 1, 6, 2, //
            4, 0, 3, 4, 3, 7, //
        ]
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

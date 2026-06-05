use fastnoise_lite::{FastNoiseLite, NoiseType};
use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::mesh::Vertex;

const CHUNK_SIZE: usize = 32;
type HeightMapArr = [[f32; CHUNK_SIZE + 1]; CHUNK_SIZE + 1];
type VerticesArr = [Vertex; (CHUNK_SIZE + 1).pow(2)];
type IndicesArr = [u16; CHUNK_SIZE.pow(2) * 6];

pub struct Chunk {
    position: (i32, i32),
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Chunk {
    pub fn new(&self, device: &Device, position: (i32, i32)) -> Self {
        let height_map = self.create_heightmap(position);
        let vertices = self.get_vertices(position, height_map);
        let indices = self.get_indices();
        let (vertex_buffer, index_buffer) = self.create_mesh(device, vertices, indices);

        Self {
            position,
            vertex_buffer,
            index_buffer,
        }
    }

    fn create_heightmap(&self, position: (i32, i32)) -> HeightMapArr {
        let mut height_map = [[0.0; CHUNK_SIZE + 1]; CHUNK_SIZE + 1];
        for x in 0..(CHUNK_SIZE + 1) {
            for z in 0..(CHUNK_SIZE + 1) {
                let sample_x = (position.0 as f32) * (CHUNK_SIZE as f32) + (x as f32);
                let sample_z = (position.1 as f32) * (CHUNK_SIZE as f32) + (z as f32);
                height_map[x][z] = self.sample_noise(sample_x, sample_z);
            }
        }
        height_map
    }

    fn sample_noise(&self, sample_x: f32, sample_z: f32) -> f32 {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.get_noise_2d(sample_x, sample_z)
    }

    fn create_mesh(
        &self,
        device: &Device,
        vertices: VerticesArr,
        indices: IndicesArr,
    ) -> (Buffer, Buffer) {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });
        (vertex_buffer, index_buffer)
    }

    fn get_vertices(&self, position: (i32, i32), height_map: HeightMapArr) -> VerticesArr {
        let mut vertices = [Vertex::default(); (CHUNK_SIZE + 1).pow(2)];
        for x in 0..(CHUNK_SIZE + 1) {
            for z in 0..(CHUNK_SIZE + 1) {
                let pos_x = (position.0 as f32) * (CHUNK_SIZE as f32) + (x as f32);
                let pos_z = (position.1 as f32) * (CHUNK_SIZE as f32) + (z as f32);
                let pos_y = height_map[x][z];
                let u = x as f32 / CHUNK_SIZE as f32;
                let v = z as f32 / CHUNK_SIZE as f32;

                vertices[x + z * (CHUNK_SIZE + 1)] = Vertex {
                    position: [pos_x, pos_y, pos_z],
                    tex_coords: [u, v],
                    normal: [0.0, 1.0, 0.0],
                }
            }
        }
        vertices
    }

    fn get_indices(&self) -> IndicesArr {
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

                tile_index += 6
            }
        }
        indices
    }
}

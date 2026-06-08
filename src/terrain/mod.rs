use std::collections::{HashMap, HashSet};

use cgmath::Point3;
use fastnoise_lite::{FastNoiseLite, NoiseType};
use wgpu::{Device, RenderPass};

use crate::{
    renderer::{frustum::Frustum, vertex::Vertex},
    terrain::{chunk::Chunk, skybox::Skybox, water::Water},
};

mod chunk;
mod skybox;
mod water;

const MAX_CHUNKS_PER_FRAME: usize = 8;
const RENDER_DISTANCE_RADIUS: u32 = 16;
const NOISE_VALUES: [(f32, f32); 4] = [(12.0, 0.005), (6.0, 0.02), (4.0, 0.05), (2.0, 0.1)];
const CHUNK_SIZE: usize = 64;
const TEXTURE_SCALE: f32 = 0.5;
type VerticesArr = [Vertex; (CHUNK_SIZE + 1).pow(2)];
type IndicesArr = [u16; CHUNK_SIZE.pow(2) * 6];

pub struct Terrain {
    chunks: HashMap<(i32, i32), Chunk>,
    pending_chunks: Vec<(i32, i32)>,
    noises: Vec<(FastNoiseLite, f32)>,
    water: Water,
    skybox: Skybox,
}

impl Terrain {
    pub fn new(device: &Device, seed: i32) -> Self {
        let mut noises = Vec::with_capacity(NOISE_VALUES.len());
        for (i, (amplitude, frequency)) in NOISE_VALUES.iter().enumerate() {
            let mut noise = FastNoiseLite::with_seed(seed + i as i32);
            noise.set_noise_type(Some(NoiseType::Perlin));
            noise.set_frequency(Some(*frequency));
            noises.push((noise, *amplitude));
        }

        let camera_chunk_x: i32 = 0;
        let camera_chunk_z: i32 = 0;
        let mut chunks = HashMap::new();
        let positions = Self::chunk_positions_in_radius(camera_chunk_x, camera_chunk_z);
        for pos in positions {
            chunks.insert(pos, Chunk::new(device, &noises, pos));
        }

        let water = Water::new(device);
        let skybox = Skybox::new(device);

        Self {
            chunks,
            pending_chunks: vec![],
            noises,
            water,
            skybox,
        }
    }

    fn chunk_positions_in_radius(chunk_x: i32, chunk_z: i32) -> HashSet<(i32, i32)> {
        let radius_sq = RENDER_DISTANCE_RADIUS.pow(2) as f32;
        let mut positions =
            HashSet::with_capacity((RENDER_DISTANCE_RADIUS as usize * 2 + 1).pow(2));

        for x in
            (chunk_x - RENDER_DISTANCE_RADIUS as i32)..=(chunk_x + RENDER_DISTANCE_RADIUS as i32)
        {
            for z in (chunk_z - RENDER_DISTANCE_RADIUS as i32)
                ..=(chunk_z + RENDER_DISTANCE_RADIUS as i32)
            {
                if ((x - chunk_x).pow(2) + (z - chunk_z).pow(2)) as f32 <= radius_sq {
                    positions.insert((x, z));
                }
            }
        }

        positions
    }

    pub fn chunk_aabb(chunk_pos: (i32, i32)) -> (Point3<f32>, Point3<f32>) {
        let max_noise_height: f32 = NOISE_VALUES.iter().map(|(amplitude, _)| amplitude).sum();
        let x = chunk_pos.0 as f32 * CHUNK_SIZE as f32;
        let z = chunk_pos.1 as f32 * CHUNK_SIZE as f32;
        (
            Point3::new(x, -max_noise_height, z),
            Point3::new(
                x + CHUNK_SIZE as f32,
                max_noise_height,
                z + CHUNK_SIZE as f32,
            ),
        )
    }

    pub fn update(&mut self, device: &Device, camera_position: &cgmath::Point3<f32>) {
        let camera_chunk_x = (camera_position.x / CHUNK_SIZE as f32).floor() as i32;
        let camera_chunk_z = (camera_position.z / CHUNK_SIZE as f32).floor() as i32;
        let positions = Self::chunk_positions_in_radius(camera_chunk_x, camera_chunk_z);
        self.chunks.retain(|pos, _| positions.contains(pos));
        self.pending_chunks.clear();
        self.pending_chunks.extend(
            positions
                .iter()
                .filter(|pos| !self.chunks.contains_key(pos)),
        );
        let count = MAX_CHUNKS_PER_FRAME.min(self.pending_chunks.len());
        for pos in self.pending_chunks.drain(..count) {
            self.chunks
                .insert(pos, Chunk::new(device, &self.noises, pos));
        }
        self.water.update_instance_buffer(device, &positions);
    }

    pub fn draw(&self, render_pass: &mut RenderPass, frustum: &Frustum) {
        self.chunks
            .iter()
            .filter(|(pos, _)| frustum.intersects_aabb(Self::chunk_aabb(**pos)))
            .for_each(|(_, chunk)| chunk.draw(render_pass));
    }

    pub fn draw_water(&self, render_pass: &mut RenderPass) {
        self.water.draw(render_pass);
    }

    pub fn draw_skybox(&self, render_pass: &mut RenderPass) {
        self.skybox.draw(render_pass);
    }
}

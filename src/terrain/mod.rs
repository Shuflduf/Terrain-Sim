use fastnoise_lite::{FastNoiseLite, NoiseType};
use wgpu::{Device, RenderPass};

use crate::{
    renderer::vertex::Vertex,
    terrain::{chunk::Chunk, water::Water},
};

mod chunk;
mod water;

const NOISE_VALUES: [(f32, f32); 4] = [(12.0, 0.005), (6.0, 0.02), (4.0, 0.05), (2.0, 0.1)];
const CHUNK_SIZE: usize = 32;
type VerticesArr = [Vertex; (CHUNK_SIZE + 1).pow(2)];
type IndicesArr = [u16; CHUNK_SIZE.pow(2) * 6];

pub struct Terrain {
    chunks: Vec<Chunk>,
    noises: Vec<(FastNoiseLite, f32)>,
    water: Water,
}

impl Terrain {
    pub fn new(device: &Device, seed: i32) -> Self {
        let mut noises = Vec::with_capacity(NOISE_VALUES.len());
        for (i, (amplitude, frequency)) in NOISE_VALUES.iter().enumerate() {
            let mut noise = FastNoiseLite::with_seed(seed + i as i32);
            noise.set_noise_type(Some(NoiseType::Perlin));
            noise.set_frequency(Some(*frequency));
            noises.push((noise, *amplitude))
        }

        let mut chunks = Vec::with_capacity(20 * 20);

        for x in -10..10 {
            for z in -10..10 {
                chunks.push(Chunk::new(device, &noises, (x, z)))
            }
        }

        let water = Water::new(device);

        Self {
            chunks,
            noises,
            water,
        }
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        self.chunks.iter().for_each(|chunk| chunk.draw(render_pass));
    }

    pub fn draw_water(&self, render_pass: &mut RenderPass) {
        self.water.draw(render_pass);
    }
}

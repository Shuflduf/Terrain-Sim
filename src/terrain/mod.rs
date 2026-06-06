use fastnoise_lite::{FastNoiseLite, NoiseType};
use wgpu::{Device, RenderPass};

use crate::terrain::chunk::Chunk;

mod chunk;

pub struct Terrain {
    chunks: Vec<Chunk>,
    noise: FastNoiseLite,
}

impl Terrain {
    pub fn new(device: &Device, seed: i32) -> Self {
        let mut noise = FastNoiseLite::with_seed(seed);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));

        let mut chunks = vec![];

        for x in -10..=10 {
            for z in -10..=10 {
                chunks.push(Chunk::new(device, &noise, (x, z)))
            }
        }

        Self { chunks, noise }
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        self.chunks.iter().for_each(|chunk| chunk.draw(render_pass));
    }
}

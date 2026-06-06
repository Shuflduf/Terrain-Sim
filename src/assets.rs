use std::collections::HashMap;

use wgpu::{Device, Queue};

use crate::renderer::texture::Texture;

pub struct Assets {
    pub textures: HashMap<&'static str, Texture>,
}

impl Assets {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let mut textures = HashMap::new();

        macro_rules! load_texture {
            ($name:expr, $path:expr) => {
                let bytes = include_bytes!($path);
                let texture = Texture::from_bytes(device, queue, bytes, $name)
                    .expect(concat!("Failed to load ", $name));
                textures.insert($name, texture);
            };
        }

        load_texture!("frog", "../assets/frog.png");
        Self { textures }
    }
}

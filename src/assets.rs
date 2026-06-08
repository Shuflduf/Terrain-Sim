use std::collections::HashMap;

use crate::renderer::texture::Texture;
use wgpu::{Device, Queue, ShaderModule};

pub struct Assets {
    textures: HashMap<&'static str, Texture>,
    shaders: HashMap<&'static str, ShaderModule>,
    terrain_texture: Texture,
}

impl Assets {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let mut textures = HashMap::new();
        let mut shaders = HashMap::new();

        macro_rules! load_image {
            ($path:expr) => {{
                let bytes = include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/images/",
                    $path
                ));
                image::load_from_memory(bytes).unwrap()
            }};
        }

        macro_rules! load_texture {
            ($name:expr, $path:expr) => {
                let bytes = include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/images/",
                    $path
                ));
                let texture = Texture::from_bytes(device, queue, bytes, $name)
                    .expect(concat!("Failed to load ", $name));
                textures.insert($name, texture);
            };
        }

        macro_rules! load_shader {
            ($name:expr, $path:expr) => {
                let module = device.create_shader_module(wgpu::include_wgsl!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/shaders/",
                    $path
                )));
                shaders.insert($name, module);
            };
        }

        let terrain_images = vec![
            load_image!("sand.png"),
            load_image!("grass.png"),
            load_image!("stone.png"),
            load_image!("snow.png"),
        ];
        let terrain_texture =
            Texture::create_texture_array(device, queue, &terrain_images, "Terrain Texture Array")
                .unwrap();

        load_texture!("frog", "frog.png");
        // load_texture!("grass", "grass.png");
        // load_texture!("sand", "sand.png");
        // load_texture!("stone", "stone.png");
        // load_texture!("snow", "snow.png");
        load_texture!("water", "water.png");

        load_shader!("main", "main.wgsl");
        load_shader!("water", "water.wgsl");

        Self {
            textures,
            shaders,
            terrain_texture,
        }
    }

    pub fn texture(&self, name: &str) -> &Texture {
        self.textures
            .get(name)
            .unwrap_or_else(|| panic!("Texture `{name}` not found"))
    }

    pub fn shader(&self, name: &str) -> &ShaderModule {
        self.shaders
            .get(name)
            .unwrap_or_else(|| panic!("Shader `{name}` not found"))
    }

    pub fn terrain_texture(&self) -> &Texture {
        &self.terrain_texture
    }
}

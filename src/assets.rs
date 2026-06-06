use std::collections::HashMap;

use crate::renderer::texture::Texture;
use wgpu::{Device, Queue, ShaderModule};

pub struct Assets {
    textures: HashMap<&'static str, Texture>,
    shaders: HashMap<&'static str, ShaderModule>,
}

impl Assets {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let mut textures = HashMap::new();
        let mut shaders = HashMap::new();

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

        load_texture!("frog", "frog.png");
        load_texture!("grass", "grass.png");

        load_shader!("main", "main.wgsl");

        Self { textures, shaders }
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
}

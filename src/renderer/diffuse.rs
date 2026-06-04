use wgpu::{BindGroup, BindGroupLayout, Device, Queue};

use crate::renderer::texture;

pub fn create_diffuse_bind_group(
    device: &Device,
    queue: &Queue,
    layout: &BindGroupLayout,
) -> anyhow::Result<BindGroup> {
    let diffuse_bytes = include_bytes!("../frog.png");
    let diffuse_texture = texture::Texture::from_bytes(device, queue, diffuse_bytes, "Frog Image")?;

    Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        label: Some("Diffuse Bind Group"),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            },
        ],
    }))
}

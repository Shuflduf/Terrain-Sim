use wgpu::{BindGroup, BindGroupLayout, Device};

use crate::assets::Assets;

pub fn create_diffuse_bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    assets: &Assets,
) -> anyhow::Result<BindGroup> {
    let diffuse_texture = assets.texture("grass");

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

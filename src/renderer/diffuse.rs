use wgpu::{BindGroup, BindGroupEntry, BindGroupLayout, BindingResource, Buffer, Device};

use crate::renderer::texture::Texture;

pub fn create_terrain_bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    terrain_texture: &Texture,
    blend_buffer: &Buffer,
) -> anyhow::Result<BindGroup> {
    Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        label: Some("Diffuse Bind Group"),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&terrain_texture.view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&terrain_texture.sampler),
            },
            BindGroupEntry {
                binding: 2,
                resource: blend_buffer.as_entire_binding(),
            },
        ],
    }))
}

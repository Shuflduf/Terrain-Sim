use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer, Device,
    ShaderStages,
};

use crate::assets::Assets;

pub fn create_water_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Water Bind Group Layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

pub fn create_water_bind_group(
    device: &Device,
    assets: &Assets,
    layout: &BindGroupLayout,
    uniform_buffer: &Buffer,
) -> BindGroup {
    let texture = assets.texture("water");
    let water_sampler = device.create_sampler(&wgpu::wgt::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Water Bind Group"),
        layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&water_sampler),
            },
            BindGroupEntry {
                binding: 2,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    })
}

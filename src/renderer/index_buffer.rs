use wgpu::{Buffer, Device, util::DeviceExt};

use crate::mesh::INDICES;

pub fn create_index_buffer(device: &Device) -> (Buffer, u32) {
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    let num_indices = INDICES.len() as u32;
    (index_buffer, num_indices)
}

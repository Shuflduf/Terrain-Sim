use wgpu::{
    Buffer, Device, util::DeviceExt,
};

use crate::mesh::VERTICES;

pub fn create_vertex_buffer(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

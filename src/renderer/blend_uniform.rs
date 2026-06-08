use wgpu::{Buffer, BufferUsages, Device, util::DeviceExt};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BlendUniform {
    pub thresholds: [f32; 3],
    pub blend_widths: [f32; 3],
}

impl BlendUniform {
    pub fn create_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Blend Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }
}

impl Default for BlendUniform {
    fn default() -> Self {
        Self {
            thresholds: [2.0, 12.0, 20.0],
            blend_widths: [4.0, 4.0, 4.0],
        }
    }
}

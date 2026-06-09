use wgpu::{Buffer, BufferUsages, Device, util::DeviceExt};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BlendUniform {
    // 4 instead of 3 to align to 16 bytes
    thresholds: [f32; 4],
    blend_widths: [f32; 4],
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
            thresholds: [-4.0, 2.0, 8.0, 0.0],
            blend_widths: [4.0, 4.0, 4.0, 0.0],
        }
    }
}

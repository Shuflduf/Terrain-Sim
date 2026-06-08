use wgpu::{Buffer, BufferUsages, Device, util::DeviceExt};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaterUniform {
    pub time: f32,
    pub wave_height: f32,
    pub wave_frequency: f32,
    pub scroll_speed: f32,
}

impl Default for WaterUniform {
    fn default() -> Self {
        Self {
            time: 0.0,
            wave_height: 0.3,
            wave_frequency: 0.05,
            scroll_speed: 0.02,
        }
    }
}

impl WaterUniform {
    pub fn create_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }
}

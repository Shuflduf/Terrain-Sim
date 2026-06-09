use cgmath::SquareMatrix;
use wgpu::{Buffer, Device, util::DeviceExt};

use crate::{camera::Camera, renderer::frustum::Frustum};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4],
    skybox_projection: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_projection: cgmath::Matrix4::identity().into(),
            skybox_projection: cgmath::Matrix4::identity().into(),
        }
    }
}

impl CameraUniform {
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix().into();
        self.skybox_projection = camera.build_skybox_projection_matrix().into();
    }

    pub fn frustum(&self) -> Frustum {
        Frustum::from_view_projection(&self.view_projection)
    }
}

pub fn create_camera_buffer(device: &Device, camera: &Camera) -> (CameraUniform, Buffer) {
    let mut camera_uniform = CameraUniform::default();
    camera_uniform.update_view_proj(camera);
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    (camera_uniform, camera_buffer)
}

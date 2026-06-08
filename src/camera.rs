use std::f32::consts::FRAC_PI_2;

use cgmath::{Angle, InnerSpace, Rad};

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0), //
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0), //
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0), //
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0), //
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub vertical_fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn resize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let view = cgmath::Matrix4::look_to_rh(
            self.eye,
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            self.up,
        );

        let proj = cgmath::perspective(
            cgmath::Deg(self.vertical_fov),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn build_skybox_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let view = cgmath::Matrix4::look_to_rh(
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            self.up,
        );

        let proj = cgmath::perspective(
            cgmath::Deg(self.vertical_fov),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: (0.0, 10.0, 4.0).into(),
            yaw: Rad(-FRAC_PI_2),
            pitch: Rad(0.0),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            vertical_fov: 45.0,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }
}

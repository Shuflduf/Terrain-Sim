use std::f32::consts::FRAC_PI_2;

use cgmath::{Angle, ElementWise, InnerSpace, Rad, Vector3};
use wgpu::naga::back::hlsl::EntryPointError;
use winit::keyboard::KeyCode;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0), //
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0), //
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0), //
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0), //
);

pub struct Camera {
    eye: cgmath::Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    vertical_fov: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            eye: (0.0, 0.0, 4.0).into(),
            yaw: Rad(-FRAC_PI_2),
            pitch: Rad(0.0),
            up: cgmath::Vector3::unit_y(),
            aspect: width / height,
            vertical_fov: 45.0,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }

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
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1000.0, 1000.0)
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    mouse_control: bool,
    mouse_sensitivity: f32,
    mouse_delta: (f64, f64),
}

impl CameraController {
    pub fn new(speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            speed,
            mouse_sensitivity,
            ..Default::default()
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, dx: f64, dy: f64) {
        self.mouse_delta = (self.mouse_delta.0 + dx, self.mouse_delta.1 - dy)
    }

    pub fn toggle_mouse(&mut self) -> bool {
        self.mouse_control = !self.mouse_control;
        return self.mouse_control;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        if self.mouse_control {
            camera.yaw += Rad(self.mouse_delta.0 as f32 * self.mouse_sensitivity);
            camera.pitch += Rad(self.mouse_delta.1 as f32 * self.mouse_sensitivity);
            if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
                camera.pitch = -Rad(SAFE_FRAC_PI_2);
            } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
                camera.pitch = Rad(SAFE_FRAC_PI_2);
            }
        }
        self.mouse_delta = (0.0, 0.0);

        let forward = Vector3::new(
            camera.yaw.cos() * camera.pitch.cos(),
            camera.pitch.sin(),
            camera.yaw.sin() * camera.pitch.cos(),
        )
        .normalize();
        let right = forward.cross(camera.up);

        if self.is_forward_pressed {
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.speed;
        }
        if self.is_right_pressed {
            camera.eye += right * self.speed;
        }

        // use cgmath::InnerSpace;
        // let forward = camera.target - camera.eye;
        // let forward_norm = forward.normalize();
        // let forward_mag = forward.magnitude();

        // // Prevents glitching when the camera gets too close to the
        // // center of the scene.
        // if self.is_forward_pressed && forward_mag > self.speed {
        //     camera.eye += forward_norm * self.speed;
        // }
        // if self.is_backward_pressed {
        //     camera.eye -= forward_norm * self.speed;
        // }

        // let right = forward_norm.cross(camera.up);

        // // Redo radius calc in case the forward/backward is pressed.
        // let forward = camera.target - camera.eye;
        // let forward_mag = forward.magnitude();

        // if self.is_right_pressed {
        //     // Rescale the distance between the target and the eye so
        //     // that it doesn't change. The eye, therefore, still
        //     // lies on the circle made by the target and eye.
        //     camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        // }
        // if self.is_left_pressed {
        //     camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        // }
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 0.02,
            mouse_control: false,
            mouse_sensitivity: 0.002,
            mouse_delta: (0.0, 0.0),
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}

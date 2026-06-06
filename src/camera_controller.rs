use std::f32::consts::FRAC_PI_2;

use cgmath::{Angle, InnerSpace, Rad, Vector3};
use winit::keyboard::KeyCode;

use crate::camera::Camera;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

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
        self.mouse_delta = (self.mouse_delta.0 + dx, self.mouse_delta.1 - dy);
    }

    pub fn toggle_mouse(&mut self) -> bool {
        self.mouse_control = !self.mouse_control;
        self.mouse_control
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
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 0.2,
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

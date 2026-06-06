use std::sync::Arc;

use crate::{
    camera::Camera, camera_controller::CameraController, gpu_context::GpuContext, terrain::Terrain,
};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

const SEED: i32 = 20;

pub struct Application {
    gpu_context: Option<GpuContext>,
    camera: Camera,
    camera_controller: CameraController,
    terrain: Option<Terrain>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            gpu_context: None,
            camera: Camera::default(),
            camera_controller: CameraController::default(),
            terrain: None,
        }
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        } else {
            self.camera_controller.handle_key(code, is_pressed);
        }
    }

    fn handle_mouse_button(&mut self, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left {
            let lock_mouse = self.camera_controller.toggle_mouse();
            self.gpu_context.as_ref().unwrap().lock_mouse(lock_mouse);
        }
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
    }
}

impl ApplicationHandler<GpuContext> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let size = window.inner_size();
        self.camera = Camera::new(size.width as f32, size.height as f32);
        self.gpu_context = Some(pollster::block_on(GpuContext::new(window, &self.camera)).unwrap());
        self.terrain = Some(Terrain::new(
            &self.gpu_context.as_ref().unwrap().device,
            SEED,
        ));
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: GpuContext) {
        self.gpu_context = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let gpu_context = match &mut self.gpu_context {
            Some(ctx) => ctx,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.camera.resize(size.width as f32, size.height as f32);
                gpu_context.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.update();
                if let Some(ctx) = &mut self.gpu_context {
                    ctx.update(&self.camera);
                }
                match self
                    .gpu_context
                    .as_mut()
                    .unwrap()
                    .render(self.terrain.as_ref().unwrap())
                {
                    Ok(()) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                self.handle_key(event_loop, code, key_state.is_pressed());
            }
            WindowEvent::MouseInput { button, .. } => self.handle_mouse_button(button),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.camera_controller.handle_mouse(delta.0, delta.1);
        }
    }
}

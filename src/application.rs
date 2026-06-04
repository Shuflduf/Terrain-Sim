use std::sync::Arc;

use crate::{
    camera::{Camera, CameraController},
    gpu_context::GpuContext,
};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

pub struct Application {
    gpu_context: Option<GpuContext>,
    camera: Option<Camera>,
    camera_controller: CameraController,
}

impl Application {
    pub fn new() -> Self {
        Self {
            gpu_context: None,
            camera: None,
            camera_controller: CameraController::new(0.2),
        }
    }
}

impl ApplicationHandler<GpuContext> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let size = window.inner_size();
        let camera = Camera::new(size.width as f32, size.height as f32);
        self.gpu_context = Some(pollster::block_on(GpuContext::new(window, &camera)).unwrap());
        self.camera = Some(camera);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: GpuContext) {
        self.gpu_context = Some(event)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let gpu_context = match &mut self.gpu_context {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => gpu_context.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                gpu_context.update();
                match gpu_context.render() {
                    Ok(_) => {}
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
            } => gpu_context.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

use winit::event_loop::EventLoop;

use crate::application::Application;

mod application;
mod camera;
mod camera_controller;
mod gpu_context;
mod mesh;
mod renderer;
mod terrain;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = Application::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

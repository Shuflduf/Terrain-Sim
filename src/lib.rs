use winit::event_loop::EventLoop;

use crate::application::Application;

mod application;
mod gpu_context;
mod mesh;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = Application::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

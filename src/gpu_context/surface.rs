use std::sync::Arc;

use wgpu::{
    Instance, Surface,
};
use winit::window::Window;

pub fn create_surface(
    instance: &Instance,
    window: &Arc<Window>,
) -> anyhow::Result<Surface<'static>> {
    Ok(instance.create_surface(window.clone())?)
}

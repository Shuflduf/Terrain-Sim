use std::sync::Arc;

use wgpu::{Adapter, PresentMode, Surface};
use winit::window::Window;

pub fn create_config(
    surface: &Surface,
    adapter: &Adapter,
    window: &Arc<Window>,
    vsync: bool,
) -> wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>> {
    let surface_caps = surface.get_capabilities(adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let present_mode = if vsync {
        PresentMode::FifoRelaxed
    } else {
        PresentMode::Immediate
    };

    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

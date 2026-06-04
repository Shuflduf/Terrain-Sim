use crate::{
    camera::Camera,
    gpu_context::{
        adapter::request_adapter, config::create_config, device::request_device,
        instance::create_instance, surface::create_surface,
    },
    renderer::Renderer,
};
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::{CursorGrabMode, Window};

mod adapter;
mod config;
mod device;
mod instance;
mod surface;

pub struct GpuContext {
    pub config: SurfaceConfiguration,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    window: Arc<Window>,
    is_surface_configured: bool,

    renderer: Renderer,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>, camera: &Camera) -> anyhow::Result<Self> {
        let instance = create_instance();
        let surface = create_surface(&instance, &window)?;
        let adapter = request_adapter(&instance, &surface).await?;
        let (device, queue) = request_device(&adapter).await?;
        let config = create_config(&surface, &adapter, &window);
        let renderer = Renderer::new(&device, &config, &queue, camera)?;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            is_surface_configured: false,
            renderer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.renderer.update_camera(camera, &self.queue)
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            Ok(surface_texture) => {
                if surface_texture.suboptimal {
                    self.surface.configure(&self.device, &self.config);
                }
                surface_texture
            }
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Timeout) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            Err(wgpu::SurfaceError::Lost) => anyhow::bail!("Lost device"),
            Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => return Ok(()),
        };
        let view = output
            .texture
            .create_view(&wgpu::wgt::TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .create_command_encoder(&wgpu::wgt::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        self.renderer.draw(&mut encoder, &view);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn lock_mouse(&self, lock: bool) {
        self.window.set_cursor_visible(!lock);
        if lock {
            if let Err(_) = self.window.set_cursor_grab(CursorGrabMode::Locked) {
                let _ = self.window.set_cursor_grab(CursorGrabMode::Confined);
            }
        } else {
            let _ = self.window.set_cursor_grab(CursorGrabMode::None);
        }
    }
}

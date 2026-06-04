use crate::{
    camera::Camera,
    gpu_context::{
        adapter::request_adapter,
        camera_buffer::{CameraUniform, create_camera_buffer},
        config::create_config,
        device::request_device,
        instance::create_instance,
        surface::create_surface,
    },
    renderer::Renderer,
};
use std::sync::Arc;
use wgpu::{Buffer, Device, Queue, Surface, SurfaceConfiguration};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

mod adapter;
mod camera_buffer;
mod config;
mod device;
mod instance;
mod surface;

pub struct GpuContext {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    window: Arc<Window>,
    is_surface_configured: bool,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    renderer: Renderer,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let instance = create_instance();
        let surface = create_surface(&instance, &window)?;
        let adapter = request_adapter(&instance, &surface).await?;
        let (device, queue) = request_device(&adapter).await?;
        let config = create_config(&surface, &adapter, &window);
        let camera = Camera::new(&config);
        let (camera_uniform, camera_buffer) = create_camera_buffer(&device, &camera);
        let renderer = Renderer::new(&device, &config, &queue, &camera_buffer)?;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            is_surface_configured: false,
            camera,
            camera_uniform,
            camera_buffer,
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

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn update(&mut self) {}

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
}

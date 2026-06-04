use crate::gpu_context::{
    adapter::request_adapter, config::create_config, device::request_device,
    diffuse::create_diffuse_bind_group, index_buffer::create_index_buffer,
    instance::create_instance, layout::create_texture_bind_group_layout,
    pipeline::create_render_pipeline, surface::create_surface, vertex_buffer::create_vertex_buffer,
};
use std::sync::Arc;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

mod adapter;
mod config;
mod device;
mod diffuse;
mod index_buffer;
mod instance;
mod layout;
mod pipeline;
mod surface;
mod vertex_buffer;

pub struct GpuContext {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    is_surface_configured: bool,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let instance = create_instance();
        let surface = create_surface(&instance, &window)?;
        let adapter = request_adapter(&instance, &surface).await?;

        let (device, queue) = request_device(&adapter).await?;
        let config = create_config(&surface, &adapter, &window);
        let layout = create_texture_bind_group_layout(&device);
        let diffuse_bind_group = create_diffuse_bind_group(&device, &queue, &layout)?;
        let render_pipeline = create_render_pipeline(&device, &config, &layout);
        let vertex_buffer = create_vertex_buffer(&device);
        let (index_buffer, num_indices) = create_index_buffer(&device);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            is_surface_configured: false,
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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: (None),
            timestamp_writes: (None),
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

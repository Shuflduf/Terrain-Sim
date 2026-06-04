use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureView};

use crate::renderer::{
    diffuse::create_diffuse_bind_group, index_buffer::create_index_buffer,
    layout::create_texture_bind_group_layout, pipeline::create_render_pipeline,
    vertex_buffer::create_vertex_buffer,
};

mod diffuse;
mod index_buffer;
mod layout;
mod pipeline;
mod vertex_buffer;

pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        queue: &Queue,
    ) -> anyhow::Result<Self> {
        let layout = create_texture_bind_group_layout(device);
        let diffuse_bind_group = create_diffuse_bind_group(device, queue, &layout)?;
        let render_pipeline = create_render_pipeline(device, config, &layout);
        let vertex_buffer = create_vertex_buffer(device);
        let (index_buffer, num_indices) = create_index_buffer(device);
        Ok(Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
        })
    }

    pub fn draw(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
    }
}

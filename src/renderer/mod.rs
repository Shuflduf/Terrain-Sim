use wgpu::{
    BindGroup, Buffer, CommandEncoder, Device, Queue, RenderPipeline, SurfaceConfiguration,
    TextureView,
};

use crate::{
    assets::Assets,
    camera::Camera,
    renderer::{
        camera_bind_group::create_camera_bind_group,
        camera_buffer::{CameraUniform, create_camera_buffer},
        diffuse::create_diffuse_bind_group,
        layout::create_texture_bind_group_layout,
        pipeline::create_render_pipeline,
    },
    terrain::Terrain,
};

mod camera_bind_group;
mod camera_buffer;
mod diffuse;
mod layout;
mod pipeline;
pub mod texture;
pub mod vertex;

pub struct Renderer {
    render_pipeline: RenderPipeline,
    diffuse_bind_group: BindGroup,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl Renderer {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        camera: &Camera,
        assets: &Assets,
    ) -> anyhow::Result<Self> {
        let texture_layout = create_texture_bind_group_layout(device);
        let diffuse_bind_group = create_diffuse_bind_group(device, &texture_layout, assets)?;
        let (camera_uniform, camera_buffer) = create_camera_buffer(device, camera);
        let (camera_bind_group, camera_layout) = create_camera_bind_group(device, &camera_buffer);
        let render_pipeline =
            create_render_pipeline(device, config, &texture_layout, &camera_layout, assets);

        Ok(Self {
            render_pipeline,
            diffuse_bind_group,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        })
    }

    pub fn update_camera(&mut self, camera: &Camera, queue: &Queue) {
        self.camera_uniform.update_view_proj(camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn draw(&self, encoder: &mut CommandEncoder, view: &TextureView, terrain: &Terrain) {
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
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        terrain.draw(&mut render_pass);

        drop(render_pass);
    }
}

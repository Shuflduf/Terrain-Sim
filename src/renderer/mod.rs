use wgpu::{
    BindGroup, Buffer, CommandEncoder, Device, Queue, RenderPipeline, SurfaceConfiguration,
    TextureView,
};

use crate::{
    assets::Assets,
    camera::Camera,
    renderer::{
        blend_uniform::BlendUniform,
        camera_bind_group::create_camera_bind_group,
        camera_buffer::{CameraUniform, create_camera_buffer},
        diffuse::create_terrain_bind_group,
        layout::create_texture_bind_group_layout,
        pipeline::create_render_pipeline,
        texture::Texture,
    },
    terrain::Terrain,
};

mod blend_uniform;
mod camera_bind_group;
mod camera_buffer;
mod diffuse;
mod layout;
mod pipeline;
pub mod texture;
pub mod vertex;

pub struct Renderer {
    pub depth_texture: Texture,
    render_pipeline: RenderPipeline,
    terrain_bind_group: BindGroup,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    blend_uniform: BlendUniform,
    blend_buffer: Buffer,
}

impl Renderer {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        camera: &Camera,
        assets: &Assets,
    ) -> anyhow::Result<Self> {
        let texture_layout = create_texture_bind_group_layout(device);

        let blend_uniform = BlendUniform::default();
        let blend_buffer = blend_uniform.create_buffer(device);

        let terrain_texture = assets.terrain_texture();
        let terrain_bind_group =
            create_terrain_bind_group(device, &texture_layout, terrain_texture, &blend_buffer)?;

        let (camera_uniform, camera_buffer) = create_camera_buffer(device, camera);
        let (camera_bind_group, camera_layout) = create_camera_bind_group(device, &camera_buffer);

        let depth_texture = Texture::create_depth_texture(device, config, "Depth Texture");
        let render_pipeline = create_render_pipeline(
            device,
            config,
            assets,
            &texture_layout,
            &camera_layout,
            &depth_texture,
        );

        Ok(Self {
            render_pipeline,
            terrain_bind_group,
            camera_uniform,
            camera_buffer,
            depth_texture,
            camera_bind_group,
            blend_uniform,
            blend_buffer,
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.terrain_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        terrain.draw(&mut render_pass);

        drop(render_pass);
    }
}

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
        skybox_bind_group::{create_skybox_bind_group, create_skybox_bind_group_layout},
        skybox_pipeline::create_skybox_pipeline,
        texture::Texture,
        water_bind_group::{create_water_bind_group, create_water_bind_group_layout},
        water_pipeline::create_water_pipeline,
        water_uniform::WaterUniform,
    },
    terrain::Terrain,
};

mod blend_uniform;
mod camera_bind_group;
mod camera_buffer;
mod diffuse;
mod layout;
mod pipeline;
mod skybox_bind_group;
mod skybox_pipeline;
pub mod texture;
pub mod vertex;
mod water_bind_group;
mod water_pipeline;
mod water_uniform;

pub struct Renderer {
    pub depth_texture: Texture,
    render_pipeline: RenderPipeline,
    terrain_bind_group: BindGroup,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    water_pipeline: RenderPipeline,
    water_bind_group: BindGroup,
    water_uniform: WaterUniform,
    water_buffer: Buffer,
    skybox_pipeline: RenderPipeline,
    skybox_bind_group: BindGroup,
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

        let water_uniform = WaterUniform::default();
        let water_buffer = water_uniform.create_buffer(device);
        let water_texture_layout = create_water_bind_group_layout(device);
        let water_bind_group =
            create_water_bind_group(device, assets, &water_texture_layout, &water_buffer);
        let water_pipeline = create_water_pipeline(
            device,
            config,
            assets,
            &water_texture_layout,
            &camera_layout,
        );

        let skybox_texture_layout = create_skybox_bind_group_layout(device);
        let skybox_bind_group = create_skybox_bind_group(device, assets, &skybox_texture_layout);
        let skybox_pipeline = create_skybox_pipeline(
            device,
            config,
            assets,
            &skybox_texture_layout,
            &camera_layout,
        );

        Ok(Self {
            render_pipeline,
            terrain_bind_group,
            camera_uniform,
            camera_buffer,
            depth_texture,
            camera_bind_group,
            water_pipeline,
            water_bind_group,
            water_uniform,
            water_buffer,
            skybox_pipeline,
            skybox_bind_group,
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

    pub fn draw(
        &self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        terrain: &Terrain,
        queue: &Queue,
        time: f32,
    ) {
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

        let mut updated_water_uniform = self.water_uniform;
        updated_water_uniform.time = time;
        queue.write_buffer(
            &self.water_buffer,
            0,
            bytemuck::cast_slice(&[updated_water_uniform]),
        );

        render_pass.set_pipeline(&self.water_pipeline);
        render_pass.set_bind_group(0, &self.water_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        terrain.draw_water(&mut render_pass);

        render_pass.set_pipeline(&self.skybox_pipeline);
        render_pass.set_bind_group(0, &self.skybox_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        terrain.draw_skybox(&mut render_pass);

        drop(render_pass);
    }
}

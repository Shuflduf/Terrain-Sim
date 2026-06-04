use wgpu::{
    Adapter, Instance, Surface,
};

pub async fn request_adapter(
    instance: &Instance,
    surface: &Surface<'_>,
) -> anyhow::Result<Adapter> {
    Ok(instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await?)
}

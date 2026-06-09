use wgpu::{
    Adapter, Device, Queue,
};

pub async fn request_device(adapter: &Adapter) -> anyhow::Result<(Device, Queue)> {
    let limits = if cfg!(target_arch = "wasm32") {
        wgpu::Limits::downlevel_webgl2_defaults()
    } else {
        wgpu::Limits::defaults()
    };
    Ok(adapter
        .request_device(&wgpu::wgt::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
            ..Default::default()
        })
        .await?)
}

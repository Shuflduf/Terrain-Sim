use wgpu::{
    Instance, InstanceDescriptor,
};

pub fn create_instance() -> Instance {
    let backends = if cfg!(target_arch = "wasm32") {
        wgpu::Backends::GL
    } else {
        wgpu::Backends::PRIMARY
    };
    Instance::new(&InstanceDescriptor {
        backends,
        ..Default::default()
    })
}

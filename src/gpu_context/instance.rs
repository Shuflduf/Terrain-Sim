use wgpu::{
    Instance, InstanceDescriptor,
};

pub fn create_instance() -> Instance {
    Instance::new(&InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    })
}

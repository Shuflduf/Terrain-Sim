const PI: f32 = 3.14159265f;

struct CameraUniform {
    view_proj: mat4x4<f32>,
    skybox_projection: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
};

@group(0) @binding(0)
var skybox_texture: texture_2d<f32>;
@group(0) @binding(1)
var skybox_sampler: sampler;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = model.position;
    out.clip_position = camera.skybox_projection * vec4<f32>(model.position, 1.0);
    out.clip_position = out.clip_position.xyww;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.world_position);
    let u = 0.5 + atan2(dir.z, dir.x) / (2.0 * PI);
    let v = 0.5 - asin(dir.y) / PI;
    let color = textureSample(skybox_texture, skybox_sampler, vec2<f32>(u, v));
    return color;
}


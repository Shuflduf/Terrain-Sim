struct CameraUniform {
    view_proj: mat4x4<f32>,
    skybox_projection: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct WaterUniform {
    time: f32,
    wave_height: f32,
    wave_frequency: f32,
    scroll_speed: f32,
}
@group(0) @binding(2) 
var<uniform> water: WaterUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) translation: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@group(0) @binding(0)
var water_texture: texture_2d<f32>;
@group(0) @binding(1)
var water_sampler: sampler;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;

    let world_x = model.position.x + model.translation.x;
    let world_z = model.position.z + model.translation.y;
    let wave = sin(world_x * water.wave_frequency + world_z * water.wave_frequency + water.time) * water.wave_height;
    let world_pos = vec3<f32>(
        world_x,
        model.position.y + wave,
        world_z
    );
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.normal = model.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_direction = normalize(vec3<f32>(0.5, 1.0, -0.5));
    let ambient_light_intensity = 0.3;
    let diffuse_intensity = max(dot(normalize(in.normal), light_direction), 0.0);
    let brightness = ambient_light_intensity + (1.0 - ambient_light_intensity) * diffuse_intensity;

    let scrolled_uv = in.tex_coords + water.time * water.scroll_speed;
    let color = textureSample(water_texture, water_sampler, scrolled_uv);

    return vec4<f32>(color.rgb * brightness, 0.7);
}


struct CameraUniform {
    view_proj: mat4x4<f32>,
    skybox_projection: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct BlendUniform {
    // 4 instead of 3 to align to 16 bytes
    thresholds: vec4<f32>,
    blend_widths: vec4<f32>,
}
@group(0) @binding(2)
var<uniform> blend: BlendUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) height: f32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.height = model.position.y;
    return out;
}

@group(0) @binding(0)
var terrain_texture: texture_2d_array<f32>;
@group(0) @binding(1)
var terrain_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_direction = normalize(vec3<f32>(0.5, 1.0, -0.5));
    let ambient_light_intensity = 0.1;
    let diffuse_intensity = max(dot(normalize(in.normal), light_direction), 0.0);
    let brightness = ambient_light_intensity + (1.0 - ambient_light_intensity) * diffuse_intensity;

    var weights: array<f32, 4>;
    var prev_transition = 1.0;
    for (var i = 0u; i < 3u; i = i + 1u) {
        let curr_transition = smoothstep(
            blend.thresholds[i] - blend.blend_widths[i] * 0.5,
            blend.thresholds[i] + blend.blend_widths[i] * 0.5,
            in.height
        );
        weights[i] = prev_transition - curr_transition;
        prev_transition = curr_transition;
    }
    weights[3] = prev_transition;

    var color = vec4<f32>(0.0);
    for (var i = 0u; i < 4u; i = i + 1u) {
        color += textureSample(terrain_texture, terrain_sampler, in.tex_coords, i) * weights[i];
    }

    return vec4<f32>(color.rgb * brightness, color.a);
}


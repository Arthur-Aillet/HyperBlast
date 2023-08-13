struct Outline {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: Outline;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    #ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
    #ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(5) instance_index: u32,
    #endif
) -> @location(0) vec4<f32> {
    var true_uv = uv / .5 - 0.5;
    true_uv *= f32(uv.x > 8./32.);
    true_uv *= f32(uv.y > 8./32.);
    true_uv *= f32(uv.x < 24./32.);
    true_uv *= f32(uv.y < 24./32.);
    var true_color : vec4<f32> = textureSample(base_color_texture, base_color_sampler, true_uv);

    var selected_uv = uv / .5 - 0.5;
    selected_uv *= f32(uv.x > 7./32.);
    selected_uv *= f32(uv.y > 7./32.);
    selected_uv *= f32(uv.x < 25./32.);
    selected_uv *= f32(uv.y < 25./32.);

    var outside_uv = uv / .5 - 0.5;
    outside_uv *= f32(uv.x < 8./32. || uv.x > 24./32. || uv.y < 8./32. || uv.y > 24./32.);
    outside_uv *= f32(uv.x > 7./32.);
    outside_uv *= f32(uv.y > 7./32.);
    outside_uv *= f32(uv.x < 25./32.);
    outside_uv *= f32(uv.y < 25./32.);

    var color : vec4<f32> = textureSample(base_color_texture, base_color_sampler, selected_uv);
    var extend_color : vec4<f32> = textureSample(base_color_texture, base_color_sampler, outside_uv);
    var thick = 1./16.;

    var outline : f32 = textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(0.0,thick)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(0.0,-thick)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick,-thick)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick,thick)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick,thick)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick,-thick)).a;
    outline = min(outline, 1.0);

    let final_outline = (material.color * extend_color.a + material.color * outline - color);

    return mix(true_color, material.color, final_outline - true_color.a);
}

struct Outline {
    color: vec4<f32>,
    size: vec2<f32>,
    thickness: f32,
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
    true_uv *= f32(uv.x > 0.25);
    true_uv *= f32(uv.y > 0.25);
    true_uv *= f32(uv.x < 0.75);
    true_uv *= f32(uv.y < 0.75);
    var true_color : vec4<f32> = textureSample(base_color_texture, base_color_sampler, true_uv);

    var selected_uv = uv / .5 - 0.5;
    selected_uv *= f32(uv.x > (material.size.x * 2. * 0.25 - material.thickness) / (material.size.x * 2.));
    selected_uv *= f32(uv.y > (material.size.y * 2. * 0.25 - material.thickness) / (material.size.y * 2.));
    selected_uv *= f32(uv.x < (material.size.x * 2. * 0.75 + material.thickness) / (material.size.x * 2.));
    selected_uv *= f32(uv.y < (material.size.y * 2. * 0.75 + material.thickness) / (material.size.y * 2.));

    var thick_x = material.thickness/material.size.x;
    var thick_y = material.thickness/material.size.y;

    var outline : f32 = textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick_x,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick_x,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(0.0,thick_y)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(0.0,-thick_y)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick_x,-thick_y)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick_x,thick_y)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(thick_x,thick_y)).a;
    outline += textureSample(base_color_texture, base_color_sampler, selected_uv + vec2<f32>(-thick_x,-thick_y)).a;
    outline = min(outline, 1.0);

    var outside_uv = selected_uv * f32(uv.x < 0.25 || uv.x > 0.75 || uv.y < 0.25 || uv.y > 0.75);
    var outside_color : vec4<f32> = textureSample(base_color_texture, base_color_sampler, outside_uv);

    let final_outline = vec4(1., 1., 1., min(outside_color.a + outline - true_color.a, 1.));
    return mix(true_color, material.color, final_outline.a);
}

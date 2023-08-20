use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub struct OutlinePlugin;

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid, TypePath)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct Outline {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub thickness: f32,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Outline>::default());
    }
}

impl Material2d for Outline {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline.wgsl".into()
    }
}

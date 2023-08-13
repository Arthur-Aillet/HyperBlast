use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::rendering::{Zindex, Position};

pub struct ItemsPlugin;

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid, TypePath)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct Outline {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}


impl Material2d for Outline {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline.wgsl".into()
    }
}


impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ItemsAssets>()
            .add_plugins(Material2dPlugin::<Outline>::default())
            .add_systems(Startup, setup_item);
    }
}
fn setup_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Pickup {
        name: bevy::core::Name::new("Cheese"),
        material: MaterialMesh2dBundle {
            transform: Transform::default().with_scale(Vec3::splat(16.)),
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(2.)))).into(),
            material: materials.add(Outline {
                color: Color::WHITE,
                color_texture: asset_server.load("items/Item__65.png"),
            }),
            ..default()
        },
        zindex: Zindex(25.),
    });
}

#[derive(AssetCollection, Resource)]
pub struct ItemsAssets {
    #[asset(path = "items/Item__65.png")]
    pub cheese: Handle<Image>,
}

#[derive(Bundle)]
pub struct Pickup {
    pub name: bevy::core::Name,
    pub material: MaterialMesh2dBundle<Outline>,
    pub zindex: Zindex,
}

impl Pickup {
    pub fn setup(items: &Res<ItemsAssets>, materials: &mut ResMut<Assets<Outline>>) -> () {

    }
}

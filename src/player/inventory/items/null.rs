use crate::player::inventory::items_imports::*;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct Null {}

impl ItemTrait for Null {
}

pub fn create_null_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<ItemsAssets>
) -> PickupBundle {
    PickupBundle::create(meshes, materials,
        sprites.apple.clone(),
        Vec2::new(16., 16.),
        "apple".to_string(),
        pos
    )
}

pub fn create_null_item() -> Box<dyn ItemTrait> {
    Box::new(
        Null{}
    )
}

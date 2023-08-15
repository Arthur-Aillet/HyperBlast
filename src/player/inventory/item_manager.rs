
use bevy::prelude::*;

use std::fmt::Debug;

use crate::outline::Outline;
use super::{pickup::PickupBundle, assets::ItemsAssets};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Items {
    HealthApple,
}

impl Items {
    pub fn to_pickup(&self,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprites: &Res<ItemsAssets>
    ) -> PickupBundle {
        match self {
            Items::HealthApple => create_health_apple_pickup(pos, meshes, materials, sprites),
        }
    }
}

pub fn create_health_apple_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<ItemsAssets>
) -> PickupBundle {
    PickupBundle::create(meshes, materials,
        sprites.apple.clone(),
        Vec2::new(16., 16.),
        "apple".to_string(),
        pos,
        Items::HealthApple
    )
}

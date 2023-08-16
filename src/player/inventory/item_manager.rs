use bevy::prelude::*;

use std::fmt::Debug;

use strum_macros::EnumIter;

use super::{assets::ItemsAssets, pickup::PickupBundle};
use crate::outline::Outline;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum Items {
    HealthApple,
    Mercury,
}

impl Items {
    pub fn to_pickup(
        self,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprites: &Res<ItemsAssets>,
    ) -> PickupBundle {
        match self {
            Items::HealthApple => create_health_apple_pickup(pos, meshes, materials, sprites),
            Items::Mercury => create_mercury_pickup(pos, meshes, materials, sprites),
        }
    }
}

pub fn create_mercury_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<ItemsAssets>,
) -> PickupBundle {
    PickupBundle::create(
        meshes,
        materials,
        sprites.mercury.clone(),
        Vec2::new(16., 16.),
        "mercury".to_string(),
        pos,
        Items::Mercury,
    )
}

pub fn create_health_apple_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<ItemsAssets>,
) -> PickupBundle {
    PickupBundle::create(
        meshes,
        materials,
        sprites.apple.clone(),
        Vec2::new(16., 16.),
        "apple".to_string(),
        pos,
        Items::HealthApple,
    )
}

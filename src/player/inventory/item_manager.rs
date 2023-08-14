
use bevy::ecs::component::SparseStorage;
use bevy::prelude::*;

use std::fmt::Debug;

use crate::{Component, player::stats::PlayerStats, outline::Outline};
use super::{pickup::PickupBundle, items, assets::ItemsAssets};

pub type ShootUpgradeFn = fn() -> ();
pub type MoveUpgradeFn = fn() -> ();

pub trait ItemTrait: Component<Storage = SparseStorage> + Debug {
    fn get_shoot_function(&self) -> Option<ShootUpgradeFn> {
        None
    }

    fn get_move_function(&self) -> Option<MoveUpgradeFn> {
        None
    }

    fn get_modify_player_stats_function(&self) -> Option<fn(&mut PlayerStats)> {
        None
    }

    fn get_revert_player_stats_function(&self) -> Option<fn(&mut PlayerStats)>{
        None
    }

    fn get_value(&self) -> Option<f32>{
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Items {
    Null,
}

impl Items {
    pub fn to_item(&self) -> Box<dyn ItemTrait> {
        match self {
            Items::Null => items::null::create_null_item(),
        }
    }

    pub fn to_pickup(&self,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprites: &Res<ItemsAssets>
    ) -> PickupBundle {
        match self {
            Items::Null => items::null::create_null_pickup(pos, meshes, materials, sprites),
        }
    }
}

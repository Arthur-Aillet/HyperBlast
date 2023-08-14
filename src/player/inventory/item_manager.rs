
use bevy::ecs::component::SparseStorage;

use std::fmt::Debug;

use crate::{Component, player::stats::PlayerStats};
use super::items::null::Null;

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
    pub fn to_trait(&self) -> Box<dyn ItemTrait> {
        Box::new(match self {
            Items::Null => Null{},
        })
    }
}

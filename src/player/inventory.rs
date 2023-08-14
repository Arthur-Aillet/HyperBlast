use std::fmt::Debug;

use bevy::{prelude::Component, ecs::component::SparseStorage};

use crate::player::stats::PlayerStats;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct Null {}

impl ItemTrait for Null {
    fn get_shoot_function(&self) -> Option<ShootUpgradeFn> {
        None
    }

    fn get_move_function(&self) -> Option<MoveUpgradeFn> {
        None
    }

    fn get_modify_player_stats_function(&self) -> Option<fn(&mut PlayerStats)> {
        None
    }

    fn get_revert_player_stats_function(&self) -> Option<fn(&mut PlayerStats)> {
        None
    }

    fn get_value(&self) -> Option<f32> {
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

type ShootUpgradeFn = fn() -> ();
type MoveUpgradeFn = fn() -> ();

pub trait ItemTrait: Component<Storage = SparseStorage> + Debug {
    fn get_shoot_function(&self) -> Option<ShootUpgradeFn>;
    fn get_move_function(&self) -> Option<MoveUpgradeFn>;
    fn get_modify_player_stats_function(&self) -> Option<fn(&mut PlayerStats)>;
    fn get_revert_player_stats_function(&self) -> Option<fn(&mut PlayerStats)>;
    fn get_value(&self) -> Option<f32>;
}

#[derive(Debug)]
pub struct DmgUpStruct  {

}

impl DmgUpStruct {
    pub fn get_shoot_function(&self) -> Option<ShootUpgradeFn> {
        None
    }
}

#[derive(Component)]
pub struct Inventory {
    pub content: Vec<Box<dyn ItemTrait>>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &Items) {
        self.content.push(name.to_trait())
    }

    pub fn get_all_shooting_function(&self) -> Vec<ShootUpgradeFn> {
        let mut functions = Vec::new();

        for i in &self.content {
            if let Some(function) = i.get_shoot_function() {
                functions.push(function);
            }
        }
        functions
    }
}

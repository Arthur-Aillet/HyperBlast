use std::{collections::HashMap, fmt::Debug};

use bevy::{prelude::Component, ecs::component::SparseStorage};

use crate::player::stats::PlayerStats;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Items {
    DmgUp,
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
struct Inventory {
    pub content: HashMap<Items, Box<dyn ItemTrait>>,
}

impl Inventory {
    pub fn debug_print(&self) {
        for (key, content) in &self.content {
            println!("key: {key:?}\n content: {content:?}");
        }
    }

    pub fn get_all_shooting_function(&self) -> Vec<ShootUpgradeFn> {
        let mut functions = Vec::new();

        for i in self.content.values() {
            if let Some(function) = i.get_shoot_function() {
                functions.push(function);
            }
        }
        functions
    }
}

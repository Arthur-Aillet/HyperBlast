use bevy::prelude::*;

use crate::player::inventory::item_manager::{ShootUpgradeFn, ItemTrait, Items};

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

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{player::{inventory::item_manager::{ShootUpgradeFn, ItemTrait, Items}, input::PlayerActions}, rendering::Position, outline::Outline};

use super::assets::ItemsAssets;

pub fn drop_item (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    sprites: Res<ItemsAssets>,
    mut query: Query<(
        &ActionState<PlayerActions>,
        &Position,
        &mut Inventory,
    )>
) {
    for (action, pos, mut inventory) in &mut query {
        if action.just_pressed(PlayerActions::Drop) {
            if let Some(item) = inventory.content.pop() {
                commands.spawn(item.0.to_pickup(pos.0, &mut meshes, &mut materials, &sprites));
            }
        }
    }
}

#[derive(Component)]
pub struct Inventory {
    pub content: Vec<(Items, Box<dyn ItemTrait>)>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &Items) {
        self.content.push((name.clone(), name.to_item()))
    }

    pub fn get_all_shooting_function(&self) -> Vec<ShootUpgradeFn> {
        let mut functions = Vec::new();

        for (_, function) in &self.content {
            if let Some(function) = function.get_shoot_function() {
                functions.push(function);
            }
        }
        functions
    }
}

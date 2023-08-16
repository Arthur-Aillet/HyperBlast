use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;

use crate::{player::{inventory::item_manager::Items, input::PlayerActions, stats::PlayerStats}, rendering::Position, outline::Outline};

use super::{assets::ItemsAssets, DroppedEvent};

pub fn drop_item (
    mut commands: Commands,
    mut ev_drop: EventWriter<DroppedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    sprites: Res<ItemsAssets>,
    mut query: Query<(
        Entity,
        &ActionState<PlayerActions>,
        &Position,
        &mut PlayerStats,
        &mut Inventory,
    )>
) {
    for (entity, action,pos, mut stats, mut inventory) in &mut query {
        if action.just_pressed(PlayerActions::Drop) {
            if let Some(item) = inventory.content.pop() {
                ev_drop.send(DroppedEvent(item, entity));
                commands.spawn(item.to_pickup(pos.0, &mut meshes, &mut materials, &sprites));
            }
        }
    }
}

#[derive(Component)]
pub struct Inventory {
    pub content: Vec<Items>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, name: Items) {
        self.content.push(name);
    }

    pub fn amount(&mut self, name: Items) -> usize {
        self.content.iter().filter(|&n| *n == name).count()
    }
}
use bevy::{math::Vec3Swizzles, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    player::{input::PlayerActions, inventory::item_manager::Items},
    rendering::outline::Outline,
};

use super::{assets::ItemsAssets, DroppedItemEvent, pickup::Ground};

pub fn drop_item(
    mut commands: Commands,
    mut ev_drop: EventWriter<DroppedItemEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    sprites: Res<ItemsAssets>,
    ground: Query<(Entity, With<Ground>)>,
    mut query: Query<(
        Entity,
        &ActionState<PlayerActions>,
        &Transform,
        &mut Inventory,
    )>,
) {
    for (entity, action, pos, mut inventory) in &mut query {
        if action.just_pressed(PlayerActions::DropItem) {
            if let Some(item) = inventory.content.pop() {
                ev_drop.send(DroppedItemEvent(item, entity));
                let id_new = commands.spawn(item.to_pickup(
                    pos.translation.xy(),
                    &mut meshes,
                    &mut materials,
                    &sprites,
                )).id();
                let ground_id = ground.single().0;
                commands.entity(ground_id).add_child(id_new);
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

    pub fn amount(&self, name: Items) -> usize {
        self.content.iter().filter(|&n| *n == name).count()
    }
}

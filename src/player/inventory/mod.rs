pub mod armory_manager;
pub mod assets;
pub mod inventory_manager;
pub mod item_manager;
pub mod pickup;
pub mod stats;
pub mod weapon_manager;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::{
    armory_manager::{pickup_weapon, drop_weapon, switch_weapon},
    assets::ItemsAssets,
    inventory_manager::drop_item,
    item_manager::Items,
    pickup::{spawn_items, update_pickup},
    stats::{drop_events, pickup_events},
};

#[derive(Event)]
pub struct PickupItemEvent(Items, Entity);

#[derive(Event)]
pub struct DroppedItemEvent(Items, Entity);

#[derive(Event)]
pub struct PickupWeaponEvent(Entity, Entity); // player then pickup entity

#[derive(Event)]
pub struct DroppedWeaponEvent(Entity, Entity);

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ItemsAssets>()
            .add_event::<PickupItemEvent>()
            .add_event::<DroppedItemEvent>()
            .add_event::<PickupWeaponEvent>()
            .add_event::<DroppedWeaponEvent>()
            .add_systems(Startup, spawn_items)
            .add_systems(Update, update_pickup)
            .add_systems(Update, drop_item)
            .add_systems(Update, pickup_events)
            .add_systems(Update, pickup_weapon)
            .add_systems(Update, drop_weapon)
            .add_systems(Update, switch_weapon)
            .add_systems(Update, drop_events);
    }
}

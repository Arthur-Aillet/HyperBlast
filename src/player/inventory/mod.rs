pub mod item_manager;
pub mod inventory_manager;
pub mod pickup;
pub mod assets;
mod stats;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::{assets::ItemsAssets, pickup::{spawn_items, update_pickup}, inventory_manager::drop_item, item_manager::Items, stats::{pickup_events, drop_events}};

#[derive(Event)]
pub struct PickupEvent(Items, Entity);

#[derive(Event)]
pub struct DroppedEvent(Items, Entity);

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_collection::<ItemsAssets>()
            .add_event::<PickupEvent>()
            .add_event::<DroppedEvent>()
            .add_systems(Startup, spawn_items)
            .add_systems(Update, update_pickup)
            .add_systems(Update, drop_item)
            .add_systems(Update, pickup_events)
            .add_systems(Update, drop_events);
    }
}

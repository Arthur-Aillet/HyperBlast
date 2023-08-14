pub mod items;
pub mod item_manager;
pub mod inventory_manager;
pub mod pickup;
pub mod assets;
pub mod items_imports;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::{assets::ItemsAssets, pickup::{spawn_items, update_pickup}, inventory_manager::drop_item};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ItemsAssets>()
            .add_systems(Startup, spawn_items)
            .add_systems(Update, update_pickup)
            .add_systems(Update, drop_item);
    }
}

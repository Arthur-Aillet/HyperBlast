use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::{assets::ItemsAssets, pickup::{setup_item, update_pickup}};

pub mod items;
pub mod item_manager;
pub mod inventory_manager;
pub mod pickup;
pub mod assets;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ItemsAssets>()
            .add_systems(Startup, setup_item)
            .add_systems(Update, update_pickup);
    }
}

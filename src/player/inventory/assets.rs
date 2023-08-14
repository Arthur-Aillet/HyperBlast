use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ItemsAssets {
    #[asset(path = "items/Item__65.png")]
    pub cheese: Handle<Image>,
    #[asset(path = "items/Item__64.png")]
    pub apple: Handle<Image>,
    #[asset(path = "items/Item__63.png")]
    pub item: Handle<Image>,
}

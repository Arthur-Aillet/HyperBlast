use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GunAssets {
    #[asset(path = "guns/marine_gun.png")]
    pub marine: Handle<Image>,
    #[asset(path = "guns/bullet.png")]
    pub marine_bullet: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 25., columns = 4, rows = 1, padding_x = 2., padding_y = 2., offset_x = 15., offset_y = 15.))]
    #[asset(path = "idle.png")]
    pub idle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 25., columns = 6, rows = 1, padding_x = 2., padding_y = 2., offset_x = 15., offset_y = 15.))]
    #[asset(path = "run.png")]
    pub front: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 25., columns = 6, rows = 1, padding_x = 2., padding_y = 2., offset_x = 15., offset_y = 42.))]
    #[asset(path = "run.png")]
    pub side_front: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 25., columns = 6, rows = 1, padding_x = 2., padding_y = 2., offset_x = 15., offset_y = 96.))]
    #[asset(path = "run.png")]
    pub side_back: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 17., tile_size_y = 25., columns = 6, rows = 1, padding_x = 2., padding_y = 2., offset_x = 15., offset_y = 69.))]
    #[asset(path = "run.png")]
    pub back: Handle<TextureAtlas>,
    #[asset(path = "collider.png")]
    pub collider: Handle<Image>,
}

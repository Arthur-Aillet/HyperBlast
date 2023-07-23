use bevy::prelude::*;

use crate::rendering::Position;

#[derive(Bundle, Default)]
pub struct GunBundle {
    pub sprite: SpriteBundle,
    pub pos: Position,
}

impl GunBundle {
    pub fn setup(asset_server: &Res<AssetServer>) -> Self {
        GunBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("marine_gun.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

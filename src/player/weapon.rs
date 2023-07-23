use bevy::prelude::*;

use crate::rendering::{Position, Angle, ZIndex, Offset};

#[derive(Component)]
pub struct GunStats {

}

#[derive(Bundle)]
pub struct GunBundle {
    pub name: Name,
    pub stats: GunStats,
    pub sprite: SpriteBundle,
    pub pos: Position,
    pub angle: Angle,
    pub zindex: ZIndex,
}

#[derive(Component)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(asset_server: &Res<AssetServer>) -> Self {
        GunBundle {
            name: Name::new("Gun"),
            stats: GunStats{},
            sprite: SpriteBundle {
                texture: asset_server.load("marine_gun.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Custom(Vec2 { x: -((10./14.)*0.5), y: -((5./9.)*0.5) }),
                    ..default()
                },
                ..default()
            },
            angle: Angle(0.),
            zindex: ZIndex(50.),
            pos: Position(Vec2::ZERO)
        }
    }
}

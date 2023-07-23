use bevy::prelude::*;

use crate::{
    animations::AnimationFlip,
    rendering::{Angle, Offset, Position, Size, ZIndex},
};

#[derive(Component)]
pub struct GunStats {}

#[derive(Bundle)]
pub struct GunBundle {
    pub name: Name,
    pub stats: GunStats,
    pub sprite: SpriteBundle,
    pub pos: Position,
    pub angle: Angle,
    pub zindex: ZIndex,
    pub flip: AnimationFlip,
    pub offset: Offset,
    pub size: Size,
}

#[derive(Component)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(asset_server: &Res<AssetServer>) -> Self {
        GunBundle {
            name: Name::new("Gun"),
            stats: GunStats {},
            sprite: SpriteBundle {
                texture: asset_server.load("marine_gun.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            offset: Offset(Vec2::new(2., 2.)),
            size: Size(Vec2::new(14., 9.)),
            angle: Angle(0.),
            zindex: ZIndex(50.),
            pos: Position(Vec2::ZERO),
            flip: AnimationFlip::False,
        }
    }
}

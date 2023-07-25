use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    animations::AnimationFlip,
    rendering::{Angle, Offset, Position, Size, Zindex}, player::bullets::BulletBundle,
};

use super::stats::PlayerStats;

type ShootFn = fn(&mut Commands, &Res<AssetServer>, &mut GunStats, &mut PlayerStats, Vec2, f32, Entity);

#[derive(Component)]
pub struct GunStats {
    pub handle_position: Vec2,
    pub barrel_length: f32,
    pub barrel_height: f32,
    pub shoot: ShootFn,
    pub timer: Stopwatch,
}

#[derive(Bundle)]
pub struct GunBundle {
    pub name: Name,
    pub stats: GunStats,
    pub sprite: SpriteBundle,
    pub pos: Position,
    pub angle: Angle,
    pub zindex: Zindex,
    pub flip: AnimationFlip,
    pub offset: Offset,
    pub size: Size,
}

#[derive(Component)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(asset_server: &Res<AssetServer>) -> Self {
        let mut stats = GunStats {
            handle_position: Vec2::new(2., 2.),
            barrel_length: 12.,
            barrel_height: 5.5,
            timer: Stopwatch::new(),
            shoot: basic_shoot_fn,
        };
        stats.timer.set_elapsed(Duration::new(1, 0));
        GunBundle {
            name: Name::new("Gun"),
            offset: Offset(stats.handle_position.clone()),
            stats,
            sprite: SpriteBundle {
                texture: asset_server.load("marine_gun.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            size: Size(Vec2::new(14., 9.)),
            angle: Angle(0.),
            zindex: Zindex(50.),
            pos: Position(Vec2::ZERO),
            flip: AnimationFlip::False,
        }
    }
}

pub fn basic_shoot_fn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    stats: &mut GunStats,
    _player: &mut PlayerStats,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
) {
    if stats.timer.elapsed_secs() >= 1. {
        stats.timer.reset();
        commands.spawn(BulletBundle::marine_bullet(asset_server, barrel_end, angle, owner));
    }
}


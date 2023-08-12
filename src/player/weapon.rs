use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{
    player::bullets::BulletBundle,
    rendering::{Angle, Flip, Offset, Position, Size, Zindex},
};

use super::{assets::GunAssets, stats::PlayerStats};

type ShootFn =
    fn(&mut Commands, &Res<GunAssets>, &mut GunStats, &mut PlayerStats, Vec2, f32, Entity);

#[derive(Component)]
pub struct GunStats {
    pub handle_position: Vec2,
    pub barrel_length: f32,
    pub barrel_height: f32,
    pub spread: f32,
    pub salve: i32,
    pub shoot: ShootFn,
    pub timer: Stopwatch,
    pub damage: f32,
    pub ammo: i32,
    pub max_ammo: i32,
    pub mag_ammo: i32,
    pub mag_size: i32,
    pub reload_time: f32,
    pub fire_rate: f32,
}

#[derive(Bundle)]
pub struct GunBundle {
    pub name: Name,
    pub stats: GunStats,
    pub sprite: SpriteBundle,
    pub pos: Position,
    pub angle: Angle,
    pub zindex: Zindex,
    pub flip: Flip,
    pub offset: Offset,
    pub size: Size,
}

#[derive(Component)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(guns: &Res<GunAssets>) -> Self {
        let mut stats = GunStats {
            handle_position: Vec2::new(2., 2.),
            barrel_length: 12.,
            barrel_height: 5.5,
            timer: Stopwatch::new(),
            shoot: basic_shoot_fn,
            damage: 20.,
            spread: (10_f32).to_radians(),
            salve: 1,
            ammo: 100,
            max_ammo: 100,
            mag_ammo: 10,
            mag_size: 10,
            reload_time: 2.5,
            fire_rate: 4.,
        };
        stats.timer.set_elapsed(Duration::new(1, 0));
        GunBundle {
            name: Name::new("Gun"),
            offset: Offset(stats.handle_position),
            stats,
            sprite: SpriteBundle {
                texture: guns.marine.clone(),
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
            flip: Flip::False,
        }
    }
}

pub fn basic_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    _player: &mut PlayerStats,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
) {
    if stats.mag_ammo > 0 {
        if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            let to_fire = if stats.salve > stats.mag_ammo {stats.mag_ammo} else {stats.salve};

            for _ in 0..to_fire {
                commands.spawn(BulletBundle::marine_bullet(
                    assets,
                    barrel_end,
                    angle + rng.gen_range((stats.spread * -1.)..stats.spread),
                    owner,
                ));
            }
            stats.mag_ammo -= to_fire;
        }
    } else if stats.timer.elapsed_secs() >= stats.reload_time {
            stats.timer.reset();
            if stats.ammo < stats.mag_size {
                stats.mag_ammo = stats.ammo;
                stats.ammo = 0;
            } else {
                stats.mag_ammo = stats.mag_size;
                stats.ammo -= stats.mag_size;
            }
        }
}

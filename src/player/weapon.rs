use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{
    player::bullets::BulletBundle,
    rendering::{Angle, Flip, Offset, Position, Size, Zindex},
    player::reload::ReloadStats,
    player::roll::RollStats,
};


use super::{assets::GunAssets, stats::PlayerStats};

type ShootFn =
    fn(&mut Commands, &Res<GunAssets>, &mut GunStats, &mut PlayerStats, Vec2, f32, Entity);

pub type ReloadFn =
    fn(&Res<Time>, &mut Commands, Mut<'_, Angle>, Mut<'_, GunStats>, Mut<'_, PlayerStats>, Mut<'_, ReloadStats>, Option<&RollStats>, Entity);

#[derive(Component)]
pub struct GunStats {
    pub handle_position: Vec2,
    pub barrel_length: f32,
    pub barrel_height: f32,
    pub spread: f32,
    pub speed: f32,
    pub speed_spread: f32,
    pub distance: f32,
    pub salve: i32,
    pub shoot: ShootFn,
    pub reload: ReloadFn,
    pub timer: Stopwatch,
    pub damage: f32,
    pub ammo: i32,
    pub max_ammo: i32,
    pub infinite: bool,
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

pub fn revolver_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: basic_shoot_fn,
        reload: basic_reload_fn,
        damage: 10.,
        spread: (5_f32).to_radians(),
        speed: 90.,
        speed_spread: 1.,
        distance: 80.,
        salve: 1,
        ammo: 0,
        max_ammo: 0,
        infinite: true,
        mag_ammo: 6,
        mag_size: 6,
        reload_time: 2.5,
        fire_rate: 1.5,
    }
}

pub fn shotgun_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: basic_shoot_fn,
        reload: basic_reload_fn,
        damage: 6.,
        spread: (20_f32).to_radians(),
        speed: 190.,
        speed_spread: 10.,
        distance: 50.,
        salve: 8,
        ammo: 18,
        max_ammo: 30,
        infinite: false,
        mag_ammo: 6,
        mag_size: 6,
        reload_time: 0.5,
        fire_rate: 1.,
    }
}

pub fn sniper_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: basic_shoot_fn,
        reload: basic_reload_fn,
        damage: 100.,
        spread: (0_f32).to_radians(),
        speed: 1000.,
        speed_spread: 0.,
        distance: 1000.,
        salve: 1,
        ammo: 10,
        max_ammo: 10,
        infinite: false,
        mag_ammo: 1,
        mag_size: 1,
        reload_time: 5.,
        fire_rate: 2.,
    }
}

pub fn automatic_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: basic_shoot_fn,
        reload: basic_reload_fn,
        damage: 15.,
        spread: (10_f32).to_radians(),
        speed: 90.,
        speed_spread: 5.,
        distance: 80.,
        salve: 1,
        ammo: 200,
        max_ammo: 200,
        infinite: false,
        mag_ammo: 20,
        mag_size: 20,
        reload_time: 2.,
        fire_rate: 5.,
    }
}

pub fn flamethrower_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: basic_shoot_fn,
        reload: basic_reload_fn,
        damage: 5.,
        spread: (10_f32).to_radians(),
        speed: 60.,
        speed_spread: 40.,
        distance: 40.,
        salve: 3,
        ammo: 900,
        max_ammo: 900,
        infinite: false,
        mag_ammo: 900,
        mag_size: 900,
        reload_time: 5.,
        fire_rate: 30.,
    }
}

#[derive(Component, Clone)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(guns: &Res<GunAssets>) -> Self {
        let mut stats = revolver_stats();
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

pub fn basic_reload_fn(
    time: &Res<Time>,
    commands: &mut Commands,
    mut angle: Mut<'_, Angle>,
    mut stats: Mut<'_, GunStats>,
    _player: Mut<'_, PlayerStats>,
    mut reload_stats: Mut<'_, ReloadStats>,
    roll: Option<&RollStats>,
    owner: Entity,
) {
    if roll.is_some() && !reload_stats.since.paused() { reload_stats.since.pause(); }
    if roll.is_none() && reload_stats.since.paused() { reload_stats.since.unpause(); }
    reload_stats.since.tick(time.delta());
    if reload_stats.since.elapsed_secs() >= stats.reload_time {
        if stats.infinite {
            stats.mag_ammo = stats.mag_size;
        } else if stats.ammo < (stats.mag_size - stats.mag_ammo) {
            stats.mag_ammo += stats.ammo;
            stats.ammo = 0;
        } else {
            stats.ammo -= stats.mag_size - stats.mag_ammo;
            stats.mag_ammo = stats.mag_size;
        }
    commands.entity(owner).remove::<ReloadStats>();
    } else {
        angle.0 += reload_stats.since.elapsed_secs() / stats.reload_time * 12.;
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

            for i in 0..stats.salve {
                commands.spawn(BulletBundle::marine_bullet(
                    assets,
                    barrel_end,
                    angle + (if stats.spread == 0. {0.} else {rng.gen_range((stats.spread * -1.)..stats.spread)}),
                    owner,
                    stats.speed + (if stats.spread == 0. {0.} else {rng.gen_range((stats.speed_spread * -1.)..stats.speed_spread)}),
                    stats.distance,
                ));
            }
            stats.mag_ammo -= 1;
        }
    }
}

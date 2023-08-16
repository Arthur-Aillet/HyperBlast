use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;
use leafwing_input_manager::prelude::*;

use crate::{
    player::bullets::BulletBundle,
    rendering::{Angle, Flip, Offset, Position, Size, Zindex},
    player::reload::ReloadStats,
    player::roll::RollStats,
    player::input::PlayerActions,
};


use super::{assets::GunAssets, stats::PlayerStats};

type ShootFn =
    fn(&mut Commands, &Res<GunAssets>, &mut GunStats, &mut PlayerStats, Vec2, f32, Entity, &ActionState<PlayerActions>, Option<&RollStats>, Option<&ReloadStats>);

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
    pub min_shot: i32,
    pub left_to_fire: i32,
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
    pub sub_fire_rate: f32,
}

impl Default for GunStats {
    fn default() -> Self {
        GunStats {
            handle_position: Vec2::new(2., 2.),
            barrel_length: 12.,
            barrel_height: 5.5,
            timer: Stopwatch::new(),
            shoot: auto_shoot_fn,
            reload: basic_reload_fn,
            damage: 10.,
            spread: (0_f32).to_radians(),
            speed: 90.,
            speed_spread: 0.,
            distance: 80.,
            salve: 1,
            min_shot: 1,
            left_to_fire: 0,
            ammo: 100,
            max_ammo: 100,
            infinite: false,
            mag_ammo: 10,
            mag_size: 10,
            reload_time: 2.5,
            fire_rate: 1.5,
            sub_fire_rate: 10.,
        }
    }
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
        shoot: auto_shoot_fn,
        reload: basic_reload_fn,
        damage: 10.,
        spread: (5_f32).to_radians(),
        speed: 90.,
        speed_spread: 1.,
        distance: 80.,
        ammo: 0,
        max_ammo: 0,
        infinite: true,
        mag_ammo: 6,
        mag_size: 6,
        reload_time: 2.5,
        fire_rate: 1.5,
        ..Default::default()
    }
}

pub fn shotgun_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: manual_shoot_fn,
        reload: shotgun_reload_fn,
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
        ..Default::default()
    }
}

pub fn sniper_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: manual_shoot_fn,
        reload: basic_reload_fn,
        damage: 100.,
        speed: 1000.,
        distance: 1000.,
        ammo: 4,
        max_ammo: 4,
        infinite: false,
        mag_ammo: 2,
        mag_size: 2,
        reload_time: 5.,
        fire_rate: 2.,
        ..Default::default()
    }
}

pub fn semi_automatic_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: manual_shoot_fn,
        reload: basic_reload_fn,
        damage: 15.,
        spread: (10_f32).to_radians(),
        speed: 90.,
        speed_spread: 5.,
        distance: 80.,
        min_shot: 3,
        ammo: 200,
        max_ammo: 200,
        infinite: false,
        mag_ammo: 20,
        mag_size: 20,
        reload_time: 2.,
        fire_rate: 2.,
        sub_fire_rate: 10.,
        ..Default::default()
    }
}

pub fn automatic_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: auto_shoot_fn,
        reload: basic_reload_fn,
        damage: 15.,
        spread: (10_f32).to_radians(),
        speed: 90.,
        speed_spread: 5.,
        distance: 80.,
        ammo: 200,
        max_ammo: 200,
        infinite: false,
        mag_ammo: 20,
        mag_size: 20,
        reload_time: 2.,
        fire_rate: 5.,
        ..Default::default()
    }
}

pub fn flamethrower_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
        shoot: auto_shoot_fn,
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
        ..Default::default()
    }
}

#[derive(Component, Clone)]
pub struct GunEntity(pub Entity);

impl GunBundle {
    pub fn setup(guns: &Res<GunAssets>) -> Self {
        let mut stats = semi_automatic_stats();
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

pub fn shotgun_reload_fn(
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
            stats.mag_ammo += 1;
        } else {
            stats.ammo -= 1;
            stats.mag_ammo += 1;
        }
    commands.entity(owner).remove::<ReloadStats>();
    } else {
        angle.0 += reload_stats.since.elapsed_secs() / stats.reload_time * 12.;
    }
}

pub fn manual_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    _player: &mut PlayerStats,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>
) {
    if roll.is_some() || reload.is_some() {stats.left_to_fire = 0};
    if player_actions.just_pressed(PlayerActions::Shoot) && roll.is_none() && reload.is_none() && stats.left_to_fire == 0 {
        if stats.mag_ammo > 0 {
            if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
                stats.timer.reset();
                stats.left_to_fire = if stats.min_shot < stats.mag_ammo {stats.min_shot} else {stats.mag_ammo};
            }
        }
    }
    if stats.left_to_fire != 0 {
        if stats.left_to_fire == stats.min_shot || stats.timer.elapsed_secs() >= 1. / stats.sub_fire_rate {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            for _ in 0..stats.salve {
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
            stats.left_to_fire -= 1;
        }
    }
}

pub fn auto_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    _player: &mut PlayerStats,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>
) {
    if roll.is_some() || reload.is_some() {stats.left_to_fire = 0};
    if player_actions.pressed(PlayerActions::Shoot) && roll.is_none() && reload.is_none() && stats.left_to_fire == 0 {
        if stats.mag_ammo > 0 {
            if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
                stats.timer.reset();
                stats.left_to_fire = if stats.min_shot < stats.mag_ammo {stats.min_shot} else {stats.mag_ammo};
            }
        }
    }
    if stats.left_to_fire != 0 {
        if stats.left_to_fire == stats.min_shot || stats.timer.elapsed_secs() >= 1. / stats.sub_fire_rate {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            for _ in 0..stats.salve {
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
            stats.left_to_fire -= 1;
        }
    }
}
use bevy::{prelude::*, time::Stopwatch};
use leafwing_input_manager::prelude::*;
use rand::Rng;

use crate::{
    player::bullets::BulletBundle, player::input::PlayerActions, player::reload::ReloadStats,
    player::roll::RollStats, rendering::utils::Angle,
};

use super::inventory::weapon_manager::GunAssets;
use super::{inventory::inventory_manager::Inventory, stats::PlayerStats};

type ShootFn = fn(
    &mut Commands,
    &Res<GunAssets>,
    &mut GunStats,
    &mut PlayerStats,
    &Inventory,
    Vec2,
    f32,
    Entity,
    &ActionState<PlayerActions>,
    Option<&RollStats>,
    Option<&ReloadStats>,
);

pub type ReloadFn = fn(
    &Res<Time>,
    &mut Commands,
    Mut<'_, Angle>,
    Mut<'_, GunStats>,
    Mut<'_, PlayerStats>,
    Mut<'_, ReloadStats>,
    Option<&RollStats>,
    Entity,
);

#[derive(Component)]
pub struct GunStats {
    pub handle_position: Vec2,
    pub size: Vec2,
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
    pub broken: bool,
    pub heat: f32,
    pub min_heat: f32,
    pub max_heat: f32,
}

impl Default for GunStats {
    fn default() -> Self {
        GunStats {
            handle_position: Vec2::new(2., 2.),
            size: Vec2::new(14., 9.),
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
            broken: false,
            heat: 0.,
            min_heat: 0.,
            max_heat: 20.,
        }
    }
}

#[derive(Component, Clone, Reflect)]
pub struct GunEntity(pub Entity);

//=============================================
//============================ reload functions
//=============================================

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
    if roll.is_some() && !reload_stats.since.paused() {
        reload_stats.since.pause();
    }
    if roll.is_none() && reload_stats.since.paused() {
        reload_stats.since.unpause();
    }
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
        stats.broken = false;
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
    if roll.is_some() && !reload_stats.since.paused() {
        reload_stats.since.pause();
    }
    if roll.is_none() && reload_stats.since.paused() {
        reload_stats.since.unpause();
    }
    reload_stats.since.tick(time.delta());
    if reload_stats.since.elapsed_secs() >= stats.reload_time {
        if stats.infinite {
            stats.mag_ammo += 1;
        } else {
            stats.ammo -= 1;
            stats.mag_ammo += 1;
        }
        stats.broken = false;
        commands.entity(owner).remove::<ReloadStats>();
    } else {
        angle.0 += reload_stats.since.elapsed_secs() / stats.reload_time * 12.;
    }
}

pub fn no_reload(
    _tim: &Res<Time>,
    commands: &mut Commands,
    mut angle: Mut<'_, Angle>,
    stats: Mut<'_, GunStats>,
    _player: Mut<'_, PlayerStats>,
    _reload_stats: Mut<'_, ReloadStats>,
    _roll: Option<&RollStats>,
    owner: Entity,
) {
    angle.0 += stats.heat * 100.;
    commands.entity(owner).remove::<ReloadStats>();
}

//=============================================
//============================ shoot functions
//=============================================

pub fn manual_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    player: &mut PlayerStats,
    inventory: &Inventory,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>,
) {
    if roll.is_some() || reload.is_some() || stats.broken {
        stats.left_to_fire = 0
    };
    if player_actions.just_pressed(PlayerActions::Shoot)
        && roll.is_none()
        && reload.is_none()
        && stats.left_to_fire == 0
        && !stats.broken
        && stats.mag_ammo > 0
        && stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
        stats.timer.reset();
        stats.left_to_fire = if stats.min_shot < stats.mag_ammo {
                                stats.min_shot
                            } else {
                                stats.mag_ammo
        };
    }
    if stats.left_to_fire > 0 && (stats.left_to_fire == stats.min_shot
            || stats.timer.elapsed_secs() >= 1. / stats.sub_fire_rate)
        {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            for _ in 0..stats.salve {
                commands.spawn(BulletBundle::marine_bullet(
                    assets,
                    barrel_end,
                    angle + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.spread * -1.)..stats.spread)
                        }),
                    inventory,
                    stats,
                    player,
                    owner,
                    stats.speed + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.speed_spread * -1.)..stats.speed_spread)
                        }),
                    stats.distance,
                    stats.damage,
                ));
            }
        stats.mag_ammo -= 1;
        stats.left_to_fire -= 1;
    }
}

pub fn auto_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    player: &mut PlayerStats,
    inventory: &Inventory,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>,
) {
    if roll.is_some() || reload.is_some() || stats.broken {
        stats.left_to_fire = 0
    };
    if player_actions.pressed(PlayerActions::Shoot)
        && roll.is_none()
        && reload.is_none()
        && stats.left_to_fire == 0
        && !stats.broken
    {
        if stats.mag_ammo > 0 {
            if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
                stats.timer.reset();
                stats.left_to_fire = if stats.min_shot < stats.mag_ammo {
                    stats.min_shot
                } else {
                    stats.mag_ammo
                };
            }
        }
    }
    if stats.left_to_fire != 0 {
        if stats.left_to_fire == stats.min_shot
            || stats.timer.elapsed_secs() >= 1. / stats.sub_fire_rate
        {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            for _ in 0..stats.salve {
                commands.spawn(BulletBundle::marine_bullet(
                    assets,
                    barrel_end,
                    angle
                        + (if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.spread * -1.)..stats.spread)
                        }),
                    inventory,
                    &stats,
                    player,
                    owner,
                    stats.speed + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.speed_spread * -1.)..stats.speed_spread)
                        }),
                    stats.distance,
                    stats.damage,
                ));
            }
            stats.mag_ammo -= 1;
            stats.left_to_fire -= 1;
        }
    }
}

pub fn charging_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    player: &mut PlayerStats,
    inventory: &Inventory,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>
) {
    if stats.broken {
        if stats.heat > 0. {
            stats.heat -= stats.timer.elapsed_secs();
            stats.timer.reset();
        }
        if stats.heat <= 0. {
            stats.heat = 0.;
            stats.broken = false;
        }
    }
    if player_actions.pressed(PlayerActions::Shoot) && roll.is_none() && reload.is_none() && stats.left_to_fire == 0 && !stats.broken {
        if stats.mag_ammo > 0 {
            if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
                stats.heat += stats.timer.elapsed_secs();
                stats.timer.reset();
                if stats.heat > stats.max_heat {stats.broken = true};
            }
        }
    }
    if player_actions.released(PlayerActions::Shoot) && stats.heat >= stats.min_heat && reload.is_none() && roll.is_none() && !stats.broken && stats.left_to_fire == 0 {
        if stats.heat >= stats.min_heat {
            stats.left_to_fire = stats.min_shot;
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
                    angle + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range(((stats.spread / stats.heat.log2()) * -1.)..(stats.spread / stats.heat.log2()))
                        }),
                    inventory,
                    stats,
                    player,
                    owner,
                    (stats.speed + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range(((stats.speed_spread / stats.heat.log2()) * -1.)..(stats.speed_spread / stats.heat.log2()))
                        })) * stats.heat.log2(),
                    stats.distance * stats.heat,
                    stats.damage * stats.heat.log2(),
                ));
            }
            stats.mag_ammo -= 1;
            stats.left_to_fire -= 1;
            if stats.left_to_fire == 0 {
                stats.heat = 0.;
            }
        }
    }
}

pub fn overheat_shoot_fn(
    commands: &mut Commands,
    assets: &Res<GunAssets>,
    stats: &mut GunStats,
    player: &mut PlayerStats,
    inventory: &Inventory,
    barrel_end: Vec2,
    angle: f32,
    owner: Entity,
    player_actions: &ActionState<PlayerActions>,
    roll: Option<&RollStats>,
    reload: Option<&ReloadStats>,
) {
    if roll.is_some() || reload.is_some() || stats.broken {
        stats.left_to_fire = 0
    };
    if stats.broken {
        if stats.heat > 0. {
            stats.heat -= stats.timer.elapsed_secs();
            stats.timer.reset();
        }
        if stats.heat <= 0. {
            stats.heat = 0.;
            stats.broken = false;
        }
    }
    if player_actions.pressed(PlayerActions::Shoot)
        && roll.is_none()
        && reload.is_none()
        && stats.left_to_fire == 0
        && !stats.broken
    {
        if stats.mag_ammo > 0 {
            if stats.timer.elapsed_secs() >= 1. / stats.fire_rate {
                stats.heat += stats.timer.elapsed_secs();
                stats.timer.reset();
                if stats.heat >= stats.min_heat && stats.heat < stats.max_heat {
                    stats.left_to_fire = if stats.min_shot < stats.mag_ammo {
                        stats.min_shot
                    } else {
                        stats.mag_ammo
                    };
                }
                if stats.heat > stats.max_heat {
                    stats.broken = true
                };
            }
        }
    }
    if !player_actions.pressed(PlayerActions::Shoot) {
        if stats.heat > 0. {
            stats.heat -= stats.timer.elapsed_secs();
            stats.timer.reset();
        }
        if stats.heat < 0. {
            stats.heat = 0.
        }
    }
    if stats.left_to_fire != 0 {
        if stats.left_to_fire == stats.min_shot
            || stats.timer.elapsed_secs() >= 1. / stats.sub_fire_rate
        {
            stats.timer.reset();
            let mut rng = rand::thread_rng();
            for _ in 0..stats.salve {
                commands.spawn(BulletBundle::marine_bullet(
                    assets,
                    barrel_end,
                    angle
                        + (if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.spread * -1.)..stats.spread)
                        }),
                    inventory,
                    &stats,
                    player,
                    owner,
                    stats.speed + (
                        if stats.spread == 0. {
                            0.
                        } else {
                            rng.gen_range((stats.speed_spread * -1.)..stats.speed_spread)
                        }),
                    stats.distance,
                    stats.damage,
                ));
            }
            stats.mag_ammo -= 1;
            stats.left_to_fire -= 1;
        }
    }
}

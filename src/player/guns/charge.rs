use std::{time::Duration, f32::INFINITY};

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{charging_shoot_fn, basic_reload_fn, GunStats},
    },
    rendering::outline::Outline,
};

pub fn charged_stats() -> GunStats {
    GunStats {
        ammo: 50,
        distance: 100.,
        damage: 20.,
        mag_ammo: 5,
        fire_rate: 0.5,
        mag_size: 5,
        max_ammo: 50,
        max_heat: INFINITY,
        min_heat: 0.5,
        speed_spread: 10.,
        reload: basic_reload_fn,
        reload_time: 5.,
        shoot: charging_shoot_fn,
        speed: 200.,
        spread: (30_f32).to_radians(),
        ..default()
    }
}

pub fn create_charged_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = charged_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.revolver.clone(),
        "Charged".to_string(),
        pos,
        stats,
    )
}

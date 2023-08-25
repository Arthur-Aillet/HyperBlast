use std::{time::Duration, f32::INFINITY};

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{overheat_shoot_fn, no_reload, GunStats},
    },
    rendering::outline::Outline,
};

pub fn flamethrower_stats() -> GunStats {
    GunStats {
        shoot: overheat_shoot_fn,
        reload: no_reload,
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
        max_heat: 20.,
        ..Default::default()
    }
}

pub fn create_flamethrower_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = flamethrower_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.flame_thrower.clone(),
        "Flamethrower".to_string(),
        pos,
        stats,
    )
}

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

pub fn laser_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(1., 1.),
        size: Vec2::new(16., 5.),
        barrel_length: 18.,
        barrel_height: 3.,
        shoot: overheat_shoot_fn,
        reload: no_reload,
        damage: 1.,
        spread: (0_f32).to_radians(),
        speed: 1000.,
        speed_spread: 0.,
        distance: 1000.,
        salve: 1,
        ammo: 0,
        max_ammo: 0,
        infinite: true,
        mag_ammo: 1000,
        mag_size: 1000,
        reload_time: 5.,
        fire_rate: 30.,
        max_heat: 20.,
        ..Default::default()
    }
}

pub fn create_laser_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = laser_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.flame_thrower.clone(),
        "Laser".to_string(),
        pos,
        stats,
    )
}

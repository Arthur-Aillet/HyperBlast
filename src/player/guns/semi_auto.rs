use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{manual_shoot_fn, basic_reload_fn, GunStats},
    },
    rendering::outline::Outline,
};

pub fn semi_automatic_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(10., 4.),
        size: Vec2::new(30., 8.),
        barrel_length: 20.,
        barrel_height: 1.,
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

pub fn create_semi_auto_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = semi_automatic_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.kalachnikov.clone(),
        "Kalachnikov".to_string(),
        pos,
        stats,
    )
}

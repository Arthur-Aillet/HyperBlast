use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{auto_shoot_fn, basic_reload_fn, GunStats},
    },
    rendering::outline::Outline,
};

pub fn automatic_stats() -> GunStats {
    GunStats {
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

pub fn create_auto_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = automatic_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.automatic.clone(),
        "Auto".to_string(),
        pos,
        stats,
    )
}

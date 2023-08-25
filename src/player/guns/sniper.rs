use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{basic_reload_fn, manual_shoot_fn, GunStats},
    },
    rendering::outline::Outline,
};

pub fn sniper_stats() -> GunStats {
    GunStats {
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

pub fn create_sniper_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = sniper_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.sniper.clone(),
        "Sniper".to_string(),
        pos,
        stats,
    )
}

use std::time::Duration;

use bevy::prelude::*;

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

pub fn revolver_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(10., 3.),
        size: Vec2::new(14., 10.),
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
        ..default()
    }
}

pub fn create_revolver_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = revolver_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.revolver.clone(),
        "Revolver".to_string(),
        pos,
        stats,
    )
}

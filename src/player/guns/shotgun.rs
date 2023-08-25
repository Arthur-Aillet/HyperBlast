use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    player::{
        inventory::{
            pickup::GunPickupBundle,
            weapon_manager::GunAssets,
        },
        weapon::{manual_shoot_fn, shotgun_reload_fn, GunStats},
    },
    rendering::outline::Outline,
};


pub fn shotgun_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(10., 3.),
        size: Vec2::new(27., 7.),
        barrel_length: 19.,
        barrel_height: 2.5,
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


pub fn create_shotgun_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> GunPickupBundle {
    let mut stats = shotgun_stats();
    stats.timer.set_elapsed(Duration::new(1, 0));
    GunPickupBundle::create(
        meshes,
        materials,
        sprites.shotgun.clone(),
        "Shotgun".to_string(),
        pos,
        stats,
    )
}

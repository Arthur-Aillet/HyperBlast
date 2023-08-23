use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{player::{weapon::{GunStats, basic_reload_fn, manual_shoot_fn}, inventory::{weapon_manager::{GunAssets, Guns}, pickup::{PickupBundle, PickupType}}}, rendering::{utils::{set_anchor, Angle, Zindex}, outline::Outline}};

use super::GunBundle;

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


impl GunBundle {
    pub fn sniper(guns: &Res<GunAssets>) -> Self {
        let mut stats = sniper_stats();
        stats.timer.set_elapsed(Duration::new(1, 0));
        GunBundle {
            name: Name::new("Sniper"),
            sprite: SpriteBundle {
                texture: guns.sniper.clone(),
                transform: Transform::from_translation(Vec3::new(8., 0., 50.)),
                sprite: Sprite {
                    anchor: set_anchor(stats.handle_position, stats.size),
                    ..default()
                },
                ..default()
            },
            stats,
            angle: Angle(0.),
            zindex: Zindex(50.),
        }
    }
}

pub fn create_revolver_pickup(
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Outline>>,
    sprites: &Res<GunAssets>,
) -> PickupBundle {
    PickupBundle::create(
        meshes,
        materials,
        sprites.revolver.clone(),
        Vec2::new(16., 16.),
        "Revolver".to_string(),
        pos,
        PickupType::Gun(Guns::Revolver),
    )
}

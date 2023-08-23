use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{player::{weapon::{GunStats, auto_shoot_fn, basic_reload_fn}, inventory::{weapon_manager::{GunAssets, Guns}, pickup::{PickupBundle, PickupType}}}, rendering::{utils::{set_anchor, Angle, Zindex}, outline::Outline}};

use super::GunBundle;

pub fn revolver_stats() -> GunStats {
    GunStats {
        handle_position: Vec2::new(2., 2.),
        barrel_length: 12.,
        barrel_height: 5.5,
        timer: Stopwatch::new(),
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
        ..Default::default()
    }
}

impl GunBundle {
    pub fn revolver(guns: &Res<GunAssets>) -> Self {
        let mut stats = revolver_stats();
        stats.timer.set_elapsed(Duration::new(1, 0));
        GunBundle {
            name: Name::new("Revolver"),
            sprite: SpriteBundle {
                texture: guns.revolver.clone(),
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

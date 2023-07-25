use bevy::prelude::*;

use super::weapon::GunEntity;

#[derive(Component, Reflect)]
pub struct PlayerStats {
    pub speed: f32,
    pub health: f32,
    pub damages_multiplier: f32,
    pub damages_added: f32,
}

impl PlayerStats {
    pub fn default() -> Self {
        PlayerStats {
            speed: 50.,
            health: 100.,
            damages_multiplier: 1.,
            damages_added: 0.,
        }
    }
}

pub fn player_death(
    mut commands: Commands,
    players: Query<(
        Entity,
        &GunEntity,
        &PlayerStats
    )>,
) {
    for (entity, gun, stats) in &players {
        if stats.health <= 0. {
            commands.entity(entity).despawn_recursive();
            commands.entity(gun.0).despawn_recursive();
        }
    }
}

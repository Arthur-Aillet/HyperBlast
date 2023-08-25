use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct PlayerStats {
    pub speed: f32,
    pub current_health: f32,
    pub max_health: f32,
    pub damages_multiplier: f32,
    pub damages_added: f32,
    pub roll_duration: Duration,
    pub roll_speed: f32,
}

impl PlayerStats {
    pub fn default() -> Self {
        PlayerStats {
            speed: 50.,
            current_health: 100.,
            max_health: 100.,
            damages_multiplier: 1.,
            damages_added: 0.,
            roll_duration: Duration::from_secs_f32(0.5),
            roll_speed: 75.,
        }
    }
}

pub fn player_death(mut commands: Commands, players: Query<(Entity, &PlayerStats)>) {
    for (entity, stats) in &players {
        if stats.current_health <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

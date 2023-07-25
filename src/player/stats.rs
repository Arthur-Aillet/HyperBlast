use bevy::prelude::*;

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

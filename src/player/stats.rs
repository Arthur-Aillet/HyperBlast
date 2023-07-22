use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct PlayerStats {
    pub speed: f32,
}

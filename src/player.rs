use bevy::prelude::*;

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Moving
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[derive(Default, Component)]
pub struct Player {
    pub state: PlayerState,
}


impl Player {
    pub fn new() -> Self {
        Player::default()
    }
}

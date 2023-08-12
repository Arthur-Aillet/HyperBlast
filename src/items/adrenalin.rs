use bevy::prelude::*;
use super::Item;

#[derive(Component, Reflect)]
pub struct Adrenalin {
    pub strength: i8,
}

impl Adrenalin {
    pub fn default() -> Self {
        Adrenalin {
            strength: 2,
        }
    }
}

impl Item for Adrenalin {
    fn active_effect(&mut self, player: crate::player::stats::PlayerStats) {
    }
    fn passive_effect(&mut self, player: crate::player::stats::PlayerStats) {
    }
    fn pickup_effect(&mut self, player: crate::player::stats::PlayerStats) {
    }
}
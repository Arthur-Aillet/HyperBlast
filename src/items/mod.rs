use bevy::prelude::*;
use super::player::stats::PlayerStats;
pub mod adrenalin;

pub trait Item {
    fn passive_effect(&mut self, player: PlayerStats);
    fn pickup_effect(&mut self, player: PlayerStats);
    fn active_effect(&mut self, player: PlayerStats);
}
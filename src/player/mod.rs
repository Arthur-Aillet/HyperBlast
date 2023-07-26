pub mod bullets;
pub mod input;
pub mod setup;
pub mod stats;
pub mod weapon;

use bevy::prelude::*;

use input::PlayerState;
use stats::PlayerStats;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>()
            .register_type::<PlayerState>()
            .add_plugins(InputManagerPlugin::<input::PlayerActions>::default())
            .add_systems(Update, input::move_players)
            .add_systems(Update, input::shooting_system)
            .add_systems(Update, bullets::move_bullets)
            .add_systems(Update, bullets::detect_collision_bullets)
            .add_systems(PostUpdate, stats::player_death);
    }
}

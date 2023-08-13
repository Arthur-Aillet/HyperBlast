pub mod assets;
pub mod bullets;
pub mod input;
pub mod setup;
pub mod stats;
pub mod weapon;
pub mod roll;
pub mod direction;
pub mod inventory;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_asset_loader::prelude::*;

use input::PlayerState;
use leafwing_input_manager::prelude::*;
use stats::PlayerStats;

use self::assets::{GunAssets, PlayerAssets};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>()
            .register_type::<PlayerState>()
            .init_collection::<assets::PlayerAssets>()
            .init_collection::<assets::GunAssets>()
            .add_plugins(InputManagerPlugin::<input::PlayerActions>::default())
            .add_systems(Startup, setup_players)
            .add_systems(First, direction::calculate_players_cursors)
            .add_systems(First, direction::calculate_players_move_direction)
            .add_systems(PreUpdate, roll::start_roll)
            .add_systems(Update, input::move_players)
            .add_systems(Update, roll::rolling)
            .add_systems(Update, input::shooting_system)
            .add_systems(Update, bullets::move_bullets)
            .add_systems(Update, bullets::detect_collision_bullets)
            .add_systems(PostUpdate, stats::player_death);
    }
}

fn setup_players(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
    assets: Res<PlayerAssets>,
    guns: Res<GunAssets>,
) {
    setup::PlayerBundle::setup(&mut commands, &window, true, &assets, &guns);
    setup::PlayerBundle::setup(&mut commands, &window, false, &assets, &guns);
}

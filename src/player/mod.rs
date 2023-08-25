pub mod assets;
pub mod bullets;
pub mod direction;
pub mod guns;
pub mod input;
pub mod inventory;
pub mod reload;
pub mod roll;
pub mod setup;
pub mod stats;
pub mod weapon;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_asset_loader::prelude::*;

use input::PlayerState;
use leafwing_input_manager::prelude::*;
use stats::PlayerStats;

use crate::mouse::update_cursor_state_from_window;

use self::assets::PlayerAssets;
use self::inventory::weapon_manager::GunAssets;
use self::weapon::GunEntity;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>()
            .register_type::<PlayerState>()
            .register_type::<GunEntity>()
            .init_collection::<PlayerAssets>()
            .init_collection::<GunAssets>()
            .add_plugins(InputManagerPlugin::<input::PlayerActions>::default())
            .add_plugins(inventory::ItemsPlugin)
            .add_systems(Startup, setup_players)
            .add_systems(First, direction::calculate_players_cursors)
            .add_systems(First, direction::calculate_players_move_direction)
            .add_systems(Update, reload::start_reload)
            .add_systems(PreUpdate, roll::start_roll)
            .add_systems(
                Update,
                input::move_players.after(update_cursor_state_from_window),
            )
            .add_systems(Update, roll::rolling.after(update_cursor_state_from_window))
            .add_systems(
                Update,
                input::shooting_system.after(update_cursor_state_from_window),
            )
            .add_systems(Update, reload::reload.after(input::shooting_system))
            .add_systems(
                Update,
                bullets::move_bullets.after(update_cursor_state_from_window),
            )
            .add_systems(
                Update,
                bullets::detect_collision_bullets.after(update_cursor_state_from_window),
            )
            .add_systems(PostUpdate, stats::player_death);
    }
}

fn setup_players(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
    assets: Res<PlayerAssets>,
) {
    setup::PlayerBundle::setup(&mut commands, &window, true, &assets);
    setup::PlayerBundle::setup(&mut commands, &window, false, &assets);
}

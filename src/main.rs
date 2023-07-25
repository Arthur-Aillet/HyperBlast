mod animation;
mod debug;
mod mouse;
mod physics;
mod player;
mod rendering;
mod camera;

use bevy::window::PrimaryWindow;
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

use bevy::{input::InputSystem, prelude::*};
use player::input::PlayerState;
use player::stats::PlayerStats;

fn main() {
    App::new()
        .register_type::<PlayerStats>()
        .register_type::<PlayerState>()
        .add_plugins(InputManagerPlugin::<player::input::PlayerActions>::default())
        .add_plugins(InputManagerPlugin::<debug::DebugAction>::default())
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(rendering::RenderingPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            mouse::update_cursor_state_from_window
                .in_set(InputManagerSystem::ManualControl)
                .before(InputManagerSystem::ReleaseOnDisable)
                .after(InputManagerSystem::Tick)
                .after(InputManagerSystem::Update)
                .after(InputSystem),
        )
        .add_systems(Update, player::input::move_players)
        .add_systems(Update, player::input::shooting_system)
        .add_systems(Update, player::bullets::move_bullets)
        .add_systems(Update, player::bullets::detect_collision_bullets)
        .add_systems(PostUpdate, player::stats::player_death)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    commands.spawn((
        bevy::core::Name::new("Ground"),
        SpriteBundle {
            texture: asset_server.load("basic_ground.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
    ));
    player::setup::PlayerBundle::setup(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &window,
        true,
    );
    player::setup::PlayerBundle::setup(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &window,
        false,
    );
}

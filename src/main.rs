mod animations;
mod debug;
mod mouse;
mod player;
mod rendering;
mod physics;

use animations::AnimationIndices;
use animations::AnimationState;
use animations::AnimationStateMachine;
use animations::AnimationTimer;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::window::PrimaryWindow;
use bevy_editor_pls::prelude::*;
use bevy_prototype_debug_lines::*;
use debug::debug_setup;
use debug::DebugLevel;
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

use bevy::{input::InputSystem, prelude::*};
use physics::PhysicsPlugin;
use player::input::PlayerState;
use player::stats::PlayerStats;
use rendering::Angle;
use rendering::Position;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .register_type::<AnimationIndices>()
        .register_type::<PlayerStats>()
        .register_type::<PlayerState>()
        .register_type::<AnimationState>()
        .register_type::<AnimationStateMachine>()
        .register_type::<Position>()
        .register_type::<Angle>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin::default())
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin))
        .add_plugins(InputManagerPlugin::<player::input::PlayerActions>::default())
        .add_plugins(InputManagerPlugin::<debug::DebugAction>::default())
        .add_plugins(PhysicsPlugin)
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
        .add_systems(Update, debug::switch_debug)
        .add_systems(Update, player::input::shooting_system)
        .add_systems(Update, player::bullets::move_bullets)
        .add_systems(Update, player::bullets::detect_collision_bullets)
        .add_systems(PostUpdate, animations::animate_sprites)
        .add_systems(PostUpdate, player::stats::player_death)
        .add_systems(Last, rendering::update_transforms)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(DebugLevel::None);

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2;
    commands.spawn(camera);

    commands.spawn(debug_setup());

    commands.spawn((
        bevy::core::Name::new("Global Timer"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
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
    player::setup::PlayerBundle::setup(&mut commands, &asset_server, &mut texture_atlases, &window, true);
    player::setup::PlayerBundle::setup(&mut commands, &asset_server, &mut texture_atlases, &window, false);
}

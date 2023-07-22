mod player;
mod mouse;
mod animations;
mod rendering;

use animations::AnimationIndices;
use animations::AnimationState;
use animations::AnimationStateMachine;
use animations::AnimationTimer;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use bevy::window::PrimaryWindow;
use bevy_editor_pls::prelude::*;
use leafwing_input_manager::{systems::run_if_enabled, plugin::InputManagerSystem, prelude::*};
use bevy::{input::InputSystem, prelude::*};
use player::{PlayerState, PlayerStats};
use rendering::Position;

use crate::mouse::Mouse;

fn main() {
    App::new()
        .register_type::<AnimationIndices>()
        .register_type::<PlayerStats>()
        .register_type::<PlayerState>()
        .register_type::<AnimationState>()
        .register_type::<AnimationStateMachine>()
        .register_type::<Position>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin::default())
        .add_plugins((FrameTimeDiagnosticsPlugin::default(), EntityCountDiagnosticsPlugin::default()))
        .add_plugins(InputManagerPlugin::<player::PlayerActions>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            mouse::update_cursor_state_from_window
                    /*.run_if(run_if_enabled::<mouse::Mouse>)*/
                .in_set(InputManagerSystem::ManualControl)
                .before(InputManagerSystem::ReleaseOnDisable)
                .after(InputManagerSystem::Tick)
                .after(InputManagerSystem::Update)
                .after(InputSystem),
        )
        .add_systems(Update, player::move_players)
        .add_systems(Update, player::access_mouse)
        .add_systems(PostUpdate, animations::animate_sprites)
        .add_systems(Last, rendering::update_transforms)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("FIRST!");
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.2;
    commands.spawn(camera);
    commands.spawn((
        bevy::core::Name::new("Global Timer"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
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
        }
    ));
    let player_id = commands.spawn(
        player::PlayerBundle::setup(&asset_server, &mut texture_atlases),
    ).id();

    commands.entity(window.single()).insert(ActionStateDriver {
        action: crate::mouse::Mouse::MousePosition,
        targets: player_id.into(),
    });
}

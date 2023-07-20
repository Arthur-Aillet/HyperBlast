mod player;
mod animations;
mod rendering;

use animations::AnimationIndices;
use animations::AnimationState;
use animations::AnimationStateMachine;
use animations::AnimationTimer;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::prelude::*;
use player::PlayerState;
use player::PlayerStats;
use rendering::Position;

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
        .add_systems(Update, player::move_players)
        .add_systems(PostUpdate, animations::animate_sprites)
        .add_systems(Last, rendering::update_transforms)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
    commands.spawn((
        bevy::core::Name::new("Player"),
        player::PlayerBundle::setup(&asset_server, &mut texture_atlases),
    ));
}

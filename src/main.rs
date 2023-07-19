mod player;
mod animations;

use animations::AnimationIndices;
use animations::AnimationTimer;

use bevy_editor_pls::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .register_type::<AnimationIndices>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin::default())
        .add_plugins(InputManagerPlugin::<player::PlayerActions>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, player::move_players)
        .add_systems(Update, animations::animate_sprites)
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
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    ));
    commands.spawn((
        bevy::core::Name::new("Ground"),
        SpriteBundle {
            texture: asset_server.load("basic_ground.png"),
            ..default()
        }
    ));
    commands.spawn((
        bevy::core::Name::new("Player"),
        player::PlayerBundle::setup(&asset_server, &mut texture_atlases),
    ));
}

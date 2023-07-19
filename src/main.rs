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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(EditorPlugin::default())
        .add_plugins(InputManagerPlugin::<player::PlayerActions>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, animations::animate_sprites)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        bevy::core::Name::new("Global Timer"),
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    ));
    commands.spawn((
        bevy::core::Name::new("Ground"),
        SpriteBundle {
            texture: asset_server.load("basic_ground.png"),
            transform: Transform::from_scale(Vec3::splat(5.0)),
            ..default()
        }
    ));
    commands.spawn((
        bevy::core::Name::new("Player"),
        player::PlayerBundle::setup(&asset_server, &mut texture_atlases),
    ));
}

mod player;
mod animations;

use player::Player;
use crate::animations::AnimationIndices;
use crate::animations::AnimationTimer;
use bevy_editor_pls::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .register_type::<AnimationIndices>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, crate::animations::animate_sprites)
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
        bevy::core::Name::new("Player"),
        Player::setup(&asset_server, &mut texture_atlases),
    ));
}

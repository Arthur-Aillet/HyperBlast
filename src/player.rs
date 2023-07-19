use bevy::prelude::*;

use crate::animations::AnimationIndices;

#[derive(Component, Debug)]
pub enum PlayerState {
    Idle,
    _Moving
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[derive(Bundle)]
pub struct Player {
    pub state: PlayerState,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
}

impl Player {
    pub fn setup(asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Player {
        let texture_handle = asset_server.load("idle.png");
        let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(17.0, 20.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        Player {
            state: PlayerState::Idle,
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_scale(Vec3::splat(5.0)),
                ..default()
            },
            animation_indices,
        }
    }
}

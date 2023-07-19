use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use crate::animations::AnimationIndices;

#[derive(Component, Debug)]
pub enum PlayerState {
    Idle,
    _Moving
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub state: PlayerState,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub player: Player,
    pub player_action: InputManagerBundle::<PlayerActions>,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerActions {
    Left,
    Right,
    Up,
    Down
}

impl PlayerBundle {
    pub fn setup(asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> PlayerBundle {
        let texture_handle = asset_server.load("idle.png");
        let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(17.0, 20.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        PlayerBundle {
            state: PlayerState::Idle,
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_scale(Vec3::splat(5.0)),
                ..default()
            },
            animation_indices,
            player: Player,
            player_action: InputManagerBundle::<PlayerActions> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Q, PlayerActions::Left),
                    (KeyCode::D, PlayerActions::Right),
                    (KeyCode::Z, PlayerActions::Up),
                    (KeyCode::S, PlayerActions::Down)
                ]),
            }
        }
    }
}

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
pub struct PlayerStats {
    pub speed: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub state: PlayerState,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub player: PlayerStats,
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
            player: PlayerStats { speed: 50. },
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

pub fn move_players(
    time: Res<Time>,
    mut query: Query<(
        &PlayerStats,
        &ActionState<PlayerActions>,
        &mut Transform,
    )>)
{
    for (stats, actions, mut transform) in &mut query {
        if actions.pressed(PlayerActions::Left) {
            transform.translation.x -= stats.speed * time.delta_seconds();
        }
        if actions.pressed(PlayerActions::Right) {
            transform.translation.x += stats.speed * time.delta_seconds();
        }
        if actions.pressed(PlayerActions::Up) {
            transform.translation.y += stats.speed * time.delta_seconds();
        }
        if actions.pressed(PlayerActions::Down) {
            transform.translation.y -= stats.speed * time.delta_seconds();
        }
    }
}

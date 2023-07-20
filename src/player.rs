use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use crate::{animations::{AnimationIndices, AnimationStateMachine, AnimationState, AnimationFlip}, rendering::Position};

#[derive(Component, Debug, Reflect)]
pub enum PlayerState {
    Idle,
    Left,
    Right,
    Up,
    Down,
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[derive(Component, Default, Reflect)]
pub struct PlayerStats {
    pub speed: f32,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub player: PlayerStats,
    pub player_action: InputManagerBundle::<PlayerActions>,
    pub player_position: Position,
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
        let idle_texture_handle = asset_server.load("idle.png");
        let run_texture_handle = asset_server.load("run.png");
        let idle_atlas = TextureAtlas::from_grid(idle_texture_handle, Vec2::new(17.0, 25.0), 4, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 15.}));
        let down_atlas = TextureAtlas::from_grid(run_texture_handle.clone(), Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 15.}));
        let up_atlas = TextureAtlas::from_grid(run_texture_handle.clone(), Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 69.}));
        let side_atlas = TextureAtlas::from_grid(run_texture_handle, Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 42.}));

        let idle_handle = texture_atlases.add(idle_atlas);
        let side_handle = texture_atlases.add(side_atlas);
        let up_handle = texture_atlases.add(up_atlas);
        let down_handle = texture_atlases.add(down_atlas);

        let mut state_machine = AnimationStateMachine::new();

        state_machine.insert(PlayerState::Idle, (idle_handle.clone(), AnimationIndices { first: 0, last: 3 }, AnimationFlip::False));
        state_machine.insert(PlayerState::Left, (side_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::XAxis));
        state_machine.insert(PlayerState::Right, (side_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        state_machine.insert(PlayerState::Up, (up_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        state_machine.insert(PlayerState::Down, (down_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        PlayerBundle {
            state: AnimationState::new(&PlayerState::Idle),
            sprite: SpriteSheetBundle {
                texture_atlas: idle_handle,
                sprite: TextureAtlasSprite {index: 0, anchor: bevy::sprite::Anchor::TopLeft, ..default()},
                ..default()
            },
            state_machine,
            player: PlayerStats { speed: 50. },
            player_action: InputManagerBundle::<PlayerActions> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Q, PlayerActions::Left),
                    (KeyCode::D, PlayerActions::Right),
                    (KeyCode::Z, PlayerActions::Up),
                    (KeyCode::S, PlayerActions::Down)
                ]),
            },
            ..default()
        }
    }
}

pub fn move_players(
    time: Res<Time>,
    mut query: Query<(
        &PlayerStats,
        &ActionState<PlayerActions>,
        &mut Position,
        &mut AnimationState,
    )>)
{
    for (stats, actions, mut position, mut state) in &mut query {
        if actions.pressed(PlayerActions::Left) {
            position.x -= stats.speed * time.delta_seconds();
            *state = AnimationState::new(&PlayerState::Left);
        }
        if actions.pressed(PlayerActions::Right) {
            position.x += stats.speed * time.delta_seconds();
            *state = AnimationState::new(&PlayerState::Right);
        }
        if actions.pressed(PlayerActions::Up) {
            position.y += stats.speed * time.delta_seconds();
            *state = AnimationState::new(&PlayerState::Up);
        }
        if actions.pressed(PlayerActions::Down) {
            position.y -= stats.speed * time.delta_seconds();
            *state = AnimationState::new(&PlayerState::Down);
        }
        if actions.get_pressed().is_empty() {
            *state = AnimationState::new(&PlayerState::Idle);
        }
    }
}

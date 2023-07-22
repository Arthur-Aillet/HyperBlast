use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;
use mouse::Mouse;

use crate::{animations::{AnimationIndices, AnimationStateMachine, AnimationState, AnimationFlip}, rendering::Position, mouse};

#[derive(Component, Debug, Reflect)]
pub enum PlayerState {
    Idle,
    LeftFront,
    LeftBack,
    RightFront,
    RightBack,
    Front,
    Back,
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
    pub name: Name,
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub player: PlayerStats,
    pub player_action: InputManagerBundle::<PlayerActions>,
    pub mouse_action: InputManagerBundle::<Mouse>,
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
        let back_atlas = TextureAtlas::from_grid(run_texture_handle.clone(), Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 69.}));
        let front_atlas = TextureAtlas::from_grid(run_texture_handle.clone(), Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 15.}));
        let side_front_atlas = TextureAtlas::from_grid(run_texture_handle.clone(), Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 42.}));
        let side_back_atlas = TextureAtlas::from_grid(run_texture_handle, Vec2::new(17.0, 25.0), 6, 1, Some(Vec2 {x: 2., y: 2.}), Some(Vec2{x: 15., y: 69. + 27.}));

        let idle_handle = texture_atlases.add(idle_atlas);
        let side_front_handle = texture_atlases.add(side_front_atlas);
        let side_back_handle = texture_atlases.add(side_back_atlas);
        let front_handle = texture_atlases.add(front_atlas);
        let back_handle = texture_atlases.add(back_atlas);

        let mut state_machine = AnimationStateMachine::new();

        state_machine.insert(PlayerState::Idle, (idle_handle.clone(), AnimationIndices { first: 0, last: 3 }, AnimationFlip::False));
        state_machine.insert(PlayerState::LeftFront, (side_front_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::XAxis));
        state_machine.insert(PlayerState::RightFront, (side_front_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        state_machine.insert(PlayerState::LeftBack, (side_back_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::XAxis));
        state_machine.insert(PlayerState::RightBack, (side_back_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        state_machine.insert(PlayerState::Front, (front_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        state_machine.insert(PlayerState::Back, (back_handle.clone(), AnimationIndices { first: 0, last: 5 }, AnimationFlip::False));
        PlayerBundle {
            name: bevy::core::Name::new("Player"),
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
            mouse_action: InputManagerBundle::<Mouse>::default(),
            ..default()
        }
    }
}

pub fn access_mouse(query: Query<&ActionState<Mouse>>) {
    let action_state: &ActionState<Mouse> = query.single();

    if let Some(box_pan_vector) = action_state.axis_pair(Mouse::MousePosition) {
        println!("{:?}", box_pan_vector);
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
        let mut direction = Vec2::default();

        if actions.pressed(PlayerActions::Left) {
            direction.x -= 1.;
        }
        if actions.pressed(PlayerActions::Right) {
            direction.x += 1.;
        }
        if actions.pressed(PlayerActions::Up) {
            direction.y += 1.;
        }
        if actions.pressed(PlayerActions::Down) {
            direction.y -= 1.;
        }
        if direction == Vec2::ZERO {
            *state = AnimationState::new(&PlayerState::Idle);
        } else {
            // *state = AnimationState::new(&PlayerState::Down);
            let mut angle = direction.angle_between(Vec2::NEG_Y).to_degrees();
            if angle < 0. { angle = angle + 360. }
            *state = match angle {
                n if (n < 30. + 60. * 0.) => {AnimationState::new(&PlayerState::Front)},
                n if (n <= 30. + 60. * 1.) => {AnimationState::new(&PlayerState::LeftFront)},
                n if (n < 30. + 60. * 2.) => {AnimationState::new(&PlayerState::LeftBack)},
                n if (n < 30. + 60. * 3.) => {AnimationState::new(&PlayerState::Back)},
                n if (n < 30. + 60. * 4.) => {AnimationState::new(&PlayerState::RightBack)},
                n if (n < 30. + 60. * 5.) => {AnimationState::new(&PlayerState::RightFront)},
                n if (n < 30. + 60. * 6.) => {AnimationState::new(&PlayerState::Front)},
                _ => { panic!("IMPOSSIBLE ANGLE!") }
            };
            position.0 += direction.normalize_or_zero() * stats.speed * time.delta_seconds();
        }
    }
}

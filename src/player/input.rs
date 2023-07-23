use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::{
    animations::{AnimationFlip, AnimationState},
    debug::DebugLevel,
    mouse::Mouse,
    rendering::{Angle, Position},
};

use super::{
    stats::PlayerStats,
    weapon::{GunEntity, GunStats},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerActions {
    ControllerMove,
    ControllerLook,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Debug, Reflect, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    LeftFront,
    LeftBack,
    RightFront,
    RightBack,
    Front,
    Back,
}

#[allow(clippy::type_complexity)]
pub fn rotate_player(
    mouse: Query<&ActionState<Mouse>>,
    players: Query<(&Position, &GunEntity, &ActionState<PlayerActions>, &PlayerStats)>,
    mut gun: Query<(
        &mut Position,
        &mut Angle,
        &mut AnimationFlip,
        (With<GunStats>, Without<PlayerStats>),
    )>,
    camera: Query<(&Camera, &GlobalTransform)>,
    debug_level: Res<DebugLevel>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    let mouse_action_state: &ActionState<Mouse> = mouse.single();

    for (Position(player_pos), gun_id, player_actions, stats) in &players {
        if let Some((camera, camera_transform)) =
            camera.into_iter().find(|(camera, _)| camera.is_active)
        {
            let mut cursor_position: Option<Vec2> = None;

            if stats.controller {
                if player_actions.pressed(PlayerActions::ControllerLook) {
                    let axis_pair = player_actions.clamped_axis_pair(PlayerActions::ControllerLook).unwrap();
                    cursor_position = Some(*player_pos + axis_pair.xy().normalize() * 30.);
                }
            } else {
                let mouse_ray = mouse_action_state
                .axis_pair(Mouse::MousePosition)
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor.xy()));

                if let Some(mouse_pos) = mouse_ray {
                    cursor_position = Some(mouse_pos.origin.truncate());
                }
            }

            if let Ok((mut gun_pos, mut gun_angle, mut flip, _)) = gun.get_mut(gun_id.0) {
                gun_pos.0 = *player_pos;
                gun_pos.0.x += 6.;
                if let Some(cursor_position) = cursor_position {
                    let direction = (cursor_position - gun_pos.0).normalize();
                    let mut barrel_position = gun_pos.0 + direction.perp() * 5.5;
                    let mut barrel_to_cursor = cursor_position - barrel_position;
                    *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
                    *flip = if gun_angle.0.abs().to_degrees() > 90. {
                        barrel_position = gun_pos.0 + direction.perp() * -5.5;
                        barrel_to_cursor = cursor_position - barrel_position;
                        *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
                        AnimationFlip::YAxis
                    } else {
                        AnimationFlip::False
                    };
                    if *debug_level == DebugLevel::Basic {
                        lines.line_colored((gun_pos.0).extend(0.), barrel_position.extend(0.), 0.0, Color::RED);
                        lines.line_colored(barrel_position.extend(0.), cursor_position.extend(0.), 0.0, Color::GREEN);
                        lines.line_colored(cursor_position.extend(0.), (gun_pos.0).extend(0.), 0.0, Color::GOLD);
                    }
                }
            }
        }
    }
}

pub fn player_input_setup() -> InputManagerBundle::<PlayerActions> {
    let mut input_map = InputMap::new([
        (KeyCode::Q, PlayerActions::Left),
        (KeyCode::D, PlayerActions::Right),
        (KeyCode::Z, PlayerActions::Up),
        (KeyCode::S, PlayerActions::Down),
    ]);
    input_map.insert(DualAxis::left_stick(), PlayerActions::ControllerMove);
    input_map.insert(DualAxis::right_stick(), PlayerActions::ControllerLook);
    InputManagerBundle::<PlayerActions> {
        action_state: ActionState::default(),
        input_map
    }
}

pub fn move_players(
    time: Res<Time>,
    mut query: Query<(
        &PlayerStats,
        &ActionState<PlayerActions>,
        &mut Position,
        &mut AnimationState,
    )>,
) {
    for (stats, actions, mut position, mut state) in &mut query {
        let mut direction = Vec2::ZERO;

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

        if actions.pressed(PlayerActions::ControllerMove) {
            let axis_pair = actions.clamped_axis_pair(PlayerActions::ControllerMove).unwrap();
            direction.x += axis_pair.x();
            direction.y += axis_pair.y();
        }

        if direction == Vec2::ZERO {
            *state = AnimationState::new(&PlayerState::Idle);
        } else {
            let mut angle = direction.angle_between(Vec2::NEG_Y).to_degrees();
            if angle < 0. {
                angle += 360.
            }
            *state = match angle {
                n if (n < 30. + 60. * 0.) => AnimationState::new(&PlayerState::Front),
                n if (n <= 30. + 60. * 1.) => AnimationState::new(&PlayerState::LeftFront),
                n if (n < 30. + 60. * 2.) => AnimationState::new(&PlayerState::LeftBack),
                n if (n < 30. + 60. * 3.) => AnimationState::new(&PlayerState::Back),
                n if (n < 30. + 60. * 4.) => AnimationState::new(&PlayerState::RightBack),
                n if (n < 30. + 60. * 5.) => AnimationState::new(&PlayerState::RightFront),
                n if (n < 30. + 60. * 6.) => AnimationState::new(&PlayerState::Front),
                _ => {
                    panic!("IMPOSSIBLE ANGLE!")
                }
            };
            position.0 += direction.clamp_length(0., 1.) * stats.speed * time.delta_seconds();
        }
    }
}

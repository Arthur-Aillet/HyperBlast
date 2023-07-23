use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::{prelude::ActionState, Actionlike};

use crate::{animations::AnimationState, debug::DebugLevel, mouse::Mouse, rendering::{Position, Angle}};

use super::{stats::PlayerStats, weapon::{GunEntity, GunStats}};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerActions {
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

pub fn rotate_player(
    mouse: Query<&ActionState<Mouse>>,
    players: Query<(&Position, &GunEntity, With<PlayerStats>)>,
    mut gun: Query<(&mut Position, &mut Angle, (With<GunStats>, Without<PlayerStats>))>,
    camera: Query<(&Camera, &GlobalTransform)>,
    debug_level: Res<DebugLevel>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    let action_state: &ActionState<Mouse> = mouse.single();

    for (player_pos, gun_id, _) in &players {
        if let Some((camera, camera_transform)) =
            camera.into_iter().find(|(camera, _)| camera.is_active)
        {
            if let Some(mouse_pos) = action_state
                .axis_pair(Mouse::MousePosition)
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor.xy()))
            {
                if let Ok((mut gun_pos, mut gun_angle, _)) = gun.get_mut(gun_id.0) {
                    let arms_vector = (Vec2::new(mouse_pos.origin.x, mouse_pos.origin.y) - player_pos.0).normalize() * 1.;
                    let cannon_center = (player_pos.0 + arms_vector.perp() * 5.5).extend(0.);
                    let gun_orientation = mouse_pos.origin - cannon_center;
                    gun_pos.0 = player_pos.0 + arms_vector;
                    *gun_angle = Angle(gun_orientation.y.atan2(gun_orientation.x));
                    if *debug_level == DebugLevel::Basic {
                        lines.line_colored(
                            player_pos.0.extend(0.),
                            cannon_center,
                            0.0,
                            Color::RED,
                        );
                        lines.line_colored(
                            cannon_center,
                            mouse_pos.origin,
                            0.0,
                            Color::GREEN,
                        );
                    }
                }
                if *debug_level == DebugLevel::Basic {
                    lines.line_colored(
                        mouse_pos.origin,
                        player_pos.0.extend(0.),
                        0.0,
                        Color::GOLD,
                    );
                }
            }
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
                _ => { panic!("IMPOSSIBLE ANGLE!") }
            };
            position.0 += direction.normalize_or_zero() * stats.speed * time.delta_seconds();
        }
    }
}

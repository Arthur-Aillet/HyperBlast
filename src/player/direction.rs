use bevy::{prelude::*, math::Vec3Swizzles};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    debug::DebugLevel,
    mouse::Mouse,
    player::input::{IsController, PlayerActions},
};

#[derive(Component, Default, Clone)]
pub struct CursorPosition {
    pub value: Vec2,
    pub relative: Vec2,
}

#[derive(Component, Default, Clone)]
pub struct MoveDirection {
    pub value: Vec2,
}

impl MoveDirection {
    pub fn to_angle(&self) -> f32 {
        let mut angle = self.value.angle_between(Vec2::NEG_Y).to_degrees();
        if angle < 0. {
            angle += 360.
        }
        angle
    }
}

pub fn calculate_cursor_position(
    is_controller: bool,
    player_actions: &ActionState<PlayerActions>,
    camera_transform: &GlobalTransform,
    camera: &Camera,
    player_pos: Vec2,
    mouse: Option<&ActionState<Mouse>>,
) -> Option<Vec2> {
    if is_controller {
        if player_actions.pressed(PlayerActions::ControllerLook) {
            let axis_pair = player_actions
                .clamped_axis_pair(PlayerActions::ControllerLook)
                .unwrap();
            return Some(player_pos + axis_pair.xy().normalize() * 30.);
        }
    } else if let Some(mouse_action_state) = mouse {
        let mouse_ray = mouse_action_state
            .axis_pair(Mouse::MousePosition)
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor.xy()));

        if let Some(mouse_pos) = mouse_ray {
            return Some(mouse_pos.origin.truncate());
        }
    }
    None
}

pub fn calculate_players_cursors(
    mut players: Query<(
        Option<&IsController>,
        &Transform,
        &ActionState<PlayerActions>,
        &mut CursorPosition,
    )>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    mouse: Query<&ActionState<Mouse>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    debug_level: Res<crate::debug::DebugLevel>,
) {
    if let Some((camera, camera_transform)) =
        camera.into_iter().find(|(camera, _)| camera.is_active)
    {
        for (controller, transfom, player_actions, mut cursor) in &mut players {
            let mouse_maybe = mouse.get_single();
            let player_pos = transfom.translation.xy();

            if let Some(pos) = calculate_cursor_position(
                controller.is_some(),
                player_actions,
                camera_transform,
                camera,
                player_pos,
                mouse_maybe.ok(),
            ) {
                cursor.relative = pos - player_pos;
                cursor.value = pos;
                if *debug_level == DebugLevel::Basic {
                    lines.line_colored(Vec3::ZERO, (cursor.relative).extend(0.), 0.0, Color::RED);
                    lines.line_colored(
                        (player_pos).extend(0.),
                        (pos).extend(0.),
                        0.0,
                        Color::LIME_GREEN,
                    );
                }
            } else {
                cursor.value = cursor.relative + player_pos;
                if *debug_level == DebugLevel::Basic {
                    lines.line_colored(
                        (player_pos).extend(0.),
                        (cursor.value).extend(0.),
                        0.0,
                        Color::PURPLE,
                    );
                }
            }
        }
    }
}

pub fn calculate_players_move_direction(
    mut query: Query<(
        Option<&IsController>,
        &ActionState<PlayerActions>,
        &mut MoveDirection,
    )>,
) {
    for (controller, actions, mut direction) in &mut query {
        direction.value = Vec2::ZERO;

        if controller.is_some() {
            if actions.pressed(PlayerActions::ControllerMove) {
                let axis_pair = actions
                    .clamped_axis_pair(PlayerActions::ControllerMove)
                    .unwrap();
                direction.value.x += axis_pair.x();
                direction.value.y += axis_pair.y();
            }
        } else {
            if actions.pressed(PlayerActions::Left) {
                direction.value.x -= 1.;
            }
            if actions.pressed(PlayerActions::Right) {
                direction.value.x += 1.;
            }
            if actions.pressed(PlayerActions::Up) {
                direction.value.y += 1.;
            }
            if actions.pressed(PlayerActions::Down) {
                direction.value.y -= 1.;
            }
        }
    }
}

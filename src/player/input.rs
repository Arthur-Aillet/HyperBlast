use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::{
    animation::AnimationState,
    debug::DebugLevel,
    rendering::{Angle, Flip, Position},
};

use crate::player::{
    stats::PlayerStats,
    weapon::{GunEntity, GunStats},
};

use crate::player::roll::RollStats;

use super::{direction::{MoveDirection, CursorPosition}, reload::ReloadStats};

#[derive(Component)]
pub struct IsController;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerActions {
    ControllerMove,
    ControllerLook,
    Left,
    Right,
    Up,
    Down,
    Shoot,
    Roll,
    Reload,
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
    DodgeLeftFront,
    DodgeLeftBack,
    DodgeRightFront,
    DodgeRightBack,
    DodgeFront,
    DodgeBack,
}

pub fn update_gun_angle(
    debug_level: DebugLevel,
    lines: &mut bevy_prototype_debug_lines::DebugLines,
    gun_pos: Vec2,
    cursor_position: Vec2,
    gun_stats: &GunStats,
    gun_angle: &mut Angle,
    flip: &mut Flip,
) {
    let direction = (cursor_position - gun_pos).normalize();
    let mut barrel_position = gun_pos + direction.perp() * gun_stats.barrel_height;
    let mut barrel_to_cursor = cursor_position - barrel_position;
    *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
    *flip = if gun_angle.0.abs().to_degrees() > 90. {
        barrel_position = gun_pos + direction.perp() * -gun_stats.barrel_height;
        barrel_to_cursor = cursor_position - barrel_position;
        *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
        Flip::YAxis
    } else {
        Flip::False
    };
    if debug_level == DebugLevel::Basic {
        lines.line_colored(
            (gun_pos).extend(0.),
            barrel_position.extend(0.),
            0.0,
            Color::RED,
        );
        lines.line_colored(
            barrel_position.extend(0.),
            cursor_position.extend(0.),
            0.0,
            Color::GREEN,
        );
        lines.line_colored(
            cursor_position.extend(0.),
            (gun_pos).extend(0.),
            0.0,
            Color::GOLD,
        );
    }
}

pub fn shooting_system(
    time: Res<Time>,
    mut players: Query<(
        Entity,
        &Position,
        &GunEntity,
        &ActionState<PlayerActions>,
        &mut PlayerStats,
        &CursorPosition,
        Option<&RollStats>,
        Option<&ReloadStats>,
    )>,
    mut guns: Query<(
        &mut Position,
        &mut Angle,
        &mut Flip,
        &mut GunStats,
        Without<PlayerStats>,
    )>,
    debug_level: Res<DebugLevel>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    mut commands: Commands,
    gun_assets: Res<super::assets::GunAssets>,
) {

    for (entity, Position(player_pos), gun_id, player_actions, mut stats, cursor_position, roll, reload) in
        &mut players
    {
        if let Ok((mut gun_pos, mut gun_angle, mut flip, mut gun_stats, _)) =
            guns.get_mut(gun_id.0)
        {
            gun_pos.0 = *player_pos;
            gun_pos.0.x += 6.;

            update_gun_angle(
                (*debug_level).clone(),
                &mut lines,
                gun_pos.0,
                cursor_position.value,
                &gun_stats,
                &mut gun_angle,
                &mut flip,
            );
            let angle = gun_angle.0;
            let direction = Vec2::from_angle(angle).normalize();
            let barrel_position = if *flip == Flip::False {
                gun_pos.0 + direction.perp() * gun_stats.barrel_height
            } else {
                gun_pos.0 + direction.perp() * -gun_stats.barrel_height
            };
            let barrel_end =
                barrel_position + Vec2::from_angle(angle) * gun_stats.barrel_length;
            if *debug_level == DebugLevel::Basic {
                lines.line_colored(
                    (barrel_end).extend(0.),
                    (barrel_end + Vec2::from_angle(angle + gun_stats.spread) * 30.).extend(0.),
                    0.0,
                    Color::LIME_GREEN,
                );
                lines.line_colored(
                    (barrel_end).extend(0.),
                    (barrel_end + Vec2::from_angle(angle - gun_stats.spread) * 30.).extend(0.),
                    0.0,
                    Color::LIME_GREEN,
                );
            }

            gun_stats.timer.tick(time.delta());
            if player_actions.pressed(PlayerActions::Shoot) && roll.is_none() && reload.is_none() {
                (gun_stats.shoot)(
                    &mut commands,
                    &gun_assets,
                    &mut gun_stats,
                    &mut stats,
                    barrel_end,
                    angle,
                    entity,
                );
            }
        }
    }
}

pub fn player_input_setup(is_controller: bool) -> InputManagerBundle<PlayerActions> {
    let mut input_map: InputMap<PlayerActions>;
    if is_controller {
        input_map = InputMap::new([
            (GamepadButtonType::RightTrigger2, PlayerActions::Shoot),
            (GamepadButtonType::LeftTrigger2, PlayerActions::Roll),
            (GamepadButtonType::South, PlayerActions::Reload),
        ]);
        input_map
            .insert(DualAxis::left_stick(), PlayerActions::ControllerMove)
            .insert(DualAxis::right_stick(), PlayerActions::ControllerLook);
    } else {
        input_map = InputMap::new([
            (KeyCode::Q, PlayerActions::Left),
            (KeyCode::D, PlayerActions::Right),
            (KeyCode::Z, PlayerActions::Up),
            (KeyCode::S, PlayerActions::Down),
            (KeyCode::Space, PlayerActions::Roll),
            (KeyCode::R, PlayerActions::Reload),
        ]);
        input_map.insert(MouseButton::Left, PlayerActions::Shoot);
    }

    InputManagerBundle::<PlayerActions> {
        action_state: ActionState::default(),
        input_map,
    }
}

type PlayerEntity<'a> = (
    &'a MoveDirection,
    &'a PlayerStats,
    &'a mut Position,
    &'a mut AnimationState,
    Without<RollStats>
);

pub fn move_players(time: Res<Time>, mut query: Query<PlayerEntity>) {
    for (direction, stats, mut position, mut state, _) in &mut query {
        if direction.value == Vec2::ZERO {
            *state = AnimationState::new(&PlayerState::Idle);
        } else {
            *state = match direction.to_angle() {
                n if (n < 30. + 60. * 0.) => AnimationState::new(&PlayerState::Front),
                n if (n <= 30. + 60. * 1.) => AnimationState::new(&PlayerState::LeftFront),
                n if (n < 30. + 60. * 2.) => AnimationState::new(&PlayerState::LeftBack),
                n if (n < 30. + 60. * 3.) => AnimationState::new(&PlayerState::Back),
                n if (n < 30. + 60. * 4.) => AnimationState::new(&PlayerState::RightBack),
                n if (n <= 30. + 60. * 5.) => AnimationState::new(&PlayerState::RightFront),
                n if (n < 30. + 60. * 6.) => AnimationState::new(&PlayerState::Front),
                _ => {
                    panic!("IMPOSSIBLE ANGLE!")
                }
            };
            position.0 += direction.value.clamp_length(0., 1.) * stats.speed * time.delta_seconds();
        }
    }
}

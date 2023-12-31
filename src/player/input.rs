use bevy::{math::Vec3Swizzles, prelude::*, reflect::TypePath};
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::rendering::utils::set_anchor;
use crate::{animation::AnimationState, debug::DebugLevel, rendering::utils::Angle};

use crate::player::{
    reload::ReloadStats,
    roll::RollStats,
    stats::PlayerStats,
    weapon::{GunEntity, GunStats},
};

use super::{
    direction::{CursorPosition, MoveDirection},
    inventory::inventory_manager::Inventory,
};

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
    Pickup,
    DropItem,
    DropWeapon,
    NextWeapon,
    LastWeapon,
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
    sprite: &mut Sprite,
) {
    let direction = (cursor_position - gun_pos).normalize();
    let mut barrel_position = gun_pos + direction.perp() * gun_stats.barrel_height;
    let mut barrel_to_cursor = cursor_position - barrel_position;
    *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
    if gun_angle.0.abs().to_degrees() > 90. {
        barrel_position = gun_pos + direction.perp() * -gun_stats.barrel_height;
        barrel_to_cursor = cursor_position - barrel_position;
        *gun_angle = Angle(barrel_to_cursor.y.atan2(barrel_to_cursor.x));
        sprite.flip_y = true;
        sprite.anchor = set_anchor(
            Vec2 {
                x: gun_stats.handle_position.x,
                y: gun_stats.size.y - gun_stats.handle_position.y,
            },
            gun_stats.size,
        );
    } else {
        sprite.flip_y = false;
        sprite.anchor = set_anchor(gun_stats.handle_position, gun_stats.size);
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
        &GunEntity,
        &ActionState<PlayerActions>,
        &mut PlayerStats,
        &Inventory,
        &CursorPosition,
        Option<&RollStats>,
        Option<&ReloadStats>,
    )>,
    mut gun: Query<(
        &GlobalTransform,
        &mut Angle,
        &mut Sprite,
        &mut GunStats,
        Without<PlayerStats>,
    )>,
    debug_level: Res<DebugLevel>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    mut commands: Commands,
    gun_assets: Res<super::inventory::weapon_manager::GunAssets>,
) {
    for (entity, gun_id, player_actions, mut stats, inv, cursor_position, roll, reload) in
        &mut players
    {
        if let Ok((gun_transform, mut gun_angle, mut sprite, mut gun_stats, _)) =
            gun.get_mut(gun_id.0)
        {
            let gun_pos = gun_transform.translation().xy();

            update_gun_angle(
                (*debug_level).clone(),
                &mut lines,
                gun_pos,
                cursor_position.value,
                &gun_stats,
                &mut gun_angle,
                &mut sprite,
            );
            let angle = gun_angle.0;
            let direction = Vec2::from_angle(angle).normalize();
            let barrel_position = if !sprite.flip_y {
                gun_pos + direction.perp() * gun_stats.barrel_height
            } else {
                gun_pos + direction.perp() * -gun_stats.barrel_height
            };
            let barrel_end = barrel_position + Vec2::from_angle(angle) * gun_stats.barrel_length;
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
            (gun_stats.shoot)(
                &mut commands,
                &gun_assets,
                &mut gun_stats,
                &mut stats,
                inv,
                barrel_end,
                angle,
                entity,
                player_actions,
                roll,
                reload,
            );
        }
    }
}

pub fn player_input_setup(is_controller: bool) -> InputManagerBundle<PlayerActions> {
    let mut input_map: InputMap<PlayerActions>;
    if is_controller {
        input_map = InputMap::new([
            (GamepadButtonType::RightTrigger2, PlayerActions::Shoot),
            (GamepadButtonType::LeftTrigger2, PlayerActions::Roll),
            (GamepadButtonType::North, PlayerActions::Reload),
            (GamepadButtonType::South, PlayerActions::Pickup),
            (GamepadButtonType::Start, PlayerActions::DropItem),
            (GamepadButtonType::Select, PlayerActions::DropWeapon),
            (GamepadButtonType::DPadRight, PlayerActions::NextWeapon),
            (GamepadButtonType::DPadLeft, PlayerActions::LastWeapon),
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
            (KeyCode::E, PlayerActions::Pickup),
            (KeyCode::W, PlayerActions::DropItem),
            (KeyCode::X, PlayerActions::DropWeapon),
            (KeyCode::Key1, PlayerActions::LastWeapon),
            (KeyCode::Key2, PlayerActions::NextWeapon),
        ]);
        input_map
            .insert(MouseButton::Left, PlayerActions::Shoot)
            .insert(MouseWheelDirection::Up, PlayerActions::NextWeapon)
            .insert(MouseWheelDirection::Down, PlayerActions::LastWeapon);
    }

    InputManagerBundle::<PlayerActions> {
        action_state: ActionState::default(),
        input_map,
    }
}

type PlayerEntity<'a> = (
    &'a MoveDirection,
    &'a PlayerStats,
    &'a mut Velocity,
    &'a mut AnimationState,
    Without<RollStats>,
);

pub fn move_players(mut query: Query<PlayerEntity>) {
    for (direction, stats, mut velocity, mut state, _) in &mut query {
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
        }
        velocity.linvel = direction.value.clamp_length(0., 1.) * stats.speed;
    }
}

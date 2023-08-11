use bevy::prelude::*;
use bevy::time::Stopwatch;
use leafwing_input_manager::prelude::ActionState;

use crate::animation::{AnimationState, AnimationStateMachine};

use crate::player::input::{PlayerState, PlayerActions};
use crate::player::stats::PlayerStats;

use crate::player::input;

#[derive(Component)]
pub struct RollStats {
    since: Stopwatch,
    current_frame: u8,
    start_direction: input::Direction,
}

impl RollStats {
    pub fn new(start_direction: input::Direction) -> Self {
        RollStats {
            since: Stopwatch::new(),
            start_direction,
            current_frame: 1u8,
        }
    }
}

pub fn start_roll(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ActionState<PlayerActions>,
        &input::Direction,
        &mut AnimationState,
        &mut AnimationStateMachine,
        &PlayerStats,
        Without<RollStats>
    )>
) {
    for (entity, action_state, direction, mut state, mut machine, stats, _) in &mut query {
        if action_state.pressed(PlayerActions::Roll) { // Why Just Pressed work half of the time
            let roll_stats;
            *state = if direction.value == Vec2::ZERO {
                roll_stats = RollStats::new(input::Direction { value: Vec2::NEG_Y });
                AnimationState::new(&PlayerState::DodgeFront)
            } else {
                roll_stats = RollStats::new(direction.clone());
                match direction.to_angle() {
                    n if (n < 30. + 60. * 0.) => AnimationState::new(&PlayerState::DodgeFront),
                    n if (n <= 30. + 60. * 1.) => AnimationState::new(&PlayerState::DodgeLeftFront),
                    n if (n < 30. + 60. * 2.) => AnimationState::new(&PlayerState::DodgeLeftBack),
                    n if (n < 30. + 60. * 3.) => AnimationState::new(&PlayerState::DodgeBack),
                    n if (n < 30. + 60. * 4.) => AnimationState::new(&PlayerState::DodgeRightBack),
                    n if (n < 30. + 60. * 5.) => AnimationState::new(&PlayerState::DodgeRightFront),
                    n if (n < 30. + 60. * 6.) => AnimationState::new(&PlayerState::DodgeFront),
                    _ => {
                        panic!("IMPOSSIBLE ANGLE!")
                    }
                }
            };
            machine.set_manual(true);
            commands.entity(entity).insert(roll_stats);
        }
    }
}

pub fn rolling(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut crate::rendering::Position,
        &mut RollStats,
        &mut AnimationStateMachine,
        &PlayerStats,
    )>
) {
    for (player, mut pos, mut roll_stats, mut machine, stats) in &mut query {
        roll_stats.since.tick(time.delta());

        pos.0 += roll_stats.start_direction.value.clamp_length(0., 1.) * stats.roll_speed * time.delta_seconds();

        if roll_stats.since.elapsed_secs() >= (stats.roll_duration.as_secs_f32() / 9.) * roll_stats.current_frame as f32 {
            roll_stats.current_frame += 1;
            machine.next();
        }
        if roll_stats.since.elapsed_secs() >= stats.roll_duration.as_secs_f32() {
            machine.set_manual(false);
            commands.entity(player).remove::<RollStats>();
        }
    }
}

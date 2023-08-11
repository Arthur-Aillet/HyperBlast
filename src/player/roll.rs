use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::animation::AnimationState;

use crate::player::input::{PlayerState, PlayerActions};

pub fn roll(mut query: Query<(&ActionState<PlayerActions>, &mut AnimationState)>) {

}

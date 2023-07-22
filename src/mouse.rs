use bevy::prelude::*;
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Reflect)]
pub enum Mouse {
    MousePosition,
}

pub fn update_cursor_state_from_window(
    window_query: Query<(&Window, &ActionStateDriver<Mouse>)>,
    mut action_state_query: Query<&mut ActionState<Mouse>>,
) {
    for (window, driver) in window_query.iter() {
        for entity in driver.targets.iter() {
            let mut action_state = action_state_query
                .get_mut(*entity)
                .expect("Entity does not exist, or does not have an `ActionState` component");

            if let Some(val) = window.cursor_position() {
                action_state.action_data_mut(driver.action).axis_pair =
                    Some(DualAxisData::from_xy(val));
            }
        }
    }
}

use bevy::prelude::*;
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};

use crate::rendering::zoom::PostProcessSettings;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Reflect)]
pub enum Mouse {
    MousePosition,
}

pub fn update_cursor_state_from_window(
    window_query: Query<(&Window, &ActionStateDriver<Mouse>)>,
    settings: Query<&PostProcessSettings>,
    mut action_state_query: Query<&mut ActionState<Mouse>>,
) {
    for (window, driver) in &window_query {
        for entity in driver.targets.iter() {
            if let Ok(mut action_state) = action_state_query.get_mut(*entity) {
                if let Some(pos) = window.cursor_position() {
                    let center: Vec2 = Vec2::new(window.width() / 2., window.height() / 2.);
                    let center_mouse_vec = pos - center;
                    let center_mouse_scaled = center_mouse_vec * settings.single().intensity;

                    action_state.action_data_mut(driver.action).axis_pair = Some(DualAxisData::from_xy(center_mouse_scaled + center));
                }
            }
        }
    }
}

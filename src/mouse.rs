use bevy::{prelude::*, math::Vec3Swizzles};
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};

use crate::{rendering::zoom::PostProcessSettings, camera::CameraData};

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Reflect)]
pub enum Mouse {
    MousePosition,
}

pub fn update_cursor_state_from_window(
    window_query: Query<(&Window, &ActionStateDriver<Mouse>)>,
    settings: Query<&PostProcessSettings>,
    mut action_state_query: Query<&mut ActionState<Mouse>>,
    camera: Query<(
        &Transform,
        With<CameraData>,
    )>
) {
    for (window, driver) in &window_query {
        for entity in driver.targets.iter() {
            if let Ok(mut action_state) = action_state_query.get_mut(*entity) {
                if let Some(pos) = window.cursor_position() {
                    //let pos_from_center = Vec2::new(pos.x - window.width() / 2., pos.y - window.height() / 2.) / settings.single().intensity;
                    let center: Vec2 = Vec2::new(window.width() / 2., window.height() / 2.);
                    let center_mouse_vec = pos - center;
                    let center_mouse_scaled = center_mouse_vec * settings.single().intensity;
                    //println!("Intense = {} {} {}", settings.single().intensity, pos_from_center, pos);
                    action_state.action_data_mut(driver.action).axis_pair = Some(DualAxisData::from_xy(center_mouse_scaled + center));
                }
            }
        }
    }
}

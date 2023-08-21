pub mod zoom;
pub mod utils;
pub mod outline;

use crate::rendering::utils::Size;
use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher};

use self::{utils::{Position, Zindex, Offset, Flip}, zoom::{setup, PostProcessPlugin}};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(AssetPlugin {
                // Hot reloading the shader works correctly
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..default()
            }))
            .add_systems(Startup, setup)
            //.add_systems(Update, update_settings)
            .add_plugins((crate::rendering::outline::OutlinePlugin, PostProcessPlugin))
            .register_type::<Position>()
            .register_type::<Zindex>()
            .register_type::<Offset>()
            .register_type::<Size>()
            .register_type::<Flip>()
            .add_systems(Update, crate::rendering::utils::set_zindex)
            .add_systems(PostUpdate, crate::rendering::utils::set_angle);
    }
}

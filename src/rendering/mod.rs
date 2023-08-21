pub mod zoom;
pub mod utils;
pub mod outline;

use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher};

use self::{utils::Zindex, zoom::{setup, ZoomPlugin, ZoomSettings}, outline::OutlinePlugin};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()).set(AssetPlugin {
                // Hot reloading the shader works correctly
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..default()
                }),
                OutlinePlugin,
                ZoomPlugin))
            .add_systems(Startup, setup)
            .register_type::<Zindex>()
            .add_systems(Update, crate::rendering::utils::set_zindex)
            .add_systems(Update, disable_pixel_perfect)
            .add_systems(PostUpdate, crate::rendering::utils::set_angle);
    }
}

fn disable_pixel_perfect(input: Res<Input<KeyCode>>, mut settings: Query<&mut ZoomSettings>)  {
    if input.just_pressed(KeyCode::P) {
        let mut set = settings.single_mut();
        set.enabled = if set.enabled == 1. { 0. } else { 1. };
    }
}

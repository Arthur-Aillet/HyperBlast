pub mod outline;
pub mod utils;
pub mod zoom;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};

use crate::camera::CameraData;

use bevy_pixel_perfect_zoom::{ZoomPlugin, ZoomSettings};

use self::{
    outline::OutlinePlugin,
    utils::Zindex,
    zoom::setup,
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "HyperBlast!".into(),
                        ..default()
                    }),
                    ..default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(AssetPlugin {
                        // Hot reloading the shader works correctly
                        watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                        ..default()
                    }),
                OutlinePlugin,
                ZoomPlugin,
            ))
            .add_systems(Startup, setup)
            .register_type::<Zindex>()
            .add_systems(Update, crate::rendering::utils::set_auto_zindex)
            .add_systems(Update, crate::rendering::utils::set_zindex)
            .add_systems(Update, disable_pixel_perfect)
            .add_systems(PostUpdate, crate::rendering::utils::set_angle);
    }
}

fn disable_pixel_perfect(
    input: Res<Input<KeyCode>>,
    mut set: Query<&mut ZoomSettings>,
    window_query: Query<&Window>,
    mut camera: Query<(
        &mut CameraData,
        &mut Transform,
        &mut OrthographicProjection,
        With<Camera2d>,
    )>,
) {
    if input.just_pressed(KeyCode::P) {
        let mut set = set.single_mut();
        for (mut camera_data, mut transform, mut projection, _) in &mut camera {
            camera_data.pixel = !camera_data.pixel;
            if camera_data.pixel {
                transform.translation = Vec3::new(0., 0., 999.9);
                projection.scale = 1.;
                let window = window_query.single();
                set.position = Vec2::new(
                    camera_data.pos.x / window.width(),
                    -camera_data.pos.y / window.height(),
                );
                set.intensity = camera_data.scale;
            } else {
                set.intensity = 1.;
                set.position = Vec2::new(0., 0.);
                transform.translation = camera_data.pos.extend(999.9);
                projection.scale = camera_data.scale;
            }
        }
    }
}

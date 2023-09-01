use bevy_pixel_perfect_zoom::PixelPerfectZoomSettings;

use bevy::prelude::*;


/// Set up a simple 3D scene
pub fn setup(mut commands: Commands) {
    commands.spawn((
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        PixelPerfectZoomSettings {
            intensity: 1.,
            position: Vec2::new(0., 0.),
        },
    ));
}

use bevy_pixel_perfect_zoom::PixelPerfectZoomSettings;

use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        PixelPerfectZoomSettings {
            intensity: 1.,
            position: Vec2::new(0., 0.),
        },
    ));
}

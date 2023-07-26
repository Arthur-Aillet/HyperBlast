use bevy::prelude::*;

use crate::{debug::{draw_rectangle, DebugLevel}, player::stats::PlayerStats, rendering::Position};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, resize_camera);
    }
}

pub fn maximum(first: f32, other: f32) -> f32 {
    if first > other {
        first
    } else if other > first {
        other
    } else if first == other {
        if first.is_sign_positive() && other.is_sign_negative() { first } else { other }
    } else {
        first + other
    }
}

fn resize_camera(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    window_query: Query<&Window>,
    debug_level: ResMut<DebugLevel>,
    mut query: Query<(&Position, With<PlayerStats>)>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, With<Camera2d>)>
) {
    let camera_position = Vec2::ZERO;

    for (mut camera_pos, mut camera_projection, _) in &mut camera {
        let average_player_positions: Vec2 = query.iter().map(|(Position(pos), _)| *pos).sum::<Vec2>() / query.iter().len() as f32;
        let mut max: Vec2 = Vec2::NEG_INFINITY;
        let mut min: Vec2 = Vec2::INFINITY;
        for (Position(pos), _) in &query {
            max = max.max(*pos);
            min = min.min(*pos);
        };

        let distance = max - min;
        let mut camera_size = if distance.x/distance.y > 16./9. {
            Vec2::new(distance.x , distance.x * 9./16.)
        } else {
            Vec2::new(distance.y * 16./9., distance.y)
        };

        (*camera_pos).translation = average_player_positions.extend(999.9);
        camera_size = camera_size.max(Vec2::new(1920., 1080.) / 12.) + Vec2::new(200., 200. * 9./16.);
        if window_query.single().width() / window_query.single().height() > 16./9. {
            camera_projection.scale = camera_size.y / window_query.single().height();
        } else {
            camera_projection.scale = camera_size.x / window_query.single().width();
        }
        if *debug_level == DebugLevel::Basic {
            draw_rectangle(&mut lines, average_player_positions, max - min, Color::PURPLE);
            draw_rectangle(&mut lines, average_player_positions, camera_size - Vec2::new(200., 200. * 9./16.), Color::RED);
            draw_rectangle(&mut lines, average_player_positions, camera_size, Color::GREEN);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    //camera.projection.scale = 0.4;
    commands.spawn(camera);
}


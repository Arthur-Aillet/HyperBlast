use bevy::prelude::*;

use crate::{
    debug::{draw_rectangle, DebugLevel},
    player::stats::PlayerStats,
    rendering::Position,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, calculate_camera_size)
            .add_systems(Update, resize_camera.after(calculate_camera_size));
    }
}

#[derive(Component)]
pub struct CameraData {
    pub pos: Vec2,
    pub scale: f32,
}

fn resize_camera(
    mut camera: Query<(
        &CameraData,
        &mut Transform,
        &mut OrthographicProjection,
        With<Camera2d>,
    )>,
) {
    for (camera_data, mut transform, mut projection, _) in &mut camera {
        transform.translation =
            transform.translation + (camera_data.pos.extend(999.9) - transform.translation) / 5.;
        projection.scale = projection.scale + (camera_data.scale - projection.scale) / 5.;
    }
}

fn calculate_camera_size(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    window_query: Query<&Window>,
    debug_level: ResMut<DebugLevel>,
    query: Query<(&Position, With<PlayerStats>)>,
    mut camera: Query<(&mut CameraData, With<Camera2d>)>,
) {
    for (mut camera_data, _) in &mut camera {
        let average_player_positions: Vec2 =
            query.iter().map(|(Position(pos), _)| *pos).sum::<Vec2>() / query.iter().len() as f32;
        let mut max: Vec2 = Vec2::NEG_INFINITY;
        let mut min: Vec2 = Vec2::INFINITY;
        for (Position(pos), _) in &query {
            max = max.max(*pos);
            min = min.min(*pos);
        }

        let distance = max - min;
        let mut camera_size = if distance.x / distance.y > 16. / 9. {
            Vec2::new(distance.x, distance.x * 9. / 16.)
        } else {
            Vec2::new(distance.y * 16. / 9., distance.y)
        };

        camera_data.pos = average_player_positions;
        camera_size =
            camera_size.max(Vec2::new(1920., 1080.) / 12.) + Vec2::new(200., 200. * 9. / 16.);
        if window_query.single().width() / window_query.single().height() > 16. / 9. {
            camera_data.scale = camera_size.y / window_query.single().height();
        } else {
            camera_data.scale = camera_size.x / window_query.single().width();
        }
        if *debug_level == DebugLevel::Basic {
            draw_rectangle(
                &mut lines,
                average_player_positions,
                max - min,
                Color::PURPLE,
            );
            draw_rectangle(
                &mut lines,
                average_player_positions,
                camera_size - Vec2::new(200., 200. * 9. / 16.),
                Color::RED,
            );
            draw_rectangle(
                &mut lines,
                average_player_positions,
                camera_size,
                Color::GREEN,
            );
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera).insert(CameraData {
        pos: Vec2::ZERO,
        scale: 1_f32,
    });
}

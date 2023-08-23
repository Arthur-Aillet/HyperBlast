use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    debug::{draw_rectangle, DebugLevel},
    player::stats::PlayerStats,
    rendering::zoom::ZoomSettings,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, calculate_camera_size)
            .add_systems(Update, resize_camera);
    }
}

#[derive(Component)]
pub struct CameraData {
    pub pos: Vec2,
    pub scale: f32,
    pub pixel: bool,
}

fn resize_camera(
    window_query: Query<&Window>,
    mut camera: Query<(
        &CameraData,
        &mut Transform,
        &mut OrthographicProjection,
        With<Camera2d>,
    )>,
    mut settings: Query<&mut ZoomSettings>,
) {
    for (camera_data, mut transform, mut projection, _) in &mut camera {
        let mut settings = settings.single_mut();

        if camera_data.pixel {
            projection.scale = 1.;
            let window = window_query.single();
            transform.translation = Vec3::new(0., 0., 999.9);
            settings.position = settings.position
                + (Vec2::new(
                    camera_data.pos.x / window.width(),
                    -camera_data.pos.y / window.height(),
                ) - settings.position)
                    / 5.;
            settings.intensity =
                settings.intensity + (camera_data.scale - settings.intensity) / 10.;
        } else {
            settings.intensity = 1.;
            settings.position = Vec2::new(0., 0.);
            transform.translation = transform.translation
                + (camera_data.pos.extend(999.9) - transform.translation) / 5.;
            projection.scale = projection.scale + (camera_data.scale - projection.scale) / 10.;
        }
    }
}

fn calculate_camera_size(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    window_query: Query<&Window>,
    debug_level: ResMut<DebugLevel>,
    query: Query<(&Transform, With<PlayerStats>)>,
    mut camera: Query<(&mut CameraData, With<Camera2d>)>,
) {
    for (mut camera_data, _) in &mut camera {
        let average_player_positions: Vec2 = query
            .iter()
            .map(|(pos, _)| pos.translation.xy())
            .sum::<Vec2>()
            / query.iter().len() as f32;
        let mut max: Vec2 = Vec2::NEG_INFINITY;
        let mut min: Vec2 = Vec2::INFINITY;
        for (pos, _) in &query {
            max = max.max(pos.translation.xy());
            min = min.min(pos.translation.xy());
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
        pixel: true,
    });
}

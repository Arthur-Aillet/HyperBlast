use bevy::prelude::*;

use crate::rendering::{Zindex, Position};

#[derive(Component)]
pub struct BulletStats {
    pub angle: f32,
    pub spread: f32,
    pub distance: f32,
    pub distance_traveled: f32,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub stats: BulletStats,
    pub sprite: SpriteBundle,
    pub zindex: Zindex,
    pub position: Position,
}

impl BulletBundle {
    pub fn marine_bullet(
        asset_server: &Res<AssetServer>,
        barrel_end: Vec2,
        angle: f32,
) -> Self {
        let texture: Handle<Image> = asset_server.load("bullet.png");

        BulletBundle {
            name: Name::new("Marine bullet"),
            position: Position(barrel_end),
            zindex: Zindex(150.),
            stats: BulletStats {
                distance_traveled: 0.,
                angle,
                spread: 0.5,
                distance: 20. * 8.,
                speed: 90.,
            },
            sprite: SpriteBundle {
                texture,
                ..default()
            }
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut BulletStats,
        &mut Position,
    )>,
) {
    for (entity, mut stats, mut position) in &mut query {
        (*position).0 += Vec2::from_angle(stats.angle) * stats.speed * time.delta_seconds();
        (*stats).distance_traveled += stats.speed * time.delta_seconds();
        if (*stats).distance_traveled > stats.distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

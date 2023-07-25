use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    physics::TesselatedCollider,
    rendering::{Offset, Position, Size, Zindex},
};

use super::{
    stats::PlayerStats,
    weapon::{GunEntity, GunStats},
};

#[derive(Component)]
pub struct BulletStats {
    pub angle: f32,
    pub spread: f32,
    pub distance: f32,
    pub distance_traveled: f32,
    pub speed: f32,
    pub owner: Entity,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub stats: BulletStats,
    pub sprite: SpriteBundle,
    pub zindex: Zindex,
    pub position: Position,
    pub collider: TesselatedCollider,
    pub offset: Offset,
    pub size: Size,
}

impl BulletBundle {
    pub fn marine_bullet(
        asset_server: &Res<AssetServer>,
        barrel_end: Vec2,
        angle: f32,
        player: Entity,
    ) -> Self {
        let texture: Handle<Image> = asset_server.load("bullet.png");

        BulletBundle {
            offset: Offset(Vec2::new(3., 3.)),
            name: Name::new("Marine bullet"),
            position: Position(barrel_end),
            zindex: Zindex(150.),
            size: Size(Vec2 { x: 6., y: 6. }),
            stats: BulletStats {
                owner: player,
                distance_traveled: 0.,
                angle,
                spread: 0.5,
                distance: 20. * 8.,
                speed: 90.,
            },
            sprite: SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(barrel_end.extend(150.)), // TODO: SHOULD'NT EXIST, SHOULD BE PROPERLY FIXED BY "update_transform" system
                ..default()
            },
            collider: TesselatedCollider {
                texture,
                offset: Vec2::new(-3., 3.),
            },
        }
    }
}

fn player_bullet_collision(
    commands: &mut Commands,
    player: (Entity, (&GunEntity, Mut<'_, PlayerStats>)),
    bullet: (Entity, Mut<'_, BulletStats>),
    gun: Mut<'_, GunStats>,
) {
    let (player_id, (_, mut player_stats)) = player;
    let (bullet_id, bullet_stats) = bullet;
    let gun_stats = gun;

    if bullet_stats.owner != player_id {
        commands.entity(bullet_id).despawn();
        player_stats.health -=
            (gun_stats.damage + player_stats.damages_added) * player_stats.damages_multiplier;
    }
}

pub fn detect_collision_bullets(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bullets: Query<&mut BulletStats>,
    mut players: Query<(&GunEntity, &mut PlayerStats)>,
    mut guns: Query<&mut GunStats>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            let bullet = if let Ok(bullet_found) = bullets.get_mut(*entity1) {
                Some((*entity1, bullet_found))
            } else if let Ok(bullet_found) = bullets.get_mut(*entity2) {
                Some((*entity2, bullet_found))
            } else {
                None
            };
            let player = if let Ok(player_found) = players.get_mut(*entity1) {
                Some((*entity1, player_found))
            } else if let Ok(player_found) = players.get_mut(*entity2) {
                Some((*entity2, player_found))
            } else {
                None
            };

            if let Some(bullet) = bullet {
                if let Some(player) = player {
                    let gun = guns.get_mut(player.1 .0 .0);
                    player_bullet_collision(
                        &mut commands,
                        player,
                        bullet,
                        gun.expect("Gun not found"),
                    );
                }
            }
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletStats, &mut Position)>,
) {
    for (entity, mut stats, mut position) in &mut query {
        position.0 += Vec2::from_angle(stats.angle) * stats.speed * time.delta_seconds();
        stats.distance_traveled += stats.speed * time.delta_seconds();
        if stats.distance_traveled > stats.distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

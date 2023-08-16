use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    physics::TesselatedCollider,
    rendering::{Offset, Position, Size, Zindex},
};

use crate::player::inventory::item_manager::Items;

use super::{
    assets::GunAssets,
    stats::PlayerStats,
    weapon::{GunEntity, GunStats}, roll::RollStats, inventory::inventory_manager::Inventory,
};

#[derive(Component)]
pub struct BulletStats {
    pub angle: f32,
    pub spread: f32,
    pub distance: f32,
    pub distance_traveled: f32,
    pub speed: f32,
    pub mercury_amount: usize,
    pub owner: Entity,
}

#[derive(Bundle)]
pub struct SphereCollider {
    pub collider: Collider,
    pub active: ActiveEvents,
    pub rigid: RigidBody,
    pub gravity: GravityScale,
    pub mass: ColliderMassProperties,
    pub locked_trans: LockedAxes,
}

impl SphereCollider {
    pub fn new() -> SphereCollider {
        SphereCollider {
            collider: Collider::ball(3.5),
            active: ActiveEvents::COLLISION_EVENTS,
            rigid: RigidBody::Dynamic,
            gravity: GravityScale(0.0),
            mass: ColliderMassProperties::Density(0.0),
            locked_trans: LockedAxes::TRANSLATION_LOCKED,
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub stats: BulletStats,
    pub sprite: SpriteBundle,
    pub zindex: Zindex,
    pub position: Position,
    pub collider: SphereCollider,
    pub offset: Offset,
    pub size: Size,
}

impl BulletBundle {
    pub fn marine_bullet(
        assets: &Res<GunAssets>,
        barrel_end: Vec2,
        angle: f32,
        inventory: &Inventory,
        player: Entity,
    ) -> Self {
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
                speed: 90. / (inventory.amount(Items::Mercury) as f32 * 3. + 1.),
                mercury_amount: inventory.amount(Items::Mercury),
            },
            sprite: SpriteBundle {
                texture: assets.marine_bullet.clone(),
                transform: Transform::from_translation(barrel_end.extend(150.)), // TODO: SHOULD'NT EXIST, SHOULD BE PROPERLY FIXED BY "update_transform" system
                ..default()
            },
            collider: SphereCollider::new(),
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
        player_stats.current_health -=
            (gun_stats.damage + player_stats.damages_added) * player_stats.damages_multiplier;
    }
}

pub fn detect_collision_bullets(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bullets: Query<&mut BulletStats>,
    mut players: Query<(&GunEntity, &mut PlayerStats, Without<RollStats>)>,
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
            let player = if let Ok((gun, stats,_)) = players.get_mut(*entity1) {
                Some((*entity1, (gun, stats)))
            } else if let Ok((gun, stats,_)) = players.get_mut(*entity2) {
                Some((*entity2, (gun, stats)))
            } else {
                None
            };

            if let Some(bullet) = bullet {
                if let Some(player) = player {
                    let gun = guns.get_mut(player.1.0.0);
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
        for _ in 0..stats.mercury_amount {
            stats.speed += time.delta_seconds() * 70.;
        }
        position.0 += Vec2::from_angle(stats.angle) * stats.speed * time.delta_seconds();
        stats.distance_traveled += stats.speed * time.delta_seconds();
        if stats.distance_traveled > stats.distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

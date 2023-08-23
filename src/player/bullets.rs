use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::map::colliders::WallCollider;
use crate::physics::collision_get;
use crate::rendering::utils::Zindex;

use crate::player::inventory::item_manager::Items;

use super::{
    inventory::{inventory_manager::Inventory, weapon_manager::GunAssets},
    roll::RollStats,
    stats::PlayerStats,
    weapon::{GunEntity, GunStats}, setup::PlayerCollider,
};

#[derive(Component)]
pub struct BulletStats {
    pub angle: f32,
    pub damages: f32,
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
    pub sensor: Sensor,
    pub velocity: Velocity,
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
            sensor: Sensor,
            velocity: Velocity::default(),
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub stats: BulletStats,
    pub sprite: SpriteBundle,
    pub zindex: Zindex,
    pub collider: SphereCollider,
}

impl BulletBundle {
    pub fn marine_bullet(
        assets: &Res<GunAssets>,
        barrel_end: Vec2,
        angle: f32,
        inventory: &Inventory,
        gun_stats: &GunStats,
        player_stats: &PlayerStats,
        player: Entity,
        spd: f32,
    ) -> Self {
        BulletBundle {
            name: Name::new("Marine bullet"),
            zindex: Zindex(45.),
            stats: BulletStats {
                owner: player,
                distance_traveled: 0.,
                angle,
                distance: gun_stats.distance,
                speed: spd / (inventory.amount(Items::Mercury) as f32 * 3. + 1.),
                mercury_amount: inventory.amount(Items::Mercury),
                damages: (gun_stats.damage + player_stats.damages_added) * player_stats.damages_multiplier,
            },
            sprite: SpriteBundle {
                texture: assets.marine_bullet.clone(),
                transform: Transform::from_translation(barrel_end.extend(150.)),
                ..default()
            },
            collider: SphereCollider::new(),
        }
    }
}

pub fn detect_collision_bullets(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bullets: Query<(Entity, &mut BulletStats)>,
    mut players: Query<(Entity, &mut PlayerStats, Without<RollStats>)>,
    mut player_collider: Query<(&Parent, With<PlayerCollider>)>,
    mut walls: Query<With<WallCollider>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if let Some((bullet_id, bullet_stats)) = collision_get!(bullets, entity1, entity2) {
                if let Some(_) = collision_get!(walls, entity1, entity2) {
                    commands.entity(bullet_id).despawn();
                } else {
                    if let Some((player, _)) = collision_get!(player_collider, entity1, entity2) {
                        if let Ok((id, mut stats, _)) = players.get_mut(player.get()) {
                            if bullet_stats.owner != id {
                                commands.entity(bullet_id).despawn();
                                stats.current_health -= bullet_stats.damages;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletStats, &mut Velocity)>,
) {
    for (entity, mut stats, mut vel) in &mut query {
        for _ in 0..stats.mercury_amount {
            stats.speed += time.delta_seconds() * 70.;
        }
        vel.linvel = Vec2::from_angle(stats.angle) * stats.speed;
        stats.distance_traveled += stats.speed * time.delta_seconds();
        if stats.distance_traveled > stats.distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

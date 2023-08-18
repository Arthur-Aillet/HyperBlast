use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::map::colliders::WallCollider;
use crate::physics::{self, collision_get};
use crate::rendering::{Offset, Position, Size, Zindex};

use crate::player::inventory::item_manager::Items;

use super::{
    assets::GunAssets,
    inventory::inventory_manager::Inventory,
    roll::RollStats,
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
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub stats: BulletStats,
    pub sprite: SpriteBundle,
    pub zindex: Zindex,
    //pub position: Position,
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
            //position: Position(barrel_end),
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

pub fn detect_collision_bullets(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bullets: Query<(Entity, &mut BulletStats)>,
    mut players: Query<(Entity, &GunEntity, &mut PlayerStats, Without<RollStats>)>,
    mut walls: Query<With<WallCollider>>,
    mut guns: Query<&mut GunStats>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if let Some((bullet_id, bullet_stats)) = collision_get!(bullets, entity1, entity2) {
                if let Some(_) = collision_get!(walls, entity1, entity2) {
                    commands.entity(bullet_id).despawn();

                } else if let Some((id, gun, mut stats, _)) = collision_get!(players, entity1, entity2) {
                    if let Ok(gun_stats) = guns.get_mut(gun.0) {
                        if bullet_stats.owner != id {
                            commands.entity(bullet_id).despawn();
                            stats.current_health -=
                                (gun_stats.damage + stats.damages_added) * stats.damages_multiplier;
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

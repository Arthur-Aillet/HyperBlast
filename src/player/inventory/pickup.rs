use std::f32::INFINITY;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    player::{
        input::PlayerActions,
        inventory::{inventory_manager::Inventory, item_manager::Items},
        stats::PlayerStats,
    },
    rendering::outline::Outline,
    rendering::utils::Zindex,
};

use super::{
    assets::ItemsAssets,
    weapon_manager::{GunAssets, Guns},
    PickupItemEvent, PickupWeaponEvent,
};

const PICKUP_RANGE: f32 = 25. * 1.5;

pub fn update_pickup(
    time: Res<Time>,
    mut ev_pickup_i: EventWriter<PickupItemEvent>,
    mut ev_pickup_w: EventWriter<PickupWeaponEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<Outline>>,
    mut pickups: Query<(
        Entity,
        &Handle<Outline>,
        &mut Transform,
        &mut Pickup,
        &mut Zindex,
        Without<PlayerStats>,
    )>,
    mut players: Query<(
        Entity,
        &mut Transform,
        &mut Inventory,
        &ActionState<PlayerActions>,
        With<PlayerStats>,
    )>,
) {
    for (_, outline, mut pos, pickup, mut zindex, _) in &mut pickups {
        let float = ((time.elapsed_seconds() + pickup.anim_offset) * 3.).sin() / 10.;
        pos.translation.y += float;
        zindex.0 = float + 5.;

        if let Some(material) = materials.get_mut(outline) {
            material.color = Color::WHITE.with_a(0.);
        }
    }

    for (entity, player_pos, mut inventory, actions, _) in &mut players {
        let mut nearest: Option<Entity> = None;
        let mut distance: f32 = INFINITY;

        for (entity, _, pos, _, _, _) in &mut pickups {
            let current_distance = pos.translation.xy().distance(player_pos.translation.xy());

            if current_distance < distance && current_distance < PICKUP_RANGE {
                distance = current_distance;
                nearest = Some(entity);
            }
        }

        if let Some(valid_pickup) = nearest {
            if let Ok((_, outline, _, pickup, _, _)) = pickups.get(valid_pickup) {
                if let Some(material) = materials.get_mut(outline) {
                    material.color = Color::WHITE;
                }
                if actions.just_pressed(PlayerActions::Pickup) {
                    match &pickup.pickup_type {
                        PickupType::Gun(gun) => {
                            ev_pickup_w.send(PickupWeaponEvent(*gun, entity, valid_pickup));
                        }
                        PickupType::Item(item) => {
                            ev_pickup_i.send(PickupItemEvent(*item, entity));
                            inventory.add(*item);
                            commands.entity(valid_pickup).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Ground;

pub fn spawn_items(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    assets: Res<ItemsAssets>,
    gun_assets: Res<GunAssets>,
) {
    commands.spawn(Ground)
        .insert(SpatialBundle::default())
        .insert(Name::new("Ground"))
        .with_children(|parent| {
        let len = Items::iter().count();
        for (x, item) in Items::iter().enumerate() {
            for _ in 0..10 {
                parent.spawn(item.to_pickup(
                    Vec2::new(-(len as f32 * 30.) / 2. + x as f32 * 30. + 15., 80.),
                    &mut meshes,
                    &mut materials,
                    &assets,
                ));
            }
        }

        let len: usize = Guns::iter().count();
        for (x, gun) in Guns::iter().enumerate() {
            for _ in 0..10 {
                parent.spawn(gun.to_pickup(
                    Vec2::new(-(len as f32 * 30.) / 2. + x as f32 * 30. + 15., 110.),
                    &mut meshes,
                    &mut materials,
                    &gun_assets,
                ));
            }
        }
    }
    );
}

pub enum PickupType {
    Gun(Guns),
    Item(Items),
}

#[derive(Component)]
pub struct Pickup {
    pub anim_offset: f32,
    pub pickup_type: PickupType,
}

#[derive(Bundle)]
pub struct PickupBundle {
    pub name: bevy::core::Name,
    pub material: MaterialMesh2dBundle<Outline>,
    pub zindex: Zindex,
    pub pickup: Pickup,
}

impl PickupBundle {
    pub fn create(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprite: Handle<Image>,
        size: Vec2,
        name: String,
        pos: Vec2,
        object_type: PickupType,
    ) -> PickupBundle {
        let mut rng = rand::thread_rng();
        let place_rng = rng.gen::<f32>() * 100.;

        PickupBundle {
            name: bevy::core::Name::new(name),
            material: MaterialMesh2dBundle {
                transform: Transform::default()
                    .with_scale(size.extend(0.))
                    .with_translation(pos.floor().extend(0.)),
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::splat(2.))))
                    .into(),
                material: materials.add(Outline {
                    color: Color::WHITE,
                    size,
                    thickness: 1.,
                    color_texture: sprite,
                }),
                ..default()
            },
            zindex: Zindex(0.),
            pickup: Pickup {
                anim_offset: place_rng,
                pickup_type: object_type,
            },
        }
    }
}

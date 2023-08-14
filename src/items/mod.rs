use std::f32::NEG_INFINITY;

use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;

use crate::{rendering::{Zindex, Position}, outline::Outline, player::{stats::PlayerStats, input::PlayerActions, inventory::{Inventory, Items}}};

const PICKUP_RANGE: f32 = 25. * 1.5;

#[derive(AssetCollection, Resource)]
pub struct ItemsAssets {
    #[asset(path = "items/Item__65.png")]
    pub cheese: Handle<Image>,
    #[asset(path = "items/Item__64.png")]
    pub apple: Handle<Image>,
    #[asset(path = "items/Item__63.png")]
    pub item: Handle<Image>,
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ItemsAssets>()
            .add_systems(Startup, setup_item)
            .add_systems(Update, update_pickup);
    }
}

fn update_pickup(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<Outline>>,
    mut pickups: Query<(Entity, &Handle<Outline>, &mut Position, &mut Pickup, Without<PlayerStats>)>,
    mut players: Query<(&mut Position, &mut Inventory, &ActionState<PlayerActions>, With<PlayerStats>)>,
) {
    for (_, outline, mut pos, pickup, _) in &mut pickups {
        pos.0.y += ((time.elapsed_seconds() + pickup.anim_offset) * 3.).sin() / 10.;

        if let Some(material) = materials.get_mut(outline) {
            material.color = Color::WHITE.with_a(0.);
        }
    }

    for (player_pos, mut inventory, actions, _) in &mut players {
        let mut nearest: Option<Entity> = None;
        let mut distance: f32 = NEG_INFINITY;

        for (entity, _, pos, _, _) in &mut pickups {
            let current_distance = pos.0.distance(player_pos.0);

            if current_distance > distance && current_distance < PICKUP_RANGE {
                distance = current_distance;
                nearest = Some(entity);
            }
        }

        if let Some(valid_pickup) = nearest {
            if let Ok((_, outline, _, pickup, _)) = pickups.get(valid_pickup) {
                if let Some(material) = materials.get_mut(outline) {
                    material.color = Color::WHITE;
                }
                if actions.just_pressed(PlayerActions::Pickup) {
                    match &pickup.pickup_type {
                        PickupType::Weapon => todo!(),
                        PickupType::Item(item) => inventory.add(item),
                    }
                    commands.entity(valid_pickup).despawn_recursive();
                }
            }
        }
    }
}

fn setup_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    assets: Res<ItemsAssets>
) {
    let mut rng = rand::thread_rng();
    let cheese_rng = rng.gen::<f32>() * 100.;
    let apple_rng = rng.gen::<f32>() * 100.;

    commands.spawn(PickupBundle {
        name: bevy::core::Name::new("Cheese"),
        material: MaterialMesh2dBundle {
            transform: Transform::default().with_scale(Vec3::splat(16.)),
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(2.)))).into(),
            material: materials.add(Outline {
                color: Color::WHITE,
                size: Vec2::new(16., 16.),
                thickness: 1.,
                color_texture: assets.cheese.clone(),
            }),
            ..default()
        },
        zindex: Zindex(0.),
        position: Position(Vec2::new(40., 40.)),
        pickup: Pickup { anim_offset: cheese_rng, pickup_type: PickupType::Weapon }
    });

    commands.spawn(PickupBundle {
        name: bevy::core::Name::new("Apple"),
        material: MaterialMesh2dBundle {
            transform: Transform::default().with_scale(Vec3::splat(16.)),
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(2.)))).into(),
            material: materials.add(Outline {
                color: Color::WHITE,
                size: Vec2::new(16., 16.),
                thickness: 1.,
                color_texture: assets.apple.clone(),
            }),
            ..default()
        },
        zindex: Zindex(2.),
        position: Position(Vec2::new(-40., 40.)),
        pickup: Pickup { anim_offset: apple_rng, pickup_type: PickupType::Item(Items::Null) },
    });
}

pub enum PickupType {
    Weapon,
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
    pub position: Position,
    pub pickup: Pickup,
}

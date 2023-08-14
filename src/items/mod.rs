use std::f32::NEG_INFINITY;

use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{rendering::{Zindex, Position}, outline::Outline, player::stats::PlayerStats};

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
    mut materials: ResMut<Assets<Outline>>,
    time: Res<Time>,
    mut pickups: Query<(Entity, &Handle<Outline>, &mut Position, Without<PlayerStats>)>,
    mut players: Query<(&mut Position, With<PlayerStats>)>,
) {
    for (_, outline, mut pos, _) in &mut pickups {
        pos.0.y += (time.elapsed_seconds() * 3.).sin() / 10.;

        if let Some(material) = materials.get_mut(outline) {
            material.color = Color::WHITE.with_a(0.);
        }
    }

    for (player_pos, _) in &mut players {
        let mut nearest: Option<Entity> = None;
        let mut distance: f32 = NEG_INFINITY;

        for (entity, _, pos, _) in &mut pickups {
            let current_distance = pos.0.distance(player_pos.0);

            if current_distance > distance && current_distance < PICKUP_RANGE {
                distance = current_distance;
                nearest = Some(entity);
            }
        }

        if let Some(valid_pickup) = nearest {
            if let Ok((_, outline, _, _)) = pickups.get(valid_pickup) {
                if let Some(material) = materials.get_mut(outline) {
                    material.color = Color::WHITE;
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
    commands.spawn(Pickup {
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
        zindex: Zindex(2.),
        position: Position(Vec2::new(40., 40.)),
    });

    commands.spawn(Pickup {
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
    });
}

#[derive(Bundle)]
pub struct Pickup {
    pub name: bevy::core::Name,
    pub material: MaterialMesh2dBundle<Outline>,
    pub zindex: Zindex,
    pub position: Position,
}

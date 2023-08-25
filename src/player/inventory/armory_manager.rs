use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;

use crate::{
    player::{input::PlayerActions, weapon::{GunEntity, GunStats}, stats::PlayerStats, inventory::pickup::PickupType},
    rendering::{outline::Outline, utils::set_anchor},
};

use super::{
    DroppedWeaponEvent, PickupWeaponEvent, pickup::{Pickup, Ground},
};

#[derive(Component)]
pub struct Armory {
    pub content: Vec<Entity>,
    pub current_weapon_index: usize,
}

impl Armory {
    pub fn new() -> Armory {
        Armory {
            content: Vec::new(),
            current_weapon_index: 0,
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.content.push(entity);
    }

    pub fn next(&mut self) -> usize {
        self.current_weapon_index += 1;
        if self.current_weapon_index > self.content.len() {
            self.current_weapon_index = 0;
        }
        self.current_weapon_index
    }
}

pub fn pickup_weapon(
    mut commands: Commands,
    mut pickup_event: EventReader<PickupWeaponEvent>,
    mut materials: ResMut<Assets<Outline>>,
    mut guns: Query<(&mut Visibility,(Without<Pickup>, With<GunStats>))>,
    mut pickups: Query<(
        &Handle<Outline>,
        &mut Transform,
        &GunStats,
        Without<PlayerStats>,
    )>,
    mut players: Query<(&mut Armory, Option<&mut GunEntity>)>,
) {
    for PickupWeaponEvent(player_id, pickup_id) in pickup_event.iter() {
        if let Ok((mut armory, gun_entity, )) = players.get_mut(*player_id) {
            armory.add(*pickup_id);
            armory.current_weapon_index = armory.content.len() - 1;
            commands.entity(*player_id).add_child(*pickup_id);
            if let Ok((outline, mut transfrom, stats, _)) = pickups.get_mut(*pickup_id) {
                if let Some(material) = materials.get_mut(outline) {
                    let texture = material.color_texture.clone();
                    material.color = Color::WHITE.with_a(0.);
                    transfrom.translation = Vec3::new(8., 0., 50.);
                    commands.entity(*pickup_id)
                        .insert(SpriteBundle {
                            sprite: Sprite {
                                anchor: set_anchor(stats.handle_position, stats.size),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(8., 0., 50.)),
                            texture,
                            ..default()
                        })
                        .remove::<Pickup>()
                        .remove::<Handle<Outline>>()
                        .remove::<Mesh2dHandle>();
                }
            }
            if let Some(mut holster) = gun_entity {
                if let Ok((mut visibility, _)) = guns.get_mut(holster.0) {
                    *visibility = Visibility::Hidden;
                }
                holster.0 = *pickup_id;
            } else {
                commands.entity(*player_id).insert(GunEntity(*pickup_id));
            }
        }
    }
}

pub fn switch_weapon(
    mut guns: Query<(&mut Visibility,(Without<Pickup>, With<GunStats>))>,
    mut query: Query<(
        &ActionState<PlayerActions>,
        &mut Armory,
        &mut GunEntity,
    )>,
) {
    for (action, mut armory, mut holster) in &mut query {
        if armory.content.len() <= 1 {
            return;
        }
        if action.just_pressed(PlayerActions::NextWeapon) {
            armory.current_weapon_index += 1;
            if armory.current_weapon_index >= armory.content.len() {
                armory.current_weapon_index = 0;
            }
        } else if action.just_pressed(PlayerActions::LastWeapon) {
            if armory.current_weapon_index == 0 {
                armory.current_weapon_index = armory.content.len() - 1;
            } else {
                armory.current_weapon_index -= 1;
            }
        } else {
            return;
        }
        if let Some(other_entity) = armory.content.get(armory.current_weapon_index) {
            if let Ok((mut visibility, _)) = guns.get_mut(holster.0) {
                *visibility = Visibility::Hidden;
            }
            if let Ok((mut visibility, _)) = guns.get_mut(*other_entity) {
                *visibility = Visibility::Inherited;
            }
            holster.0 = *other_entity;
        }
    }
}

pub fn drop_weapon(
    mut commands: Commands,
    mut ev_drop: EventWriter<DroppedWeaponEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    mut guns: Query<(&mut Visibility, &Handle<Image>, &GunStats, Without<Pickup>)>,
    ground: Query<(Entity, With<Ground>)>,
    mut query: Query<(
        Entity,
        &ActionState<PlayerActions>,
        &Transform,
        &mut Armory,
        Option<&mut GunEntity>,
    )>,
) {
    for (entity, action, pos, mut armory, holster_maybe) in &mut query {
        if action.just_pressed(PlayerActions::DropWeapon) {
            let mut rng = rand::thread_rng();
            let place_rng = rng.gen::<f32>() * 100.;

            if armory.content.len() < 1 {
                return
            }
            let current_index = armory.current_weapon_index;
            let gun = armory.content.remove(current_index);

            ev_drop.send(DroppedWeaponEvent(entity, gun));
            let (_, moved_sprite, moved_gun_stats, _) = guns.get(gun).expect("Gun hold innacessible");
            commands.entity(gun)
                .insert(Pickup {
                    anim_offset: place_rng,
                    pickup_type: PickupType::Gun,
                })
                .insert(MaterialMesh2dBundle {
                    transform: Transform::default()
                        .with_scale(moved_gun_stats.size.extend(0.))
                        .with_translation(pos.translation.floor()),
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(Vec2::splat(2.))))
                        .into(),
                    material: materials.add(Outline {
                        color: Color::WHITE,
                        size: moved_gun_stats.size,
                        thickness: 1.,
                        color_texture: moved_sprite.clone(),
                    }),
                    ..default()
                })
                .remove::<Sprite>()
                .remove::<Handle<Image>>();
            let ground_id = ground.single().0;
            commands.entity(ground_id).add_child(gun);

            if armory.current_weapon_index as i32 >= armory.content.len() as i32 - 1 {
                armory.current_weapon_index = 0;
            }
            if let Some(&new_gun) = armory.content.get(armory.current_weapon_index) {
                if let Some(mut holster) = holster_maybe {
                    holster.0 = new_gun;
                } else {
                    commands.entity(entity).insert(GunEntity(new_gun));
                }
                let (mut visibility, _, _, _) = guns.get_mut(new_gun).expect("New gun cant be tracked");
                *visibility = Visibility::Inherited;
            } else {
                commands.entity(entity).remove::<GunEntity>();
            }
        }
    }
}

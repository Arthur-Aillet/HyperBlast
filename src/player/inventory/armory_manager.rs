use bevy::{math::Vec3Swizzles, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    player::{input::PlayerActions, weapon::GunEntity},
    rendering::outline::Outline,
};

use super::{
    weapon_manager::{GunAssets, Guns},
    DroppedWeaponEvent, PickupWeaponEvent,
};

#[derive(Component)]
pub struct Armory {
    pub content: Vec<Guns>,
    pub current_weapon_index: usize,
}

impl Armory {
    pub fn new() -> Armory {
        Armory {
            content: Vec::new(),
            current_weapon_index: 0,
        }
    }

    pub fn add(&mut self, name: Guns) {
        self.content.push(name);
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
    mut pickup: EventReader<PickupWeaponEvent>,
    mut players: Query<Option<&mut GunEntity>>,
    assets: Res<GunAssets>,
) {
    for PickupWeaponEvent(gun_name, player_id) in pickup.iter() {
        let id = commands.spawn(gun_name.to_gun_bundle(&assets)).id();
        commands.entity(*player_id).add_child(id);
        if let Ok(player) = players.get_mut(*player_id) {
            if let Some(mut holster) = player {
                commands.entity(holster.0).despawn_recursive();
                holster.0 = id;
            } else {
                commands.entity(*player_id).insert(GunEntity(id));
            }
        }
    }
}

pub fn switch_weapon(
    mut commands: Commands,
    sprites: Res<GunAssets>,
    mut query: Query<(
        Entity,
        &ActionState<PlayerActions>,
        &mut Armory,
        &mut GunEntity,
    )>,
) {
    for (entity, action, mut armory, mut holster) in &mut query {
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
        if let Some(new_gun) = armory.content.get(armory.current_weapon_index) {
            let spawned_id = commands
                .spawn(new_gun.to_gun_bundle(&sprites))
                .id();
            commands.entity(entity).add_child(spawned_id);
            commands.entity(holster.0).despawn_recursive();
            holster.0 = spawned_id;
        }
    }
}

pub fn drop_weapon(
    mut commands: Commands,
    mut ev_drop: EventWriter<DroppedWeaponEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    sprites: Res<GunAssets>,
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
            if armory.content.len() < 1 {
                return
            }
            let current_index = armory.current_weapon_index;
            let gun = armory.content.remove(current_index);

            ev_drop.send(DroppedWeaponEvent(gun, entity));
            commands.spawn(gun.to_pickup(
                pos.translation.xy(),
                &mut meshes,
                &mut materials,
                &sprites,
            ));

            if armory.current_weapon_index as i32 >= armory.content.len() as i32 - 1 {
                armory.current_weapon_index = 0;
            }
            if let Some(new_gun) = armory.content.get(armory.current_weapon_index) {
                let spawned_id = commands
                    .spawn(new_gun.to_gun_bundle(&sprites))
                    .id();

                commands.entity(entity).add_child(spawned_id);
                if let Some(mut holster) = holster_maybe {
                    commands.entity(holster.0).despawn_recursive();
                    holster.0 = spawned_id;
                } else {
                    commands.entity(entity).insert(GunEntity(spawned_id));
                }
            } else if let Some(holster) = holster_maybe {
                commands.entity(holster.0).despawn_recursive();
                commands.entity(entity).remove::<GunEntity>();
            }
        }
    }
}

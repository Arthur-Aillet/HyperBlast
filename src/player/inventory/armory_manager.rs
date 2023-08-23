use bevy::{prelude::*, transform::commands};
use image::Pixel;

use crate::player::{weapon::{GunStats, GunEntity}, stats::PlayerStats};

use super::{weapon_manager::{Guns, GunAssets}, PickupWeaponEvent};

#[derive(Component)]
pub struct Armory {
    pub content: Vec<Guns>,
    pub current_weapon_index: u32,
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
}

pub fn pickup_weapon(
    mut commands: Commands,
    mut pickup: EventReader<PickupWeaponEvent>,
    mut players: Query<Option<&mut GunEntity>>,
    assets: Res<GunAssets>
) {
    for PickupWeaponEvent(gun_name, player_id) in pickup.iter() {
        let id = commands.spawn(gun_name.to_gun_bundle(&assets)).id();
        commands.entity(*player_id).add_child(id);
        if let Ok(player) = players.get_mut(*player_id) {
            if let Some(mut player) = player {
                commands.entity(player.0).despawn_recursive();
                player.0 = id;
            } else {
                println!("Here :DDD");
                commands.entity(*player_id).insert(GunEntity(id));
            }
        }
    }
}

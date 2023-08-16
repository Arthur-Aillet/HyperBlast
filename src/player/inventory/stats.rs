use bevy::prelude::*;

use crate::player::stats::PlayerStats;

use super::{PickupEvent, DroppedEvent};
use super::item_manager::Items;

pub fn pickup_events(
    mut pickup: EventReader<PickupEvent>,
    mut players: Query<&mut PlayerStats>,
) {
    for PickupEvent(item, player) in pickup.iter() {
        if let Ok(mut stats) = players.get_mut(*player) {
            match item {
                Items::HealthApple => {
                    stats.max_health += 50.;
                    stats.current_health += 50.
                },
                Items::Mercury => {},
            }
        }
    }
}

pub fn drop_events(
    mut drop: EventReader<DroppedEvent>,
    mut players: Query<&mut PlayerStats>,
) {
    for DroppedEvent(item, player) in drop.iter() {
        if let Ok(mut stats) = players.get_mut(*player) {
            match item {
                Items::HealthApple => {
                    stats.max_health -= 50.;
                    stats.current_health -= 50.
                },
                Items::Mercury => {},
            }
        }
    }
}

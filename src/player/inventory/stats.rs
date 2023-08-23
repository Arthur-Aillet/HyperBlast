use bevy::prelude::*;

use crate::player::stats::PlayerStats;

use super::item_manager::Items;
use super::{DroppedItemEvent, PickupItemEvent};

pub fn pickup_events(mut pickup: EventReader<PickupItemEvent>, mut players: Query<&mut PlayerStats>) {
    for PickupItemEvent(item, player) in pickup.iter() {
        if let Ok(mut stats) = players.get_mut(*player) {
            match item {
                Items::HealthApple => {
                    stats.max_health += 50.;
                    stats.current_health += 50.
                }
                _ => {}
            }
        }
    }
}

pub fn drop_events(mut drop: EventReader<DroppedItemEvent>, mut players: Query<&mut PlayerStats>) {
    for DroppedItemEvent(item, player) in drop.iter() {
        if let Ok(mut stats) = players.get_mut(*player) {
            match item {
                Items::HealthApple => {
                    stats.max_health -= 50.;
                    stats.current_health -= 50.
                }
                _ => {}
            }
        }
    }
}

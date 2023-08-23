use bevy::prelude::*;
use bevy::time::Stopwatch;
use leafwing_input_manager::prelude::*;

use crate::{
    player::{
        stats::PlayerStats,
        weapon::{GunEntity, GunStats},
    },
    rendering::utils::Angle,
};

use crate::player::input::PlayerActions;
use crate::player::roll::RollStats;

#[derive(Component)]
pub struct ReloadStats {
    pub since: Stopwatch,
    pub gun: GunEntity,
}

impl ReloadStats {
    pub fn new(entity: GunEntity) -> Self {
        ReloadStats {
            since: Stopwatch::new(),
            gun: entity,
        }
    }
}

pub fn start_reload(
    players: Query<(
        Entity,
        &GunEntity,
        &ActionState<PlayerActions>,
        Without<ReloadStats>,
    )>,
    guns: Query<(&mut GunStats, Without<PlayerStats>)>,
    mut commands: Commands,
) {
    for (entity, gun_id, player_actions, _) in &players {
        if let Ok((gunstats, _)) = guns.get(gun_id.0) {
            if player_actions.pressed(PlayerActions::Reload)
                || (player_actions.just_pressed(PlayerActions::Shoot) && gunstats.mag_ammo == 0)
            {
                if gunstats.mag_ammo < gunstats.mag_size && (gunstats.ammo > 0 || gunstats.infinite)
                {
                    let relaod_stats = ReloadStats::new(gun_id.clone());
                    commands.entity(entity).insert(relaod_stats);
                }
            }
        }
    }
}

pub fn reload(
    time: Res<Time>,
    mut players: Query<(
        Entity,
        &GunEntity,
        &mut PlayerStats,
        Option<&RollStats>,
        &mut ReloadStats,
    )>,
    mut guns: Query<(&mut GunStats, &mut Angle, Without<PlayerStats>)>,
    mut commands: Commands,
    gun_assets: Res<super::inventory::weapon_manager::GunAssets>,
) {
    for (entity, gun_id, player_stats, roll, reload) in &mut players {
        if let Ok((gunstats, gunangle, _)) = guns.get_mut(gun_id.0) {
            (gunstats.reload)(
                &time,
                &mut commands,
                gunangle,
                gunstats,
                player_stats,
                reload,
                roll,
                entity,
            );
        }
    }
}

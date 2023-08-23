pub mod revolver;
pub mod sniper;

use std::time::Duration;

use bevy::prelude::*;

use crate::rendering::utils::{set_anchor, Angle, Zindex};

use super::{
    inventory::weapon_manager::GunAssets,
    weapon::{flamethrower_stats, GunStats},
};

#[derive(Bundle)]
pub struct GunBundle {
    pub name: Name,
    pub stats: GunStats,
    pub sprite: SpriteBundle,
    pub angle: Angle,
    pub zindex: Zindex,
}

impl GunBundle {
    pub fn default(guns: &Res<GunAssets>) -> Self {
        let mut stats = flamethrower_stats();
        stats.timer.set_elapsed(Duration::new(1, 0));
        GunBundle {
            name: Name::new("Gun"),
            sprite: SpriteBundle {
                texture: guns.sniper.clone(),
                transform: Transform::from_translation(Vec3::new(8., 0., 50.)),
                sprite: Sprite {
                    anchor: set_anchor(stats.handle_position, stats.size),
                    ..default()
                },
                ..default()
            },
            stats,
            angle: Angle(0.),
            zindex: Zindex(50.),
        }
    }
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

use super::pickup::GunPickupBundle;
use crate::player::guns::auto::create_auto_pickup;
use crate::player::guns::{revolver::create_revolver_pickup, sniper::create_sniper_pickup, charge::create_charged_pickup, flamethrower::create_flamethrower_pickup, semi_auto::create_semi_auto_pickup, shotgun::create_shotgun_pickup};
use crate::rendering::outline::Outline;

#[derive(AssetCollection, Resource)]
pub struct GunAssets {
    #[asset(path = "guns/bullet.png")]
    pub marine_bullet: Handle<Image>,
    #[asset(path = "guns/marine_gun.png")]
    pub marine: Handle<Image>,
    #[asset(path = "guns/revolver.png")]
    pub revolver: Handle<Image>,
    #[asset(path = "guns/shotgun.png")]
    pub shotgun: Handle<Image>,
    #[asset(path = "guns/sniper.png")]
    pub sniper: Handle<Image>,
    #[asset(path = "guns/kalachnikov.png")]
    pub kalachnikov: Handle<Image>,
    #[asset(path = "guns/full_auto.png")]
    pub automatic: Handle<Image>,
    #[asset(path = "guns/flamethrower.png")]
    pub flame_thrower: Handle<Image>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Reflect)]
pub enum Guns {
    Revolver,
    Shotgun,
    Sniper,
    SemiAuto,
    Auto,
    Flamethrower,
    Charge
}

impl Guns {
    pub fn to_pickup(
        self,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprites: &Res<GunAssets>,
    ) -> GunPickupBundle {
        match self {
            Guns::Revolver => create_revolver_pickup(pos, meshes, materials, sprites),
            Guns::Shotgun => create_shotgun_pickup(pos, meshes, materials, sprites),
            Guns::Sniper => create_sniper_pickup(pos, meshes, materials, sprites),
            Guns::SemiAuto => create_semi_auto_pickup(pos, meshes, materials, sprites),
            Guns::Auto => create_auto_pickup(pos, meshes, materials, sprites),
            Guns::Flamethrower => create_flamethrower_pickup(pos, meshes, materials, sprites),
            Guns::Charge => create_charged_pickup(pos, meshes, materials, sprites),
        }
    }
}

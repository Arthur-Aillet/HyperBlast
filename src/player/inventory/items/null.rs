use crate::player::inventory::item_manager::ItemTrait;
use bevy::prelude::*;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct Null {}

impl ItemTrait for Null {
}

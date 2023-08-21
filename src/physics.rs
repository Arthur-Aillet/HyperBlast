use bevy::prelude::*;

pub use bevy_rapier2d::prelude::*;

macro_rules! collision_get {
    ($query:expr, $entity1:expr, $entity2:expr) => {
        if let Ok(found) = $query.get_mut(*$entity1) {
            Some(found)
        } else if let Ok(found) = $query.get_mut(*$entity2) {
            Some(found)
        } else {
            None
        }
    };
}

pub(crate) use collision_get;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default().disabled());
    }
}

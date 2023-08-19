use bevy::prelude::*;
use bevy::render::texture::Image;
use bevy::transform::TransformSystem;
use bevy_rapier2d::rapier;

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
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
            .add_plugins(RapierDebugRenderPlugin::default().disabled())
            .configure_sets(
                PostUpdate,
                (
                    PhysicsSet::SyncBackend,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Writeback,
                )
                    .chain()
                    .before(TransformSystem::TransformPropagate)
            )
            .add_systems(
                PostUpdate,
                (
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
                        .in_set(PhysicsSet::SyncBackend),
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
                        .in_set(PhysicsSet::StepSimulation),
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
                        .in_set(PhysicsSet::Writeback),
                ),
            );
    }
}

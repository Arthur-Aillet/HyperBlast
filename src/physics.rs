use bevy::prelude::*;
use bevy::render::texture::Image;
use bevy_rapier2d::rapier;

pub use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TesselatedCollider>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default().disabled())
            .add_systems(PreUpdate, generate_colliders);
    }
}

pub trait CollisionEventExt {
    fn entities(&self) -> (Entity, Entity);
    fn is_started(&self) -> bool;
    fn is_stopped(&self) -> bool;
}

impl CollisionEventExt for CollisionEvent {
    /// Get the entities involved in the collision
    fn entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(ent1, ent2, _) | CollisionEvent::Stopped(ent1, ent2, _) => {
                (*ent1, *ent2)
            }
        }
    }

    /// Whether or not the contact has just started
    fn is_started(&self) -> bool {
        match self {
            CollisionEvent::Started(_, _, _) => true,
            CollisionEvent::Stopped(_, _, _) => false,
        }
    }

    fn is_stopped(&self) -> bool {
        !self.is_started()
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct TesselatedColliderHasLoaded;

use image::DynamicImage;
use image::ImageBuffer;

#[derive(Default, Component, Reflect)]
pub struct TesselatedCollider {
    pub texture: Handle<Image>,
    pub offset: Vec2, // TODO: SHOULD'NT EXIST, WHY ARE THEY DISPLACED!!
}

fn create_compound_collider_from_image(image: DynamicImage, offset: Vec2) -> Collider {
    let mut shapes: Vec<(Vec2, Rot, Collider)> = Vec::new();

    for (count, pixel) in image.as_rgba8().unwrap().pixels().enumerate() {
        if pixel.0[3] != 0 {
            let x = count % image.width() as usize;
            let y = count / image.width() as usize;
            shapes.push((
                Vec2::new(x as f32 + 0.5 + offset.x, y as f32 * -1. - 0.5 + offset.y),
                0. as rapier::math::Real,
                Collider::cuboid(0.5, 0.5),
            ));
        }
    }
    Collider::compound(shapes)
}

fn generate_colliders(
    mut commands: Commands,
    pending_colliders: Query<(Entity, &TesselatedCollider), Without<TesselatedColliderHasLoaded>>,
    image_assets: Res<Assets<Image>>,
) {
    // TODO: Hot reload collision shape changes
    for (ent, tesselated_collider) in pending_colliders.iter() {
        // Get the collider image
        let image = if let Some(image) = image_assets.get(&tesselated_collider.texture) {
            image
        } else {
            continue;
        };
        let shape = create_compound_collider_from_image(
            DynamicImage::ImageRgba8(
                ImageBuffer::from_vec(
                    image.texture_descriptor.size.width,
                    image.texture_descriptor.size.height,
                    image.data.clone(),
                )
                .unwrap(),
            ),
            tesselated_collider.offset,
        );

        commands
            .entity(ent)
            .insert(shape)
            .insert(TesselatedColliderHasLoaded)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(ColliderMassProperties::Density(0.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(LockedAxes::TRANSLATION_LOCKED);
    }
}

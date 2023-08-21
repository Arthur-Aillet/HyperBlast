use bevy::{prelude::*, math::Vec3Swizzles};

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Zindex(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Offset(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Size(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Reflect, Default, PartialEq)]
pub enum Flip {
    #[default]
    False,
    XAxis,
    YAxis,
    XYAxis,
}

pub fn set_angle(mut query: Query<(&mut Transform, &Angle)>) {
    for (mut transform, Angle(val)) in &mut query {
        transform.rotation = Quat::from_rotation_z(*val);
    }
}

pub fn set_zindex(mut query: Query<(&mut Transform, &Zindex)>) {
    for (mut transform, Zindex(val)) in &mut query {
        transform.translation.z = *val;
    }
}

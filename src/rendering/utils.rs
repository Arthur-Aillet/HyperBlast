use bevy::{prelude::*, math::Vec3Swizzles};

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32); // Not supported yet

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

pub fn set_zindex(mut query: Query<(&mut Transform, &Zindex)>) {
    for (mut transform, Zindex(val)) in &mut query {
        transform.translation.z = *val;
    }
}

pub fn reset_positions(
    mut query: Query<(
        &mut Transform,
        &Position,
    )>,
) {
    for (mut transform, pos) in &mut query {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
    }
}

pub fn floor_transform_position(
    mut query: Query<(
        &mut Transform,
        &mut Position,
    )>,
) {
    for (mut transform, mut pos) in &mut query {
        let save = transform.translation.xy();

        transform.translation.x = transform.translation.x.floor();
        transform.translation.y = transform.translation.y.floor();
        pos.0 = save;
    }
}

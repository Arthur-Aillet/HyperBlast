use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Zindex(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct AutoZindex;

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

pub fn set_auto_zindex(mut query: Query<(&mut Transform, &AutoZindex)>) {
    for (mut transform, AutoZindex) in &mut query {
        transform.translation.z = 5. / 1. * 2_f32.powf(-0.001 * transform.translation.y);
    }
}

pub fn set_anchor(offset: Vec2, size: Vec2) -> bevy::sprite::Anchor {
    bevy::sprite::Anchor::Custom(((offset * 2. - size) / size) * 0.5)
}

use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn update_transforms(
    mut query: Query<(
        &mut Transform,
        &Position,
    )>)
{
    for (mut transfrom, position) in &mut query {
        transfrom.translation.x = position.x.floor();
        transfrom.translation.y = position.y.floor();
    }
}

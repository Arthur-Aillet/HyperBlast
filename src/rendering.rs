use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Position(pub Vec2);

pub fn update_transforms(
    mut query: Query<(
        &mut Transform,
        &Position,
    )>)
{
    for (mut transfrom, Position(position)) in &mut query {
        transfrom.translation.x = position.floor().x;
        transfrom.translation.y = position.floor().y;
    }
}

use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Position(pub Vec2);

#[derive(Component, Default, Reflect)]
pub struct Offset(pub Vec2);

pub fn update_transforms(
    mut query: Query<(
        &mut Transform,
        &Position,
        Option<&Offset>
    )>)
{
    for (mut transfrom, Position(position), offset) in &mut query {
        let offset = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();

        transfrom.translation.x = position.floor().x - offset.x;
        transfrom.translation.y = position.floor().y + offset.y;
    }
}

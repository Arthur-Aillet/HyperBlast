use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Angle(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Position(pub Vec2);


#[derive(Component, Default, Reflect)]
pub struct ZIndex(pub f32);

#[derive(Component, Default, Reflect)]
pub struct Offset(pub Vec2);

pub fn update_transforms(mut query: Query<(&mut Transform, &Position, Option<&Offset>, Option<&Angle>, Option<&ZIndex>)>) {
    for (mut transfrom, Position(position), offset, angle, zindex) in &mut query {
        let offset = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();

        transfrom.translation.x = position.floor().x - offset.x;
        transfrom.translation.y = position.floor().y + offset.y;
        if let Some(z) = zindex {
            transfrom.translation.z = z.0;
        }
        transfrom.rotation = Quat::from_rotation_z(angle.unwrap_or(&Angle(0.)).0);
    }
}

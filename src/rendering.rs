use bevy::prelude::*;

use crate::animations::AnimationFlip;

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct ZIndex(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Offset(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Size(pub Vec2);

#[allow(clippy::type_complexity)]
pub fn update_transforms(
    mut query: Query<(
        &mut Transform,
        Option<&mut Sprite>,
        &Position,
        Option<&Offset>,
        Option<&Angle>,
        Option<&ZIndex>,
        Option<&Size>,
        Option<&AnimationFlip>,
    )>,
) {
    for (
        mut transfrom,
        sprite_maybe,
        Position(position),
        offset,
        angle,
        zindex,
        size_maybe,
        flip_maybe,
    ) in &mut query
    {
        let mut offset_transform = Vec2::ZERO;

        if let Some(z) = zindex {
            transfrom.translation.z = z.0;
        }
        if let Some(mut sprite) = sprite_maybe {
            if let Some(flip) = flip_maybe {
                sprite.flip_x = *flip == AnimationFlip::XAxis || *flip == AnimationFlip::XYAxis;
                sprite.flip_y = *flip == AnimationFlip::YAxis || *flip == AnimationFlip::XYAxis;
            }
            if let Some(size) = size_maybe {
                let size = size.0.floor();
                let mut offset = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();

                if sprite.flip_x {
                    offset.x = size.x - offset.x;
                }
                if sprite.flip_y {
                    offset.y = size.y - offset.y;
                }

                sprite.anchor = bevy::sprite::Anchor::Custom(((offset * 2. - size) / size) * 0.5);
            }
        } else {
            offset_transform = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();
        }
        transfrom.translation.x = position.floor().x - offset_transform.x;
        transfrom.translation.y = position.floor().y + offset_transform.y;
        transfrom.rotation = Quat::from_rotation_z(angle.unwrap_or(&Angle(0.)).0);
    }
}

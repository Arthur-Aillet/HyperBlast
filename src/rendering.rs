use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32); // Not supported yet

#[derive(Component, Default, Reflect, Clone)]
pub struct Zindex(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Offset(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Size(pub Vec2);


#[derive(Component, Reflect, Default, PartialEq)]
pub enum Flip {
    #[default]
    False,
    XAxis,
    YAxis,
    XYAxis,
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .register_type::<Position>()
            .register_type::<Angle>()
            .register_type::<Zindex>()
            .register_type::<Offset>()
            .register_type::<Size>()
            .register_type::<Flip>()
            .add_systems(Last, update_transforms);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_transforms(
    mut query: Query<(
        &mut Transform,
        &Position,
        Option<&mut Sprite>,
        Option<&Offset>,
        Option<&Angle>,
        Option<&Zindex>,
        Option<&Size>,
        Option<&Flip>,
    )>,
) {
    for (
        mut transfrom,
        Position(position),
        sprite_maybe,
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
                sprite.flip_x = *flip == Flip::XAxis || *flip == Flip::XYAxis;
                sprite.flip_y = *flip == Flip::YAxis || *flip == Flip::XYAxis;
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
            } else {
                offset_transform = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();
            }
        } else {
            offset_transform = offset.unwrap_or(&Offset(Vec2::ZERO)).0.floor();
        }
        transfrom.translation.x = position.floor().x - offset_transform.x;
        transfrom.translation.y = position.floor().y + offset_transform.y;
        transfrom.rotation = Quat::from_rotation_z(angle.unwrap_or(&Angle(0.)).0);
    }
}

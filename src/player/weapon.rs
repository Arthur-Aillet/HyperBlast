use bevy::prelude::*;

#[derive(Bundle)]
pub struct Gun {
    pub sprite: SpriteBundle,
}

impl Gun {
    pub fn setup(
        asset_server: &Res<AssetServer>,
    ) -> Self {
    Gun {
        sprite: SpriteBundle {
            texture: asset_server.load("marine_gun.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
    }
    }
}

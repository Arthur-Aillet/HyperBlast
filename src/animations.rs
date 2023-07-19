use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


pub fn animate_sprites(
    time: Res<Time>,
    mut timer_query: Query<&mut AnimationTimer>,
    mut query: Query<(
        &AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    let mut timer = timer_query.get_single_mut().expect("Lacks global timer");
    for (indices, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

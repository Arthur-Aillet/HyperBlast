use std::fmt::Debug;

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Reflect, Default, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Reflect, Default)]
pub struct AnimationState(String);

impl AnimationState {
    pub fn new<T: Debug>(string: T) -> Self {
        AnimationState {0: format!("{string:?}")}
    }
}

#[derive(Component, Reflect, Default)]
pub struct AnimationStateMachine(HashMap<String, (Handle<TextureAtlas>, AnimationIndices)>);

impl AnimationStateMachine {
    pub fn new() -> Self {
        AnimationStateMachine {0: HashMap::new()}
    }

    pub fn insert<T: Debug>(&mut self, key: T, value: (Handle<TextureAtlas>, AnimationIndices)) {
        self.0.insert(format!("{key:?}"), value);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


pub fn animate_sprites(
    time: Res<Time>,
    mut timer_query: Query<&mut AnimationTimer>,
    mut query: Query<(
        &AnimationState,
        &mut Handle<TextureAtlas>,
        &mut AnimationStateMachine,
        &mut TextureAtlasSprite,
    )>,
) {
    let mut timer = timer_query.get_single_mut().expect("Lacks global timer");
    for (state, mut current_handle, mut machine, mut current_sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match machine.0.get(&state.0) {
                Some((sprite, indices)) => {
                    *current_handle = sprite.clone();
                    current_sprite.index = if current_sprite.index == indices.last {
                        indices.first
                    } else {
                        current_sprite.index + 1
                    }
                }
                None => {}
            }
        }
    }
}

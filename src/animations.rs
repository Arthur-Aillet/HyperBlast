use std::fmt::Debug;

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Reflect, Default, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Reflect, Default)]
pub struct AnimationState {
    id: String,
}

impl AnimationState {
    pub fn new<T: Debug>(string: &T) -> Self {
        AnimationState {
            id: format!("{string:?}"),
        }
    }
}

#[derive(Reflect, Default, PartialEq)]
pub enum AnimationFlip {
    #[default]
    False,
    XAxis,
    YAxis,
    XYAxis,
}

#[derive(Component, Reflect, Default)]
pub struct AnimationStateMachine {
    map: HashMap<String, (Handle<TextureAtlas>, AnimationIndices, AnimationFlip)>,
    last_state: AnimationState,
}

impl AnimationStateMachine {
    pub fn new() -> Self {
        AnimationStateMachine {
            map: HashMap::new(),
            last_state: AnimationState::default(),
        }
    }

    pub fn insert<T: Debug>(
        &mut self,
        key: T,
        value: (Handle<TextureAtlas>, AnimationIndices, AnimationFlip),
    ) {
        self.map.insert(format!("{key:?}"), value);
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
            if let Some((sprite, indices, flip)) = machine.map.get(&state.id) {
                current_sprite.flip_x = *flip == AnimationFlip::XAxis || *flip == AnimationFlip::XYAxis;
                current_sprite.flip_y = *flip == AnimationFlip::YAxis || *flip == AnimationFlip::XYAxis;
                if state.id != machine.last_state.id {
                    current_sprite.index = indices.first;
                }
                *current_handle = sprite.clone();
                current_sprite.index = if current_sprite.index == indices.last {
                    indices.first
                } else {
                    current_sprite.index + 1
                };
                machine.last_state = AnimationState {
                    id: state.id.clone(),
                };
            }
        }
    }
}

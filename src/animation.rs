use std::fmt::Debug;

use bevy::{prelude::*, utils::HashMap};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationIndices>()
            .register_type::<AnimationState>()
            .register_type::<AnimationFlip>()
            .register_type::<AnimationTimer>()
            .register_type::<AnimationStateMachine>()
            .add_systems(Startup, setup_animation_plugin)
            .add_systems(PostUpdate, animate_sprites);
    }
}

#[derive(Component, Reflect, Default, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect, Default, PartialEq)]
pub enum AnimationFlip {
    #[default]
    False,
    XAxis,
    YAxis,
    XYAxis,
}

#[derive(Component, Reflect)]
pub struct AnimationStateMachine {
    map: HashMap<String, (Handle<TextureAtlas>, AnimationIndices, AnimationFlip)>,
    last_state: AnimationState,
    next: bool,
    manual: bool,
}

impl AnimationStateMachine {
    pub fn new() -> Self {
        AnimationStateMachine {
            map: HashMap::new(),
            last_state: AnimationState::new(&""),
            next: false,
            manual: false,
        }
    }

    pub fn new_filled<T: Debug, const COUNT: usize>(
        content: [(T, Handle<TextureAtlas>, AnimationIndices, AnimationFlip); COUNT],
    ) -> Self {
        let mut new = Self::new();

        for (key, a, b, c) in content {
            new.map.insert(format!("{key:?}"), (a, b, c));
        }
        new
    }

    #[allow(dead_code)]
    pub fn insert<T: Debug>(
        &mut self,
        key: T,
        value: (Handle<TextureAtlas>, AnimationIndices, AnimationFlip),
    ) {
        self.map.insert(format!("{key:?}"), value);
    }

    pub fn set_manual(&mut self, state: bool) {
        self.manual = state;
    }

    pub fn next(&mut self) {
        self.next = true;
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AnimationTimer(pub Timer);

pub fn setup_animation_plugin(mut commands: Commands) {
    commands.spawn((
        bevy::core::Name::new("Global Animation Timer"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

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
    timer.tick(time.delta());
    for (state, mut current_handle, mut machine, mut current_sprite) in &mut query {
        if timer.just_finished() {
            if machine.manual && !machine.next {
                continue;
            }
            if machine.manual && machine.next {
                machine.next = false;
            }
            if let Some((sprite, indices, flip)) = machine.map.get(&state.id) {
                current_sprite.flip_x =
                    *flip == AnimationFlip::XAxis || *flip == AnimationFlip::XYAxis;
                current_sprite.flip_y =
                    *flip == AnimationFlip::YAxis || *flip == AnimationFlip::XYAxis;
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

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::prelude::*;

pub fn switch_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
    if input.just_pressed(KeyCode::L) {
        let mut level_set = level_sets.single_mut();
        let current_levels: String = level_set
            .iids
            .iter().cloned()
            .collect();
        let level_to_toggle: Vec<&&str> = super::LEVEL_IIDS
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect();
        let new_level = if current_levels != *(level_to_toggle[0]) {
            *(level_to_toggle[0])
        } else {
            *(level_to_toggle[1])
        };

        level_set.iids.clear();
        level_set.iids.insert(new_level.to_string());
    }
}

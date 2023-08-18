use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use rand::prelude::*;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .register_ldtk_entity::<Map>("Map")
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(ClearColor(Color::Rgba { red: 20./255., green: 20./255., blue: 18./255., alpha: 1. }))
            .add_systems(Startup, setup_map)
            .add_systems(Update, switch_levels);
    }
}

const LEVEL_IIDS: [&str; 2] = [
    "65e48640-3b70-11ee-a7e1-116d8f689398",
    "855efd30-3b70-11ee-acdb-0b20980d1e54",
];

#[derive(Bundle, LdtkEntity)]
pub struct Map {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut level_set = LevelSet::from_iid(LEVEL_IIDS[0]);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/map.ldtk"),
        transform: Transform::from_translation(Vec3::new(-240., -240., 0.)),
        level_set,
        ..default()
    });
}

fn switch_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
    if input.just_pressed(KeyCode::L) {
        let mut level_set = level_sets.single_mut();
        let current_levels: String = level_set.iids.iter().map(|fst| return fst.clone()).collect();
        let level_to_toggle: Vec<&&str> = LEVEL_IIDS.choose_multiple(&mut rand::thread_rng(), 2).collect();
        let new_level = if current_levels != *(level_to_toggle[0]) {
            *(level_to_toggle[0])
        } else {*(level_to_toggle[1])};

        level_set.iids.clear();
        level_set.iids.insert(new_level.to_string());
    }
}

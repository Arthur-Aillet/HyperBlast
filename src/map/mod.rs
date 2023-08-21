pub mod colliders;
pub mod switch;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use self::colliders::WallBundle;

#[derive(Bundle, LdtkEntity)]
pub struct Map {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

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
            .register_ldtk_int_cell::<WallBundle>(1)
            //.add_systems(Startup, setup_map)
            .add_systems(Update, colliders::spawn_wall_collision)
            .add_systems(Update, switch::switch_levels);
    }
}

const LEVEL_IIDS: [&str; 2] = [
    "65e48640-3b70-11ee-a7e1-116d8f689398",
    "855efd30-3b70-11ee-acdb-0b20980d1e54",
];

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_set = LevelSet::from_iid(LEVEL_IIDS[0]);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/map.ldtk"),
        transform: Transform::from_translation(Vec3::new(-240., -240., 0.)),
        level_set,
        ..default()
    });
}

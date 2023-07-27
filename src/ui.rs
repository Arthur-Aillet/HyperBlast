use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::player::stats::PlayerStats;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<UiAssets>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, spawn_health_bars)
            .add_systems(PostUpdate, manage_health_bars);
    }
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct HasHealthBar {
    pub id: Entity,
}

#[derive(Component)]
pub struct HealthBar {
    pub player_id: Entity,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/healthbar_bg.png")]
    pub health_bar_bg: Handle<Image>,
    #[asset(path = "ui/healthbar_fg.png")]
    pub health_bar_fg: Handle<Image>,
}

fn spawn_health_bars(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    root_query: Query<(Entity, With<UiRoot>)>,
    players_query: Query<(Entity,(Without<HasHealthBar>, With<PlayerStats>))>,
) {
    let root = root_query.single().0;
    for (player, _) in &players_query {
        let health_bar = commands.spawn((
                HealthBar {player_id: player},
                NodeBundle {
                    style: Style {
                        grid_row: GridPlacement::start(1),
                        width: Val::Px(107. * 3.),
                        height: Val::Px(10. * 3.),
                        margin: UiRect::all(Val::VMin(2.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(ui_assets.health_bar_bg.clone()),
        )).with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    grid_row: GridPlacement::start(1),
                    width: Val::Px(107. * 3.),
                    height: Val::Px(10. * 3.),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            UiImage::new(ui_assets.health_bar_fg.clone()),));
        }).id();
        commands.get_entity(player).unwrap().insert(HasHealthBar{id: health_bar});
        commands.get_entity(root).unwrap().add_child(health_bar);    }
}

fn manage_health_bars(
    mut commands: Commands,
    players_query: Query<(Entity, Option<&HasHealthBar>, &PlayerStats, Without<UiRoot>)>,
    health_bars_query: Query<(Entity, &HealthBar)>,
) {
    for (id, stats) in &health_bars_query {
        match players_query.get(stats.player_id) {
            Ok(_) => {},
            Err(_) => {
                commands.get_entity(id).unwrap().despawn_recursive();
            },
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("UI Root"))
        .insert(UiRoot);
}

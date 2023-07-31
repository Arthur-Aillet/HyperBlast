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
pub struct HealthBarFg;

#[derive(Component)]
pub struct HasHealthBar {
    pub id: Entity,
}

#[derive(Component)]
pub struct HealthBar {
    pub player_id: Entity,
    pub health_bar_fg: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/healthbar_bg.png")]
    pub health_bar_bg: Handle<Image>,
}

fn spawn_health_bars(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    root_query: Query<(Entity, With<UiRoot>)>,
    players_query: Query<(Entity,(Without<HasHealthBar>, With<PlayerStats>))>,
) {

    let root = root_query.single().0;
    for (player, _) in &players_query {
        let fg_handle = asset_server.load("ui/healthbar_fg.png");
        let fg_atlas = TextureAtlas::from_grid(
            fg_handle,
            Vec2::new(100.0, 10.0),
            1,
            1,
            None,
            None,
        );
        let fg_atlas_handle = texture_atlases.add(fg_atlas);

        let health_bar = commands.spawn((

                HealthBar {player_id: player, health_bar_fg: fg_atlas_handle.clone()},
                NodeBundle {
                    style: Style {
                        width: Val::Px(108. * 3.),
                        height: Val::Px(10. * 3.),
                        margin: UiRect::all(Val::VMin(2.)),
                        padding: UiRect::left(Val::Px(8. * 3.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(ui_assets.health_bar_bg.clone()),
        )).with_children(|parent| {
            parent.spawn((AtlasImageBundle {
                style: Style {
                    width: Val::Px(100. * 3.),
                    height: Val::Px(10. * 3.),
                    ..default()
                },
                texture_atlas: fg_atlas_handle.clone(),
                texture_atlas_image: UiTextureAtlasImage::default(),
                ..default()
            },
            HealthBarFg,
            ));
        }).id();
        commands.get_entity(player).unwrap().insert(HasHealthBar{id: health_bar});
        commands.get_entity(root).unwrap().add_child(health_bar);
    }
}

fn manage_health_bars(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    players_query: Query<(&PlayerStats, (With<HasHealthBar>, Without<UiRoot>))>,
    health_bars_query: Query<(&Children, Entity, &HealthBar)>,
    mut health_bars_fg_query: Query<(&mut Style, With<HealthBarFg>)>,
) {
    for (children, id, healthbar_struct) in &health_bars_query {
        for childrens_entity in children {
            if let Ok((mut fg_style, _)) = health_bars_fg_query.get_mut(*childrens_entity) {
                match players_query.get(healthbar_struct.player_id) {
                    Ok((stats, _)) => {
                        let health_bar = atlases.get_mut(&healthbar_struct.health_bar_fg);
                        let percentage = stats.current_health / stats.max_health * 100.;

                        health_bar.unwrap().textures[0] = Rect::new(100. - percentage, 0., 100., 10.);
                        fg_style.width = Val::Px(percentage * 3.);
                    },
                    Err(_) => commands.get_entity(id).unwrap().despawn_recursive(),
                }
            }
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

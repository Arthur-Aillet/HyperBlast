use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::player::{stats::PlayerStats, weapon::{GunEntity, GunStats}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<UiAssets>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, spawn_player_ui)
            .add_systems(PostUpdate, (manage_health_bars, manage_ammo_count));
    }
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct HealthBarFg;


#[derive(Component)]
pub struct HealthBar {
    pub player_id: Entity,
    pub health_bar_fg: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct AmmoCounter {
    pub player_id: Entity,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/healthbar_bg.png")]
    pub health_bar_bg: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerUiAccess {
    pub health_bar_id: Entity,
    pub ammo_counter_id: Entity,
}

fn spawn_player_ui(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
<<<<<<< HEAD
    ui_root: Query<(Entity, With<UiRoot>)>,
    players_query: Query<(Entity, (Without<PlayerUiAccess>, With<PlayerStats>))>,
) {
    for (id, _) in &players_query {
        // Create healthbar:
=======
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    root_query: Query<(Entity, With<UiRoot>)>,
    players_query: Query<(Entity, (Without<HasHealthBar>, With<PlayerStats>))>,
) {
    let root = root_query.single().0;
    for (player, _) in &players_query {
>>>>>>> 4bd7630eeeb0ce87252513483ba838522594af48
        let fg_handle = asset_server.load("ui/healthbar_fg.png");
        let fg_atlas = TextureAtlas::from_grid(fg_handle, Vec2::new(100.0, 10.0), 1, 1, None, None);
        let fg_atlas_handle = texture_atlases.add(fg_atlas);
<<<<<<< HEAD
        let hb_id = commands.spawn((
            HealthBar {player_id: id, health_bar_fg: fg_atlas_handle.clone()},
            NodeBundle {
                style: Style {
                    width: Val::Px(108. * 3.),
                    height: Val::Px(10. * 3.),
                    padding: UiRect::left(Val::Px(8. * 3.)),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            UiImage::new(ui_assets.health_bar_bg.clone()),
        ))
            .with_children(|healthbar_bg| {
                healthbar_bg.spawn((AtlasImageBundle {
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
        // Create Ammo count:
        let count_id = commands.spawn((
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Extended_font.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                }),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Extended_font.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                }),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Extended_font.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                }),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Extended_font.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                    ..default()
                }),
            ]), AmmoCounter {player_id: id},
        )).id();
        let player_ui_id = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect::all(Val::VMin(2.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("PlayerUI"))
            .add_child(hb_id)
            .add_child(count_id).id();
        commands.entity(ui_root.single().0)
            .add_child(player_ui_id);
        commands.entity(id).insert(PlayerUiAccess { health_bar_id: hb_id, ammo_counter_id: count_id });
=======

        let health_bar = commands
            .spawn((
                HealthBar {
                    player_id: player,
                    health_bar_fg: fg_atlas_handle.clone(),
                },
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
            ))
            .with_children(|parent| {
                parent.spawn((
                    AtlasImageBundle {
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
            })
            .id();
        commands
            .get_entity(player)
            .unwrap()
            .insert(HasHealthBar { id: health_bar });
        commands.get_entity(root).unwrap().add_child(health_bar);
>>>>>>> 4bd7630eeeb0ce87252513483ba838522594af48
    }
}


fn manage_health_bars(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    players_query: Query<(&PlayerStats, &PlayerUiAccess)>,
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

                        health_bar.unwrap().textures[0] =
                            Rect::new(100. - percentage, 0., 100., 10.);
                        fg_style.width = Val::Px(percentage * 3.);
                    }
                    Err(_) => commands.get_entity(id).unwrap().despawn_recursive(),
                }
            }
        }
    }
}

fn manage_ammo_count(
    players: Query<(&PlayerStats, &GunEntity)>,
    guns: Query<&GunStats, Without<PlayerStats>>,
    mut texts: Query<( &mut Text, &mut AmmoCounter)>
) {
    for (mut text, count) in &mut texts {
        if let Ok((_, gunentity)) =players.get(count.player_id) {
            if let Ok(gunstats) = guns.get(gunentity.0) {
                text.sections[0].value = format!("ammo: ");
                text.sections[1].value = format!("{}", gunstats.mag_ammo);
                text.sections[1].style.color =
                    if (gunstats.mag_ammo as f32) / (gunstats.mag_size as f32) > 1./3. { Color::WHITE
                } else if (gunstats.mag_ammo as f32) / (gunstats.mag_size as f32) > 1./5. { Color::YELLOW}
                else {Color::RED};
                text.sections[2].value = format!("/{}\n", gunstats.mag_size);
                if !gunstats.infinite {
                    text.sections[3].value = format!("{}", gunstats.ammo);
                    text.sections[3].style.color =
                        if (gunstats.ammo as f32) / (gunstats.max_ammo as f32) > 1./3. { Color::WHITE
                    } else if (gunstats.ammo as f32) / (gunstats.max_ammo as f32) > 1./5. { Color::YELLOW}
                    else {Color::RED};
                } else {
                    text.sections[3].value = format!("\u{ec}");
                    text.sections[3].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
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

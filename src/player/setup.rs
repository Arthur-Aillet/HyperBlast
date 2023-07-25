use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{prelude::ActionStateDriver, InputManagerBundle};
use mouse::Mouse;

use crate::{
    animation::{AnimationFlip, AnimationIndices, AnimationState, AnimationStateMachine},
    mouse,
    physics::TesselatedCollider,
    rendering::{Offset, Position, Zindex},
};

use input::PlayerActions;

use super::{
    input::{self, IsController, PlayerState},
    stats::PlayerStats,
    weapon::{GunBundle, GunEntity},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub player: PlayerStats,
    pub player_action: InputManagerBundle<PlayerActions>,
    pub player_position: Position,
    pub zindex: Zindex,
    pub player_offset: Offset,
    pub current_gun: GunEntity,
    pub collider: TesselatedCollider,
}

impl PlayerBundle {
    pub fn setup(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        window: &Query<Entity, With<PrimaryWindow>>,
        controller: bool,
    ) {
        let idle_texture_handle = asset_server.load("idle.png");
        let run_texture_handle = asset_server.load("run.png");
        let idle_atlas = TextureAtlas::from_grid(
            idle_texture_handle.clone(),
            Vec2::new(17.0, 25.0),
            4,
            1,
            Some(Vec2 { x: 2., y: 2. }),
            Some(Vec2 { x: 15., y: 15. }),
        );
        let back_atlas = TextureAtlas::from_grid(
            run_texture_handle.clone(),
            Vec2::new(17.0, 25.0),
            6,
            1,
            Some(Vec2 { x: 2., y: 2. }),
            Some(Vec2 { x: 15., y: 69. }),
        );
        let front_atlas = TextureAtlas::from_grid(
            run_texture_handle.clone(),
            Vec2::new(17.0, 25.0),
            6,
            1,
            Some(Vec2 { x: 2., y: 2. }),
            Some(Vec2 { x: 15., y: 15. }),
        );
        let side_front_atlas = TextureAtlas::from_grid(
            run_texture_handle.clone(),
            Vec2::new(17.0, 25.0),
            6,
            1,
            Some(Vec2 { x: 2., y: 2. }),
            Some(Vec2 { x: 15., y: 42. }),
        );
        let side_back_atlas = TextureAtlas::from_grid(
            run_texture_handle,
            Vec2::new(17.0, 25.0),
            6,
            1,
            Some(Vec2 { x: 2., y: 2. }),
            Some(Vec2 {
                x: 15.,
                y: 69. + 27.,
            }),
        );

        let idle_handle = texture_atlases.add(idle_atlas);
        let side_front_handle = texture_atlases.add(side_front_atlas);
        let side_back_handle = texture_atlases.add(side_back_atlas);
        let front_handle = texture_atlases.add(front_atlas);
        let back_handle = texture_atlases.add(back_atlas);

        let mut state_machine = AnimationStateMachine::new();

        state_machine.insert(
            PlayerState::Idle,
            (
                idle_handle.clone(),
                AnimationIndices { first: 0, last: 3 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::LeftFront,
            (
                side_front_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
        );
        state_machine.insert(
            PlayerState::RightFront,
            (
                side_front_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::LeftBack,
            (
                side_back_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
        );
        state_machine.insert(
            PlayerState::RightBack,
            (
                side_back_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::Front,
            (
                front_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::Back,
            (
                back_handle.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );

        let gun_id = commands.spawn(GunBundle::setup(asset_server)).id();

        let player = PlayerBundle {
            name: bevy::core::Name::new("Player"),
            state: AnimationState::new(&PlayerState::Idle),
            sprite: SpriteSheetBundle {
                texture_atlas: idle_handle,
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            state_machine,
            player: PlayerStats::default(),
            player_action: input::player_input_setup(),
            player_offset: Offset(Vec2::new(17. / 2., 25. / 2. + 8.)),
            zindex: Zindex(25.),
            player_position: Position(Vec2::ZERO),
            current_gun: GunEntity(gun_id),
            collider: TesselatedCollider {
                texture: asset_server.load("collider.png"),
                offset: Vec2::ZERO,
            },
        };
        if controller {
            commands.spawn(player).insert(IsController);
        } else {
            let player_id = commands
                .spawn(player)
                .insert(InputManagerBundle::<Mouse>::default())
                .id();

            commands.entity(window.single()).insert(ActionStateDriver {
                action: crate::mouse::Mouse::MousePosition,
                targets: player_id.into(),
            });
        }
    }
}

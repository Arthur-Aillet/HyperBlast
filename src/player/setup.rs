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
    assets::{GunAssets, PlayerAssets},
    input::{self, IsController, PlayerState},
    stats::PlayerStats,
    weapon::{GunBundle, GunEntity},
    direction::MoveDirection,
    direction::CursorPosition, inventory::inventory_manager::Inventory,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub stats: PlayerStats,
    pub action: InputManagerBundle<PlayerActions>,
    pub position: Position,
    pub zindex: Zindex,
    pub offset: Offset,
    pub current_gun: GunEntity,
    pub collider: TesselatedCollider,
    pub direction: MoveDirection,
    pub cursor: CursorPosition,
    pub inventory: Inventory,
}

impl PlayerBundle {
    pub fn setup(
        commands: &mut Commands,
        window: &Query<Entity, With<PrimaryWindow>>,
        controller: bool,
        assets: &Res<PlayerAssets>,
        guns_assets: &Res<GunAssets>,
    ) {
        let state_machine = AnimationStateMachine::new_filled([
            (
                PlayerState::Idle,
                assets.idle.clone(),
                AnimationIndices { first: 0, last: 3 },
                AnimationFlip::False,
            ),
            (
                PlayerState::LeftFront,
                assets.side_front.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
            (
                PlayerState::RightFront,
                assets.side_front.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
            (
                PlayerState::LeftBack,
                assets.side_back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
            (
                PlayerState::RightBack,
                assets.side_back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
            (
                PlayerState::Front,
                assets.front.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
            (
                PlayerState::Back,
                assets.back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
            (
                PlayerState::DodgeLeftFront,
                assets.dodge_side_front.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::XAxis,
            ),
            (
                PlayerState::DodgeRightFront,
                assets.dodge_side_front.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::False,
            ),
            (
                PlayerState::DodgeLeftBack,
                assets.dodge_side_back.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::XAxis,
            ),
            (
                PlayerState::DodgeRightBack,
                assets.dodge_side_back.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::False,
            ),
            (
                PlayerState::DodgeFront,
                assets.dodge_front.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::False,
            ),
            (
                PlayerState::DodgeBack,
                assets.dodge_back.clone(),
                AnimationIndices { first: 0, last: 8 },
                AnimationFlip::False,
            ),
        ]);

        let gun_id = commands.spawn(GunBundle::setup(guns_assets)).id();

        let player = PlayerBundle {
            name: bevy::core::Name::new("Player"),
            state: AnimationState::new(&PlayerState::Idle),
            sprite: SpriteSheetBundle {
                texture_atlas: assets.idle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            state_machine,
            stats: PlayerStats::default(),
            action: input::player_input_setup(controller),
            offset: Offset(Vec2::new(17. / 2., 25. / 2. + 8.)),
            zindex: Zindex(25.),
            position: Position(Vec2::ZERO),
            current_gun: GunEntity(gun_id),
            collider: TesselatedCollider {
                texture: assets.collider.clone(),
                offset: Vec2::ZERO,
            },
            direction: MoveDirection::default(),
            cursor: CursorPosition::default(),
            inventory: Inventory::new(),
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

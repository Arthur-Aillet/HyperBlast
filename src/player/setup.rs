use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionStateDriver, InputManagerBundle};
use mouse::Mouse;

use crate::{
    animation::{AnimationFlip, AnimationIndices, AnimationState, AnimationStateMachine},
    mouse,
    rendering::utils::{set_anchor, AutoZindex},
};

use input::PlayerActions;

use super::{
    assets::PlayerAssets,
    direction::CursorPosition,
    direction::MoveDirection,
    input::{self, IsController, PlayerState},
    inventory::{armory_manager::Armory, inventory_manager::Inventory},
    stats::PlayerStats,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub stats: PlayerStats,
    pub action: InputManagerBundle<PlayerActions>,
    pub velocity: Velocity,
    pub zindex: AutoZindex,
    pub direction: MoveDirection,
    pub cursor: CursorPosition,
    pub inventory: Inventory,
    pub armory: Armory,
    pub active: ActiveEvents,
    pub rigid_body: RigidBody,
    pub gravity: GravityScale,
    pub locked_axes: LockedAxes,
}

#[derive(Debug, Reflect, Component)]
pub struct PlayerCollider;

impl PlayerBundle {
    pub fn setup(
        commands: &mut Commands,
        window: &Query<Entity, With<PrimaryWindow>>,
        controller: bool,
        assets: &Res<PlayerAssets>,
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

        let player = PlayerBundle {
            name: bevy::core::Name::new("Player"),
            state: AnimationState::new(&PlayerState::Idle),
            sprite: SpriteSheetBundle {
                texture_atlas: assets.idle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: set_anchor(Vec2::new(17. / 2., 25. / 2. - 8.), Vec2::new(17., 25.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    controller as i32 as f32 * 60.,
                    0.,
                    0.,
                )),
                ..default()
            },
            state_machine,
            stats: PlayerStats::default(),
            action: input::player_input_setup(controller),
            zindex: AutoZindex,
            velocity: bevy_rapier2d::prelude::Velocity {
                linvel: Vec2::new(0., 0.),
                angvel: 0.0,
            },
            direction: MoveDirection::default(),
            cursor: CursorPosition::default(),
            inventory: Inventory::new(),
            armory: Armory::new(),
            active: ActiveEvents::COLLISION_EVENTS,
            rigid_body: RigidBody::Dynamic,
            gravity: GravityScale(0.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        };
        if controller {
            commands
                .spawn(player)
                .with_children(|parent| {
                    parent.spawn((
                        Collider::capsule_y(3.25, 13. / 2.),
                        Sensor,
                        TransformBundle::from(Transform::from_xyz(0., 6., 0.)),
                        ColliderDebugColor(Color::BLUE),
                        PlayerCollider,
                    ));
                    parent.spawn((
                        Collider::capsule_y(0., 13. / 2.),
                        TransformBundle::from(
                            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(1., 0.7, 1.)),
                        ),
                    ));
                })
                .insert(IsController);
        } else {
            let player_id = commands
                .spawn(player)
                .with_children(|parent| {
                    parent.spawn((
                        Collider::capsule_y(3.25, 13. / 2.),
                        Sensor,
                        TransformBundle::from(Transform::from_xyz(0., 6., 0.)),
                        ColliderDebugColor(Color::BLUE),
                        PlayerCollider,
                    ));
                    parent.spawn((
                        Collider::capsule_y(0., 13. / 2.),
                        TransformBundle::from(
                            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(1., 0.7, 1.)),
                        ),
                    ));
                })
                .insert(InputManagerBundle::<Mouse>::default())
                .id();

            commands.entity(window.single()).insert(ActionStateDriver {
                action: crate::mouse::Mouse::MousePosition,
                targets: player_id.into(),
            });
        }
    }
}

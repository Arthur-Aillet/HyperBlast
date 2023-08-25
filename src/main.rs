#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod animation;
pub mod camera;
pub mod debug;
pub mod map;
pub mod mouse;
pub mod physics;
pub mod player;
pub mod rendering;
pub mod ui;

use leafwing_input_manager::plugin::InputManagerSystem;

use bevy::{input::InputSystem, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            physics::PhysicsPlugin,
            rendering::RenderingPlugin,
            animation::AnimationPlugin,
            debug::DebugPlugin,
            ui::UiPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
            map::MapPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            mouse::update_cursor_state_from_window
                .in_set(InputManagerSystem::ManualControl)
                .before(InputManagerSystem::ReleaseOnDisable)
                .after(InputManagerSystem::Tick)
                .after(InputManagerSystem::Update)
                .after(InputSystem),
        )
        .run();
}

fn setup() {}

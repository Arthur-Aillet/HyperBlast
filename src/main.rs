#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod animation;
mod camera;
mod debug;
mod mouse;
mod outline;
mod physics;
mod player;
mod rendering;
mod ui;
mod map;

use leafwing_input_manager::plugin::InputManagerSystem;

use bevy::{input::InputSystem, prelude::*};

fn main() {
    App::new()
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(rendering::RenderingPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(map::MapPlugin)
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

fn setup() {

}

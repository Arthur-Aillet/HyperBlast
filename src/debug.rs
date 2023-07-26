use bevy::{prelude::*, reflect::TypePath};
use bevy_rapier2d::render::DebugRenderContext;
use leafwing_input_manager::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorPlugin::default())
            .add_plugins(DebugLinesPlugin::default())
            .add_plugins((FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin))
            .add_systems(Startup, setup_debug)
            .add_systems(Update, switch_debug);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum DebugAction {
    Click,
}

#[derive(Resource, PartialEq, Clone)]
pub enum DebugLevel {
    None,
    Basic,
}

pub fn draw_rectangle(
    lines: &mut ResMut<bevy_prototype_debug_lines::DebugLines>,
    center: Vec2,
    size: Vec2,
    color: Color
) {
    let point_a = Vec2::new(center.x - size.x / 2., center.y - size.y / 2.);
    let point_b = Vec2::new(center.x + size.x / 2., center.y - size.y / 2.);
    let point_c = Vec2::new(center.x - size.x / 2., center.y + size.y / 2.);
    let point_d = Vec2::new(center.x + size.x / 2., center.y + size.y / 2.);

    lines.line_colored(
        point_a.extend(0.),
        point_b.extend(0.),
        0.0,
        color,
    );
    lines.line_colored(
        point_b.extend(0.),
        point_d.extend(0.),
        0.0,
        color,
    );
    lines.line_colored(
        point_d.extend(0.),
        point_c.extend(0.),
        0.0,
        color,
    );
    lines.line_colored(
        point_c.extend(0.),
        point_a.extend(0.),
        0.0,
        color,
    );
}

fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugLevel::None);
    commands.spawn(debug_setup());
}

pub fn switch_debug(
    action: Query<&ActionState<DebugAction>>,
    mut debug_level: ResMut<DebugLevel>,
    mut rapier_debug: ResMut<DebugRenderContext>,
) {
    for action in &action {
        if action.just_pressed(DebugAction::Click) {
            *debug_level = match *debug_level {
                DebugLevel::None => DebugLevel::Basic,
                DebugLevel::Basic => DebugLevel::None,
            };
            rapier_debug.enabled = *debug_level == DebugLevel::Basic;
        }
    }
}

#[derive(Bundle)]
pub struct DebugBundle {
    pub name: Name,
    pub action: InputManagerBundle<DebugAction>,
}

pub fn debug_setup() -> DebugBundle {
    DebugBundle {
        name: Name::new("DebugManager"),
        action: InputManagerBundle::<DebugAction> {
            action_state: ActionState::<DebugAction>::default(),
            input_map: InputMap::new([(KeyCode::V, DebugAction::Click)]),
        },
    }
}

use std::time::Duration;

use bevy::{prelude::*, render::{Render, RenderSet, RenderApp}, asset::ChangeWatcher, transform};
use bevy::{
    core_pipeline::{
        clear_color::ClearColorConfig, core_3d,
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
            BindGroupLayoutEntry, BindingResource, BindingType, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
    },
};

#[derive(Component, Default, Reflect, Clone)]
pub struct Angle(pub f32); // Not supported yet

#[derive(Component, Default, Reflect, Clone)]
pub struct Zindex(pub f32);

#[derive(Component, Default, Reflect, Clone)]
pub struct Offset(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Size(pub Vec2);

#[derive(Component, Default, Reflect, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Reflect, Default, PartialEq)]
pub enum Flip {
    #[default]
    False,
    XAxis,
    YAxis,
    XYAxis,
}


pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(AssetPlugin {
                // Hot reloading the shader works correctly
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..default()
            }))
            .add_plugins(crate::outline::OutlinePlugin)
            .register_type::<Position>()
            .register_type::<Angle>()
            .register_type::<Zindex>()
            .register_type::<Offset>()
            .register_type::<Size>()
            .register_type::<Flip>()
            .add_systems(Update, set_zindex);
    }
}

fn set_zindex(mut query: Query<(&mut Transform, &Zindex)>) {
    for (mut transform, Zindex(val)) in &mut query {
        transform.translation.z = *val;
    }
}

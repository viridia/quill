use std::f32::consts::PI;

use bevy::{color::palettes, prelude::*, ui};
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderLayout};
use bevy_quill::*;
use bevy_quill_obsidian::{
    controls::{Button, IconButton, ToolIconButton, ToolPalette},
    viewport, RoundedCorners,
};

use crate::{gen::NodeOutput, graph::NodeSelected, pipeline::NodeShader3dHandle};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum PreviewMode {
    #[default]
    Square,
    Square2X2,
    Square3X3,
    Sphere,
    Cuboid,
    Tetra,
    Torus,
}

// Computed state that says whether we are in 3D preview mode vs 2d. This sets up and tears
// down the 3d scene.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct PreviewMode3d;

impl ComputedStates for PreviewMode3d {
    // Computed states can be calculated from one or many source states.
    type SourceStates = PreviewMode;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: PreviewMode) -> Option<PreviewMode3d> {
        match sources {
            PreviewMode::Cuboid | PreviewMode::Sphere | PreviewMode::Tetra | PreviewMode::Torus => {
                Some(PreviewMode3d)
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PreviewControls;

fn style_preview_controls(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .flex_wrap(ui::FlexWrap::Wrap)
        .justify_items(ui::JustifyItems::End)
        .gap(6);
}

impl ViewTemplate for PreviewControls {
    type View = impl View;

    fn create(&self, _cx: &mut bevy_quill::Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style(style_preview_controls)
            .children((
                PreviewModeButtons,
                IconButton::new("embedded://bevy_quill_obsidian/assets/icons/lock.png"),
                Button::new().children("Source..."),
                Button::new().children("Export..."),
            ))
    }
}

#[derive(Clone, PartialEq)]
pub struct PreviewModeButtons;

impl ViewTemplate for PreviewModeButtons {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let mode = *cx.use_resource::<State<PreviewMode>>().get();

        ToolPalette::new().columns(7).children((
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/square.png")
                .size(Vec2::splat(20.))
                .corners(RoundedCorners::Left)
                .selected(mode == PreviewMode::Square)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Square);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/square2x2.png")
                .size(Vec2::splat(20.))
                .corners(RoundedCorners::None)
                .selected(mode == PreviewMode::Square2X2)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Square2X2);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/square3x3.png")
                .size(Vec2::splat(20.))
                .corners(RoundedCorners::None)
                .selected(mode == PreviewMode::Square3X3)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Square3X3);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/sphere.png")
                .size(Vec2::splat(20.))
                .no_tint(true)
                .corners(RoundedCorners::None)
                .selected(mode == PreviewMode::Sphere)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Sphere);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/cuboid.png")
                .size(Vec2::splat(20.))
                .no_tint(true)
                .corners(RoundedCorners::None)
                .selected(mode == PreviewMode::Cuboid)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Cuboid);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/tetra.png")
                .size(Vec2::splat(20.))
                .no_tint(true)
                .corners(RoundedCorners::None)
                .selected(mode == PreviewMode::Tetra)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Tetra);
                    }),
                ),
            ToolIconButton::new("embedded://bevy_vortex/assets/icons/torus.png")
                .size(Vec2::splat(20.))
                .no_tint(true)
                .corners(RoundedCorners::Right)
                .selected(mode == PreviewMode::Torus)
                .on_click(
                    cx.create_callback(|mut mode: ResMut<NextState<PreviewMode>>| {
                        mode.set(PreviewMode::Torus);
                    }),
                ),
        ))
    }
}

/// A marker component for our shapes
#[derive(Component)]
pub(crate) struct PreviewShape;

#[derive(Component)]
pub(crate) struct Preview3DEntity;

#[derive(Resource, Default)]
pub struct PreviewShaderHandle(pub Handle<Shader>);

fn enter_preview_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2.0, 4.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 0.5,
                ..default()
            }),
            ..default()
        },
        viewport::ViewportCamera,
        Preview3DEntity,
    ));

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                // intensity: 9000.0,
                intensity: 10000000.0,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default()
        },
        Preview3DEntity,
    ));

    // ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::from(palettes::css::SILVER)),
            transform: Transform::from_xyz(0., -2.0, 0.0),
            ..default()
        },
        Preview3DEntity,
    ));
}

fn exit_preview_3d(mut commands: Commands, query: Query<Entity, With<Preview3DEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

// pub fn enter_mode_square(mut commands: Commands) {}

// pub fn exit_mode_square(mut commands: Commands) {}

fn exit_mode_shape3d(mut commands: Commands, query: Query<Entity, With<PreviewShape>>) {
    for shape in query.iter() {
        commands.entity(shape).despawn()
    }
}

fn enter_mode_cuboid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    shader: Res<PreviewShaderHandle>,
) {
    let shape = meshes.add(Cuboid::new(1.4, 1.4, 1.4));
    commands.spawn((
        shape,
        NodeShader3dHandle(shader.0.clone()),
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..SpatialBundle::INHERITED_IDENTITY
        },
        PreviewShape,
    ));
}

fn enter_mode_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    shader: Res<PreviewShaderHandle>,
) {
    let shape = meshes.add(Sphere::new(0.95).mesh().ico(5).unwrap());
    commands.spawn((
        shape,
        NodeShader3dHandle(shader.0.clone()),
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..SpatialBundle::INHERITED_IDENTITY
        },
        PreviewShape,
    ));
}

fn enter_mode_tetra(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    shader: Res<PreviewShaderHandle>,
) {
    let shape = meshes.add(Tetrahedron::default());
    commands.spawn((
        shape,
        NodeShader3dHandle(shader.0.clone()),
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.))
                .with_scale(Vec3::splat(1.4)),
            ..SpatialBundle::INHERITED_IDENTITY
        },
        PreviewShape,
    ));
}

fn enter_mode_torus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    shader: Res<PreviewShaderHandle>,
) {
    let shape = meshes.add(Torus::default());
    commands.spawn((
        shape,
        NodeShader3dHandle(shader.0.clone()),
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..SpatialBundle::INHERITED_IDENTITY
        },
        PreviewShape,
    ));
}

fn rotate_preview_shapes(mut query: Query<&mut Transform, With<PreviewShape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Update the preview shader handle based on the selected node.
fn update_preview_shader(
    mut commands: Commands,
    q_selected: Query<(&NodeOutput, Option<&NodeSelected>)>,
    q_preview_shapes: Query<Entity, With<PreviewShape>>,
    mut resource: ResMut<PreviewShaderHandle>,
    placeholder: Res<PlaceholderShaderHandle>,
) {
    let mut shader: Option<Handle<Shader>> = None;
    for (output, selected) in q_selected.iter() {
        if selected.is_some() {
            if shader.is_none() {
                shader = Some(output.shader.clone());
            } else {
                // Multiple selected, so we can't preview
                return;
            }
        }
    }

    if let Some(handle) = shader {
        if resource.0 != handle {
            // println!("Updating shader preview material to node output");
            resource.0 = handle.clone();
            for shape_entity in q_preview_shapes.iter() {
                commands
                    .entity(shape_entity)
                    .insert(NodeShader3dHandle(handle.clone()));
            }
        }
    } else if resource.0 != placeholder.0 {
        // println!("Updating shader preview material to placeholder");
        resource.0 = placeholder.0.clone();
        for shape_entity in q_preview_shapes.iter() {
            commands
                .entity(shape_entity)
                .insert(NodeShader3dHandle(placeholder.0.clone()));
        }
    }
}

#[derive(Resource, Default)]
struct PlaceholderShaderHandle(pub Handle<Shader>);

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PreviewMode::Cuboid)
            .add_computed_state::<PreviewMode3d>()
            .add_systems(Update, (update_preview_shader, rotate_preview_shapes))
            .add_systems(OnEnter(PreviewMode3d), enter_preview_3d)
            .add_systems(OnExit(PreviewMode3d), exit_preview_3d)
            .add_systems(OnEnter(PreviewMode::Cuboid), enter_mode_cuboid)
            .add_systems(OnEnter(PreviewMode::Sphere), enter_mode_sphere)
            .add_systems(OnEnter(PreviewMode::Tetra), enter_mode_tetra)
            .add_systems(OnEnter(PreviewMode::Torus), enter_mode_torus)
            .add_systems(OnExit(PreviewMode::Sphere), exit_mode_shape3d)
            .add_systems(OnExit(PreviewMode::Cuboid), exit_mode_shape3d)
            .add_systems(OnExit(PreviewMode::Tetra), exit_mode_shape3d)
            .add_systems(OnExit(PreviewMode::Torus), exit_mode_shape3d);
    }

    fn finish(&self, app: &mut App) {
        let mut shader_assets = app.world_mut().resource_mut::<Assets<Shader>>();
        let handle = shader_assets.add(Shader::from_wgsl(
            include_str!("placeholder_shader.wgsl"),
            "".to_string(),
        ));
        app.insert_resource(PlaceholderShaderHandle(handle.clone()));
        app.insert_resource(PreviewShaderHandle(handle));
    }
}

use bevy::{color::palettes, prelude::*, ui};
use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderLayout};
use bevy_quill::*;
use quill_obsidian::{
    controls::{Button, ToolIconButton, ToolPalette},
    viewport, RoundedCorners,
};

use std::f32::consts::PI;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PreviewMode {
    #[default]
    Square,
    Square2X2,
    Square3X3,
    Sphere,
    Cuboid,
    Tetra,
    Torus,
}

// While we can simply do `OnEnter(GameState::InGame{paused: true})`,
// we need to be able to reason about "while we're in the game, paused or not".
// To this end, we define the `InGame` computed state.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PreviewMode3d;

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
                Button::new().children("X"),
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
pub(crate) struct Shape;

#[derive(Component)]
pub(crate) struct Preview3DEntity;

pub fn enter_preview_3d(
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

pub fn exit_preview_3d(mut commands: Commands, query: Query<Entity, With<Preview3DEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

// pub fn enter_mode_square(mut commands: Commands) {}

// pub fn exit_mode_square(mut commands: Commands) {}

pub fn exit_mode_shape3d(mut commands: Commands, query: Query<Entity, With<Shape>>) {
    for shape in query.iter() {
        commands.entity(shape).despawn()
    }
}

pub fn enter_mode_cuboid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shape = meshes.add(Cuboid::new(1.4, 1.4, 1.4));
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Shape,
    ));
}

pub fn enter_mode_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shape = meshes.add(Sphere::new(0.95).mesh().ico(5).unwrap());
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Shape,
    ));
}

pub fn enter_mode_tetra(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shape = meshes.add(Tetrahedron::default());
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.))
                .with_scale(Vec3::splat(1.4)),
            ..default()
        },
        Shape,
    ));
}

pub fn enter_mode_torus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shape = meshes.add(Torus::default());
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Shape,
    ));
}

pub fn rotate_shapes(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

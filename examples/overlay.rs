//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use std::f32::consts::PI;

use bevy::{color::palettes, prelude::*};
use bevy_quill::prelude::*;
use bevy_quill_overlays::{
    Overlay, PolygonOptions, QuillOverlaysPlugin, ShapeOrientation, StrokeMarker,
};
use common::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            QuillPlugin,
            QuillOverlaysPlugin,
        ))
        .add_systems(Startup, (setup, setup_view_root.after(setup)))
        .add_systems(Update, (close_on_esc, rotate))
        .run();
}

fn setup_view_root(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
    let camera = q_camera.single();
    commands.spawn(
        Element::<SpatialBundle>::new()
            .insert(TargetCamera(camera))
            .children(OverlayExample)
            .to_root(),
    );
}

/// Example of a view template that displays a string.
#[derive(Clone, PartialEq)]
struct OverlayExample;

impl ViewTemplate for OverlayExample {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        let mut trans = Transform::from_translation(Vec3::new(0., 0.03, 0.));
        trans.rotate_local_y(-PI * 0.3);

        Overlay::new()
            .shape(|sb| {
                sb.with_stroke_width(0.3)
                    .with_orientation(ShapeOrientation::YPositive)
                    .stroke_rect(Rect::from_center_size(Vec2::new(0., 0.), Vec2::new(2., 2.)))
                    .fill_rect(Rect::from_center_size(Vec2::new(3., 0.), Vec2::new(2., 2.)))
                    .stroke_circle(Vec2::new(0., 3.), 0.7, 32)
                    .fill_circle(Vec2::new(3., 3.), 0.7, 32)
                    .fill_triangle(Vec2::new(-3., 3.), Vec2::new(-2., 3.), Vec2::new(-3., 4.))
                    .fill_quad(
                        Vec2::new(-3., 6.),
                        Vec2::new(-2., 6.),
                        Vec2::new(-2., 8.),
                        Vec2::new(-3., 7.),
                    )
                    .stroke_polygon(
                        &[
                            Vec2::new(3., -2.),
                            Vec2::new(2., -2.),
                            Vec2::new(2., -4.),
                            Vec2::new(3., -5.),
                        ],
                        PolygonOptions::default(),
                    )
                    .stroke_polygon(
                        &[
                            Vec2::new(5., -2.),
                            Vec2::new(4., -2.),
                            Vec2::new(4., -4.),
                            Vec2::new(5., -5.),
                        ],
                        PolygonOptions {
                            closed: true,
                            ..Default::default()
                        },
                    )
                    .stroke_polygon(
                        &[
                            Vec2::new(7., -2.),
                            Vec2::new(6., -2.),
                            Vec2::new(6., -4.),
                            Vec2::new(7., -5.),
                        ],
                        PolygonOptions {
                            start_marker: StrokeMarker::Arrowhead,
                            end_marker: StrokeMarker::Arrowhead,
                            ..default()
                        },
                    )
                    .stroke_polygon_3d(
                        &[
                            Vec3::new(1., 0.1, -2.),
                            Vec3::new(0., 0., -2.),
                            Vec3::new(0., 0., -4.),
                            Vec3::new(1., -0.2, -5.),
                        ],
                        PolygonOptions {
                            start_marker: StrokeMarker::Arrowhead,
                            end_marker: StrokeMarker::Arrowhead,
                            ..default()
                        },
                    )
                    .stroke_line_segment(Vec2::new(4., -6.), Vec2::new(5., -7.))
                    .stroke_line_segment_3d(Vec3::new(4., 1., -6.), Vec3::new(5., 1.5, -7.));
            })
            .color(palettes::css::YELLOW)
            .transform(trans)
    }
}

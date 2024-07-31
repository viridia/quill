//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use std::f32::consts::PI;

use bevy::{color::palettes, prelude::*};
use bevy_quill::prelude::*;
use bevy_quill_overlays::{Overlay, QuillOverlaysPlugin};
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
                sb.with_stroke_width(0.5)
                    .stroke_rect(Rect::from_center_size(Vec2::new(0., 0.), Vec2::new(2., 2.)));
            })
            .color(palettes::css::YELLOW)
            .transform(trans)
    }
}

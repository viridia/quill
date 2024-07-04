//! Demo of button variations.
#![feature(impl_trait_in_assoc_type)]

use bevy::{prelude::*, ui};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{colors, controls::Swatch, ObsidianUiPlugin};

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Grid)
        .grid_template_columns(vec![
            ui::RepeatedGridTrack::auto(1),
            ui::RepeatedGridTrack::auto(1),
        ])
        .grid_auto_flow(ui::GridAutoFlow::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .position(ui::PositionType::Absolute)
        .padding(3)
        .left(0)
        .right(0)
        .top(0)
        .bottom(0)
        .row_gap(4)
        .background_color(colors::U2);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
        ))
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands
        .spawn((Camera2dBundle {
            camera: Camera::default(),
            camera_2d: Camera2d {},
            ..default()
        },))
        .id();

    commands.spawn(ButtonsDemo { camera }.to_root());
}

#[derive(Clone, PartialEq)]
struct ButtonsDemo {
    camera: Entity,
}

impl ViewTemplate for ButtonsDemo {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .insert_dyn(TargetCamera, self.camera)
            .style(style_test)
            .children((
                ("U1", Swatch::new(colors::U1)),
                ("U2", Swatch::new(colors::U2)),
                ("U3", Swatch::new(colors::U3)),
                ("U4", Swatch::new(colors::U4)),
                ("U5", Swatch::new(colors::U5)),
                ("BACKGROUND", Swatch::new(colors::BACKGROUND)),
                ("FOREGROUND", Swatch::new(colors::FOREGROUND)),
                ("DIM", Swatch::new(colors::DIM)),
                ("ACCENT", Swatch::new(colors::ACCENT)),
                ("ANIMATION", Swatch::new(colors::ANIMATION)),
                ("ASSET", Swatch::new(colors::ASSET)),
                ("CODE", Swatch::new(colors::CODE)),
                ("LIGHT", Swatch::new(colors::LIGHT)),
                ("RESOURCE", Swatch::new(colors::RESOURCE)),
                ("X_RED", Swatch::new(colors::X_RED)),
                ("Y_GREEN", Swatch::new(colors::Y_GREEN)),
                ("Z_BLUE", Swatch::new(colors::Z_BLUE)),
                ("PRIMARY", Swatch::new(colors::PRIMARY)),
                ("PRIMARY_ACC", Swatch::new(colors::PRIMARY_ACC)),
                ("DESTRUCTIVE", Swatch::new(colors::DESTRUCTIVE)),
                ("DESTRUCTIVE_ACC", Swatch::new(colors::DESTRUCTIVE_ACC)),
                ("TRANSPARENT", Swatch::new(colors::TRANSPARENT)),
                ("FOCUS", Swatch::new(colors::FOCUS)),
                ("TEXT_SELECT", Swatch::new(colors::TEXT_SELECT)),
            ))
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

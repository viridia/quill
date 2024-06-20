//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_picking::{debug::DebugPickingMode, DefaultPickingPlugins};
use bevy_mod_stylebuilder::*;
use common::*;
use obsidian_ui_quill::{hooks::UseIsHover, ObsidianUiPlugin};
use quill::{Cx, Element, QuillPlugin, View, ViewTemplate};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
        ))
        .insert_resource(DebugPickingMode::Disabled)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(Element::<NodeBundle>::new().children(HoverTest).to_root());
}

#[derive(Clone, PartialEq)]
struct HoverTest;

impl ViewTemplate for HoverTest {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let hovering = cx.use_is_hover(id);
        Element::<NodeBundle>::for_entity(id)
            .style_effect(
                |hovering, ss| {
                    if hovering {
                        ss.border_color(palettes::css::RED).border(3);
                    } else {
                        ss.border_color(palettes::css::LIME).border(3);
                    }
                },
                hovering,
            )
            .children("Hover Me ")
    }
}

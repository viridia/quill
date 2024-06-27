//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Dynamic, Element, IntoViewChild, QuillPlugin, View, ViewTemplate};
use common::*;

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            QuillPlugin,
        ))
        // .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate, update_counter))
        .run();
}

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .border_color(palettes::css::LIME)
        .border(3)
        .padding(3);
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(
        Element::<NodeBundle>::new()
            .style(style_test)
            .children(("Hello, ", "world! ", EvenOdd))
            .to_root(),
    );
}

#[derive(Clone, PartialEq)]
struct EvenOdd;

impl ViewTemplate for EvenOdd {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        Dynamic::new(if counter.count & 1 == 0 {
            Element::<NodeBundle>::new()
                .children("[Even]")
                .into_view_child()
        } else {
            "[Odd]".into_view_child()
        })
    }
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
    pub foo: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Space) {
        println!("-- Space pressed --");
        counter.count += 1;
    }
}

//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cond, Cx, Element, QuillPlugin, View, ViewTemplate};
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
            .children((
                "Hello, ",
                "world! ",
                ChildViewExample,
                " ",
                EvenOdd,
                " ",
                Nested,
                DynamicStyle,
            ))
            .to_root(),
    );
}

/// Example of a view template that displays a string.
#[derive(Clone, PartialEq)]
struct ChildViewExample;

impl ViewTemplate for ChildViewExample {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        format!("{}", counter.count)
    }
}

/// Example of a `Cond` view.
#[derive(Clone, PartialEq)]
struct EvenOdd;

impl ViewTemplate for EvenOdd {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        Cond::new(counter.count & 1 == 0, "[Even]", "[Odd]")
    }
}

/// Example of a view template that invokes another view template.
#[derive(Clone, PartialEq)]
struct Nested;

impl ViewTemplate for Nested {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        NestedInner {
            count: counter.count,
        }
    }
}

#[derive(Clone, PartialEq)]
struct NestedInner {
    count: u32,
}

impl ViewTemplate for NestedInner {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Cond::new(self.count & 1 == 0, "[E]", "[O]")
    }
}

#[derive(Clone, PartialEq)]
struct DynamicStyle;

impl ViewTemplate for DynamicStyle {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        Element::<NodeBundle>::new()
            .style_dyn(
                |ct, ss| {
                    if ct {
                        ss.border_color(palettes::css::RED).border(3);
                    } else {
                        ss.border_color(palettes::css::LIME).border(3);
                    }
                },
                counter.count & 1 == 0,
            )
            .children("Style")
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

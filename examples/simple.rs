//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_stylebuilder::*;
use common::*;
use quill::{Cond, Cx, Element, QuillPlugin, View, ViewTemplate};

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
            ))
            .to_root(),
    );
    //     commands.spawn(ViewRoot::new(
    //         Element::<NodeBundle>::new()
    //             .style(style_test)
    //             .insert(BorderColor(palettes::css::LIME.into()))
    //             .insert_computed(|cx| {
    //                 let counter = cx.use_resource::<Counter>();
    //                 BackgroundColor(if counter.count & 1 == 0 {
    //                     palettes::css::DARK_GRAY.into()
    //                 } else {
    //                     palettes::css::MAROON.into()
    //                 })
    //             })
    //             .create_effect(|cx, ent| {
    //                 let count = cx.use_resource::<Counter>().count;
    //                 let mut border = cx.world_mut().get_mut::<BorderColor>(ent).unwrap();
    //                 border.0 = if count & 1 == 0 {
    //                     palettes::css::LIME.into()
    //                 } else {
    //                     palettes::css::RED.into()
    //                 };
    //             })
    //             .children((
    //                 Element::<NodeBundle>::new(),
    //                 text("Count: "),
    //                 text_computed(|cx| {
    //                     let counter = cx.use_resource::<Counter>();
    //                     format!("{}", counter.count)
    //                 }),
    //                 ", ",
    //                 NestedView,
    //                 ": ",
    //                 Cond::new(
    //                     |cx: &Rcx| {
    //                         let counter = cx.use_resource::<Counter>();
    //                         counter.count & 1 == 0
    //                     },
    //                     || "[Even]",
    //                     || "[Odd]",
    //                 ),
    //                 DynamicKeyed::new(
    //                     |cx| cx.use_resource::<Counter>().count,
    //                     |count| format!(":{}:", count),
    //                 ),
    //                 For::each(
    //                     |cx| {
    //                         let counter = cx.use_resource::<Counter>();
    //                         [counter.count, counter.count + 1, counter.count + 2].into_iter()
    //                     },
    //                     |item| format!("item: {}", item),
    //                 ),
    //             )),
    //     ));
}

#[derive(Clone, PartialEq)]
struct ChildViewExample;

impl ViewTemplate for ChildViewExample {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        // println!("NestedView::create() counter={}", counter.count);
        format!("{}", counter.count)
    }
}

#[derive(Clone, PartialEq)]
struct EvenOdd;

impl ViewTemplate for EvenOdd {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        // println!("EvenOdd::create() counter={}", counter.count);
        Cond::new(counter.count & 1 == 0, "[Even]", "[Odd]")
        // Element::<NodeBundle>::new().children(Cond::new(counter.count & 1 == 0, "[Even]", "[Odd]"))
    }
}

#[derive(Clone, PartialEq)]
struct Nested;

impl ViewTemplate for Nested {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let counter = cx.use_resource::<Counter>();
        NestedInner {
            count: counter.count,
        }
        // Element::<NodeBundle>::new().children(NestedInner {
        //     count: counter.count,
        // })
    }
}

#[derive(Clone, PartialEq)]
struct NestedInner {
    count: u32,
}

impl ViewTemplate for NestedInner {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Cond::new(self.count & 1 == 0, "[Evenish]", "[Oddish]")
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

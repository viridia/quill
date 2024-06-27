//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Element, For, QuillPlugin, View, ViewTemplate};
use common::*;

fn main() {
    App::new()
        .init_resource::<List>()
        .init_resource::<Random32>()
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

const SUITS: &[&str] = &["hearts", "spades", "clubs", "diamonds"];

fn setup_view_root(mut commands: Commands) {
    commands.spawn(
        Element::<NodeBundle>::new()
            .style(style_test)
            .children((IterExample, IndexedIterExample))
            .to_root(),
    );
}

/// Example of a view template that displays a string.
#[derive(Clone, PartialEq)]
struct IterExample;

impl ViewTemplate for IterExample {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let items = cx.use_resource::<List>();
        Element::<NodeBundle>::new().children((
            "Suits: ",
            For::each(items.items.clone(), |item| format!("[{}]", item)).with_fallback("No items"),
        ))
    }
}

#[derive(Clone, PartialEq)]
struct IndexedIterExample;

impl ViewTemplate for IndexedIterExample {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let items = cx.use_resource::<List>();
        Element::<NodeBundle>::new().children((
            "Suits: ",
            For::index(&items.items, |item, index| format!("[{}:{}]", item, index))
                .with_fallback("No items"),
        ))
    }
}

#[derive(Resource, Default)]
pub struct List {
    pub items: Vec<String>,
}

fn update_counter(
    mut list: ResMut<List>,
    key: Res<ButtonInput<KeyCode>>,
    mut random: ResMut<Random32>,
) {
    if key.pressed(KeyCode::Space) {
        println!("-- Space pressed --");
        let i = (random.next() as usize) % SUITS.len();
        list.items.push(SUITS[i].to_string());
        while list.items.len() > 10 {
            list.items.remove(0);
        }
    } else if key.pressed(KeyCode::Minus) {
        println!("-- Minus pressed --");
        list.items.pop();
    }
}

#[derive(Resource)]
struct Random32 {
    state: u32,
}

impl Random32 {
    // Generate a pseudo-random number
    fn next(&mut self) -> u32 {
        // Constants for 32-bit LCG (example values, you might want to choose different ones)
        let a: u32 = 1664525; // Multiplier
        let c: u32 = 1013904223; // Increment
        let m: u32 = 2u32.pow(31); // Modulus, often set to 2^31 for a 32-bit generator

        // Simple LCG formula: X_{n+1} = (aX_n + c) mod m
        self.state = (a.wrapping_mul(self.state).wrapping_add(c)) % m;
        self.state
    }
}

impl Default for Random32 {
    fn default() -> Self {
        Self { state: 17 }
    }
}

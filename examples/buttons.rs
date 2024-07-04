//! Demo of button variations.
#![feature(impl_trait_in_assoc_type)]

use bevy::{prelude::*, ui};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{
    colors,
    controls::{Button, ButtonVariant, IconButton},
    size::Size,
    ObsidianUiPlugin, RoundedCorners,
};

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .position(ui::PositionType::Absolute)
        .padding(3)
        .left(0)
        .right(0)
        .top(0)
        .bottom(0)
        .row_gap(4)
        .background_color(colors::U2);
}

fn style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
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
    fn create(&self, cx: &mut Cx) -> Self::View {
        let click = cx.create_callback(|| {
            info!("Clicked!");
        });
        Element::<NodeBundle>::new()
            .insert_dyn(TargetCamera, self.camera)
            .style(style_test)
            .children((
                "Variants",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new().on_click(click).children("Default"),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Primary)
                        .children("Primary"),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Danger)
                        .children("Danger"),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Selected)
                        .children("Selected"),
                    Button::new().minimal(true).children("Minimal"),
                )),
                "Variants (disabled)",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new()
                        .on_click(click)
                        .children("Default")
                        .disabled(true),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Primary)
                        .children("Primary")
                        .disabled(true),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Danger)
                        .children("Danger")
                        .disabled(true),
                    Button::new()
                        .on_click(click)
                        .variant(ButtonVariant::Selected)
                        .children("Selected")
                        .disabled(true),
                    Button::new()
                        .on_click(click)
                        .minimal(true)
                        .children("Minimal")
                        .disabled(true),
                )),
                "Size",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new()
                        .on_click(click)
                        .children("size: Xl")
                        .size(Size::Xl),
                    Button::new()
                        .on_click(click)
                        .children("size: Lg")
                        .size(Size::Lg),
                    Button::new()
                        .on_click(click)
                        .children("size: Md")
                        .size(Size::Md),
                    Button::new()
                        .on_click(click)
                        .children("size: Sm")
                        .size(Size::Sm),
                    Button::new()
                        .on_click(click)
                        .children("size: Xs")
                        .size(Size::Xs),
                    Button::new()
                        .on_click(click)
                        .children("size: Xxs")
                        .size(Size::Xxs),
                    Button::new()
                        .on_click(click)
                        .children("size: Xxxs")
                        .size(Size::Xxxs),
                )),
                "Corners",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new()
                        .on_click(click)
                        .children("corners: All")
                        .corners(RoundedCorners::All),
                    Button::new()
                        .on_click(click)
                        .children("corners: Top")
                        .corners(RoundedCorners::Top),
                    Button::new()
                        .on_click(click)
                        .children("corners: Bottom")
                        .corners(RoundedCorners::Bottom),
                    Button::new()
                        .on_click(click)
                        .children("corners: Left")
                        .corners(RoundedCorners::Left),
                    Button::new()
                        .on_click(click)
                        .children("corners: Right")
                        .corners(RoundedCorners::Right),
                    Button::new()
                        .on_click(click)
                        .children("corners: None")
                        .corners(RoundedCorners::None),
                )),
                "IconButton",
                Element::<NodeBundle>::new().style(style_row).children((
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .on_click(click),
                    // IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                    //     .variant(ButtonVariant::Primary),
                    // IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                    //     .variant(ButtonVariant::Danger),
                    // IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                    //     .variant(ButtonVariant::Selected),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .on_click(click)
                        .minimal(true),
                )),
                "IconButton Size",
                Element::<NodeBundle>::new().style(style_row).children((
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Xl)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Lg)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Md)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Sm)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Xs)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Xxs)
                        .on_click(click),
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/chevron_left.png")
                        .size(Size::Xxxs)
                        .on_click(click),
                )),
            ))
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

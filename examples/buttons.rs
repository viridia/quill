//! Example of a simple UI layout

use bevy::{
    asset::io::{file::FileAssetReader, AssetSource},
    prelude::*,
    ui,
};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::{
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
        .background_color(colors::U1);
}

fn style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
}

fn main() {
    App::new()
        .register_asset_source(
            "obsidian_ui",
            AssetSource::build()
                .with_reader(|| Box::new(FileAssetReader::new("crates/quill_obsidian/assets"))),
        )
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
            StyleBuilderPlugin,
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

    commands.spawn(
        Element::<NodeBundle>::new()
            .insert(move |_| TargetCamera(camera), ())
            .style(style_test)
            .children((
                "Variants",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new().children("Default"),
                    Button::new()
                        .variant(ButtonVariant::Primary)
                        .children("Primary"),
                    Button::new()
                        .variant(ButtonVariant::Danger)
                        .children("Danger"),
                    Button::new()
                        .variant(ButtonVariant::Selected)
                        .children("Selected"),
                    Button::new().minimal(true).children("Minimal"),
                )),
                "Variants (disabled)",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new().children("Default").disabled(true),
                    Button::new()
                        .variant(ButtonVariant::Primary)
                        .children("Primary")
                        .disabled(true),
                    Button::new()
                        .variant(ButtonVariant::Danger)
                        .children("Danger")
                        .disabled(true),
                    Button::new()
                        .variant(ButtonVariant::Selected)
                        .children("Selected")
                        .disabled(true),
                    Button::new()
                        .minimal(true)
                        .children("Minimal")
                        .disabled(true),
                )),
                "Size",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new().children("size: Xl").size(Size::Xl),
                    Button::new().children("size: Lg").size(Size::Lg),
                    Button::new().children("size: Md").size(Size::Md),
                    Button::new().children("size: Sm").size(Size::Sm),
                    Button::new().children("size: Xs").size(Size::Xs),
                    Button::new().children("size: Xxs").size(Size::Xxs),
                    Button::new().children("size: Xxxs").size(Size::Xxxs),
                )),
                "Corners",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new()
                        .children("corners: All")
                        .corners(RoundedCorners::All),
                    Button::new()
                        .children("corners: Top")
                        .corners(RoundedCorners::Top),
                    Button::new()
                        .children("corners: Bottom")
                        .corners(RoundedCorners::Bottom),
                    Button::new()
                        .children("corners: Left")
                        .corners(RoundedCorners::Left),
                    Button::new()
                        .children("corners: Right")
                        .corners(RoundedCorners::Right),
                    Button::new()
                        .children("corners: None")
                        .corners(RoundedCorners::None),
                )),
                "IconButton",
                Element::<NodeBundle>::new().style(style_row).children((
                    IconButton::new("obsidian_ui://icons/chevron_left.png"),
                    // IconButton::new("obsidian_ui://icons/chevron_left.png")
                    //     .variant(ButtonVariant::Primary),
                    // IconButton::new("obsidian_ui://icons/chevron_left.png")
                    //     .variant(ButtonVariant::Danger),
                    // IconButton::new("obsidian_ui://icons/chevron_left.png")
                    //     .variant(ButtonVariant::Selected),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").minimal(true),
                )),
                "IconButton Size",
                Element::<NodeBundle>::new().style(style_row).children((
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Xl),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Lg),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Md),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Sm),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Xs),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Xxs),
                    IconButton::new("obsidian_ui://icons/chevron_left.png").size(Size::Xxxs),
                )),
            ))
            .to_root(),
    );
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

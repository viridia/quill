//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

use bevy::{color::palettes, prelude::*, ui};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use quill_obsidian::{
    colors,
    controls::{
        Button, ButtonVariant, Checkbox, ColorGradient, Dialog, DialogFooter, DialogHeader,
        GradientSlider, MenuButton, MenuDivider, MenuItem, MenuPopup, Slider, SpinBox, Swatch,
    },
    ObsidianUiPlugin,
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

fn style_swatch(ss: &mut StyleBuilder) {
    ss.width(24).height(24);
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
        let checked = cx.create_mutable(true);
        let disabled = cx.create_mutable(false);
        let dialog_open = cx.create_mutable(false);
        let on_checked = cx.create_callback(move |value: In<bool>, world: &mut World| {
            checked.set(world, *value);
            // info!("Checked: {}", *value);
        });
        let spin_value = cx.create_mutable::<f32>(50.);
        let slider_value = cx.create_mutable::<f32>(50.);
        let color_value = cx.create_mutable::<Srgba>(Srgba::new(1.0, 0.0, 0.0, 1.0));
        let color = color_value.get(cx);
        Element::<NodeBundle>::new()
            .insert_dyn(TargetCamera, self.camera)
            .style(style_test)
            .children((
                "Checkbox",
                Element::<NodeBundle>::new().style(style_row).children((
                    Checkbox::new()
                        .checked(checked.get(cx))
                        .on_change(on_checked)
                        .label("Checkbox"),
                    Checkbox::new()
                        .disabled(disabled.get(cx))
                        .checked(checked.get(cx))
                        .on_change(on_checked)
                        .label("Checkbox (disabled)"),
                    Checkbox::new()
                        .checked(disabled.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<bool>, world: &mut World| {
                                disabled.set(world, *value);
                                // info!("Disabled: {}", *value);
                            }),
                        )
                        .label("Disable"),
                )),
                "Swatch",
                Element::<NodeBundle>::new().style(style_row).children((
                    Swatch::new(palettes::css::RED).style(style_swatch),
                    Swatch::new(colors::U1).style(style_swatch),
                    Swatch::new(Srgba::new(0.5, 1.0, 0.0, 0.5)).style(style_swatch),
                )),
                "Spinbox",
                Element::<NodeBundle>::new()
                    .style(style_row)
                    .children((SpinBox::new()
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(100);
                        })
                        .value(spin_value.get(cx))
                        .on_change(cx.create_callback(
                            move |value: In<f32>, world: &mut World| {
                                spin_value.set(world, *value);
                            },
                        )),)),
                "Slider",
                Element::<NodeBundle>::new().style(style_row).children((
                    " Normal:",
                    Slider::new()
                        .range(0. ..=100.)
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(100);
                        })
                        .value(slider_value.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<f32>, world: &mut World| {
                                slider_value.set(world, *value);
                            }),
                        ),
                    " Compact:",
                    Slider::new()
                        .range(0. ..=100.)
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(50);
                        })
                        .value(slider_value.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<f32>, world: &mut World| {
                                slider_value.set(world, *value);
                            }),
                        ),
                    " Precision:",
                    Slider::new()
                        .range(0. ..=100.)
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(100);
                        })
                        .precision(2)
                        .value(slider_value.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<f32>, world: &mut World| {
                                slider_value.set(world, *value);
                            }),
                        ),
                    " Labeled:",
                    Slider::new()
                        .range(0. ..=100.)
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(100);
                        })
                        .label("Gain")
                        .value(slider_value.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<f32>, world: &mut World| {
                                slider_value.set(world, *value);
                            }),
                        ),
                    " Custom Format:",
                    Slider::new()
                        .range(0. ..=100.)
                        .style(|sb: &mut StyleBuilder| {
                            sb.width(100);
                        })
                        .formatted_value(format!("{:.0} dB", slider_value.get(cx)))
                        .value(slider_value.get(cx))
                        .on_change(
                            cx.create_callback(move |value: In<f32>, world: &mut World| {
                                slider_value.set(world, *value);
                            }),
                        ),
                )),
                "GradientSlider",
                GradientSlider::new()
                    .gradient(ColorGradient::new(&[
                        Srgba::new(color.red, 0.0, color.blue, 1.0),
                        Srgba::new(color.red, 1.0, color.blue, 1.0),
                    ]))
                    .min(0.)
                    .max(255.)
                    .value(color.green * 255.0)
                    .style(|sb: &mut StyleBuilder| {
                        sb.width(100);
                    })
                    .precision(1)
                    .on_change(
                        cx.create_callback(move |value: In<f32>, world: &mut World| {
                            color_value.update(world, |mut state| state.green = *value / 255.0);
                        }),
                    ),
                "Dialog",
                Element::<NodeBundle>::new().style(style_row).children((
                    Button::new()
                        .on_click(cx.create_callback(move |world: &mut World| {
                            dialog_open.set(world, true);
                        }))
                        .children("Open..."),
                    Dialog::new()
                        .width(ui::Val::Px(400.))
                        .open(dialog_open.get(cx))
                        .on_close(cx.create_callback(move |world: &mut World| {
                            dialog_open.set(world, false);
                        }))
                        .children((
                            DialogHeader::new().children("Dialog Header"),
                            DialogFooter::new().children((
                                Button::new()
                                    .children("Cancel")
                                    .on_click(cx.create_callback(move |world: &mut World| {
                                        dialog_open.set(world, false);
                                    })),
                                Button::new()
                                    .children("Close")
                                    .variant(ButtonVariant::Primary)
                                    .autofocus(true)
                                    .on_click(cx.create_callback(move |world: &mut World| {
                                        dialog_open.set(world, false);
                                    })),
                            )),
                        )),
                )),
                "MenuButton",
                Element::<NodeBundle>::new()
                    .style(style_row)
                    .children((MenuButton::new().children("Menu").popup(
                        MenuPopup::new().children((
                            MenuItem::new()
                                .label("Alpha Male")
                                .on_click(cx.create_callback(|| {
                                    println!("Alpha item clicked");
                                })),
                            MenuItem::new()
                                .label("Beta Test")
                                .on_click(cx.create_callback(|| {
                                    println!("Beta item clicked");
                                })),
                            MenuItem::new()
                                .label("Delta Sleep")
                                .on_click(cx.create_callback(|| {
                                    println!("Delta item clicked");
                                })),
                            MenuItem::new()
                                .label("Gamma Ray")
                                .on_click(cx.create_callback(|| {
                                    println!("Gamma item clicked");
                                })),
                            MenuDivider,
                            MenuItem::new()
                                .label("Omega Point")
                                .on_click(cx.create_callback(|| {
                                    println!("Omega item clicked");
                                })),
                        )),
                    ),)),
            ))
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

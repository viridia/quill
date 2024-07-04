use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{
    colors,
    controls::{Icon, MenuButton, MenuPopup, Spacer, Swatch},
    floating::{FloatAlign, FloatSide},
    size::Size,
};

fn style_field(ss: &mut StyleBuilder) {
    ss.flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexStart)
        .color(colors::FOREGROUND);
}

fn style_swatch(ss: &mut StyleBuilder) {
    ss.width(16).height(16).margin_right(4);
}

fn style_menu_icon(ss: &mut StyleBuilder) {
    ss.margin((2, 0));
}

use crate::{
    templates::{
        color_edit::{ColorEdit, ColorEditState, ColorMode, RecentColors},
        field_label::FieldLabel,
    },
    Inspectable,
};

#[derive(Clone)]
pub struct SrgbaInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for SrgbaInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for SrgbaInspector {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let value = match field.reflect(cx) {
            Some(value) if value.is::<Srgba>() => *value.downcast_ref::<Srgba>().unwrap(),
            _ => Srgba::NONE,
        };

        let state = cx.create_mutable(ColorEditState {
            mode: ColorMode::Rgb,
            rgb: Srgba::default(),
            hsl: Hsla::default(),
        });

        let field = self.0.clone();
        cx.create_effect(
            move |world, _| {
                let next_state = state.get(world);
                if let Some(reflect) = field.reflect_untracked(world) {
                    let value = *reflect.downcast_ref::<Srgba>().unwrap();
                    if value != next_state.rgb {
                        field.set_value(world, &next_state.rgb);
                    }
                }
            },
            (),
        );

        (
            FieldLabel {
                field: self.0.clone(),
            },
            Element::<NodeBundle>::new().style(style_field).children((
                Swatch::new(value).style(style_swatch),
                value.to_hex(),
                Spacer,
                MenuButton::new()
                    .children(
                        Icon::new("embedded://bevy_quill_obsidian/assets/icons/tune.png")
                            .size(Vec2::splat(16.0))
                            .style(style_menu_icon)
                            .color(Color::from(colors::DIM)),
                    )
                    .popup(
                        MenuPopup::new()
                            .side(FloatSide::Right)
                            .align(FloatAlign::Start)
                            .children(ColorEdit::new(
                                state.get(cx),
                                cx.create_callback(
                                    move |st: In<ColorEditState>, world: &mut World| {
                                        state.set(world, *st);
                                    },
                                ),
                            )),
                    )
                    .size(Size::Xxs)
                    .minimal(true)
                    .no_caret(true)
                    .on_state_change(cx.create_callback(
                        move |open: In<bool>, world: &mut World| {
                            // When popup closes, we're done editing, so add to recent colors.
                            if !*open {
                                // Add color to recent colors.
                                let color = state.get(world).rgb;
                                let mut recent_colors =
                                    world.get_resource_mut::<RecentColors>().unwrap();
                                recent_colors.add(color);
                            }
                        },
                    )),
            )),
        )
    }
}

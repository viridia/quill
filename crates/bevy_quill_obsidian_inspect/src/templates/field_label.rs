use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{
    colors,
    controls::{IconButton, Spacer},
    size::Size,
    typography,
};

use crate::Inspectable;

fn style_field_label(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexStart)
        .font_size(16)
        .color(colors::DIM)
        .padding_left(16);
}

/// Label for editable struct field in an inspector.
#[derive(Clone)]
pub struct FieldLabel {
    /// The content of the label.
    pub field: Arc<Inspectable>,
}

impl PartialEq for FieldLabel {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
    }
}

impl ViewTemplate for FieldLabel {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let name = self.field.name().to_string();
        let can_remove = self.field.can_remove();
        let field = self.field.clone();
        let remove = cx.create_callback(move |world: &mut World| {
            field.remove(world);
        });
        Element::<NodeBundle>::new()
            .style((typography::text_default, style_field_label))
            .children((
                name,
                Cond::new(
                    can_remove,
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/close.png")
                        .size(Size::Xs)
                        .minimal(true)
                        .on_click(remove),
                    (),
                ),
                Spacer,
            ))
    }
}

fn style_field_label_wide(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexStart)
        .grid_column_span(2)
        .font_size(16)
        .min_width(64)
        .color(colors::DIM);
}

/// Label for editable struct field in an inspector.
#[derive(Clone)]
pub struct FieldLabelWide {
    /// The content of the label.
    pub field: Arc<Inspectable>,
    /// Name to display.
    pub name: Option<ViewChild>,
    /// Additional buttons in the label.
    pub buttons: Option<ViewChild>,
}

impl PartialEq for FieldLabelWide {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.field, &other.field)
            && self.name == other.name
            && self.buttons == other.buttons
    }
}

impl FieldLabelWide {
    /// Create a new field label.
    pub fn new(field: Arc<Inspectable>) -> Self {
        Self {
            field,
            name: None,
            buttons: None,
        }
    }

    /// Set the name of the field.
    pub fn name(mut self, name: impl IntoViewChild) -> Self {
        self.name = Some(name.into_view_child());
        self
    }

    /// Set additional buttons in the label.
    pub fn buttons(mut self, buttons: impl IntoViewChild) -> Self {
        self.buttons = Some(buttons.into_view_child());
        self
    }
}

impl ViewTemplate for FieldLabelWide {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let can_remove = self.field.can_remove();
        let field = self.field.clone();
        let remove = cx.create_callback(move |world: &mut World| {
            field.remove(world);
        });
        Element::<NodeBundle>::new()
            .style((typography::text_default, style_field_label_wide))
            .children((
                self.name.clone(),
                Spacer,
                self.buttons.clone(),
                Cond::new(
                    can_remove,
                    IconButton::new("embedded://bevy_quill_obsidian/assets/icons/close.png")
                        .size(Size::Xs)
                        .minimal(true)
                        .on_click(remove),
                    (),
                ),
            ))
    }
}

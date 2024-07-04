use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{colors, typography};

fn style_field_readonly_value(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexStart)
        .border(1)
        .border_color(colors::U3)
        .font_size(16)
        .color(colors::DIM)
        .padding((4, 1));
}

/// Readonly value displayed as text in the inspector.
#[derive(Clone, Default, PartialEq)]
pub struct FieldReadonlyValue {
    /// The text representation of the value.
    pub children: ViewChild,
    /// Additional styles for the label.
    pub style: StyleHandle,
}

impl FieldReadonlyValue {
    /// Create a new readonly value with the given text.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }

    /// Set the additional styles for the button.
    #[allow(dead_code)]
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl ViewTemplate for FieldReadonlyValue {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style((
                typography::text_default,
                style_field_readonly_value,
                self.style.clone(),
            ))
            .children(self.children.clone())
    }
}

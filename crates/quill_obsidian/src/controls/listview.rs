use bevy::ui;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::colors;

use super::ScrollView;

fn style_listview(ss: &mut StyleBuilder) {
    ss.background_color(colors::U1)
        .border_radius(5.0)
        .padding(3);
}

fn style_listview_inner(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
        .align_self(ui::AlignSelf::Stretch)
        .justify_self(ui::JustifySelf::Stretch);
}

/// A scrollable list of items.
#[derive(Clone, PartialEq, Default)]
pub struct ListView {
    /// Additional styles to be applied to the list view.
    pub style: StyleHandle,

    /// The content of the dialog header.
    pub children: ViewChild,
}

impl ListView {
    /// Create a new list view.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set additional styles to be applied to the list view.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }
}

impl ViewTemplate for ListView {
    type View = ScrollView;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        ScrollView::new()
            .children(self.children.clone())
            .style((style_listview, self.style.clone()))
            .content_style(style_listview_inner)
            .scroll_enable_y(true)
    }
}

use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill_core::*;

fn style_flex(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex);
}

fn style_flex_row(ss: &mut StyleBuilder) {
    ss.flex_row();
}

fn style_flex_column(ss: &mut StyleBuilder) {
    ss.flex_column();
}

/// A widget which creates a flexbox
#[derive(Clone, PartialEq, Default)]
pub struct Flex {
    style: StyleHandle,
    /// The content to display inside the button.
    pub children: ViewChild,
}

impl Flex {
    /// Create a new flex element
    pub fn new<S: Fn(&mut StyleBuilder) + Send + Sync + 'static>(style: S) -> Self {
        Self {
            style: (style_flex, style).into_handle(),
            children: Default::default(),
        }
    }

    /// Create a new flex element with a row layout
    pub fn row<S: Fn(&mut StyleBuilder) + Send + Sync + 'static>(style: S) -> Self {
        Self {
            style: (style_flex_row, style).into_handle(),
            children: Default::default(),
        }
    }

    /// Create a new flex element with a column layout
    pub fn column<S: Fn(&mut StyleBuilder) + Send + Sync + 'static>(style: S) -> Self {
        Self {
            style: (style_flex_column, style).into_handle(),
            children: Default::default(),
        }
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }
}

impl ViewTemplate for Flex {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style(self.style.clone())
            .children(self.children.clone())
    }
}

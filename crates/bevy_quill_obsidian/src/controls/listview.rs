use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode, Focus,
    },
    color::{Alpha, Srgba},
    prelude::*,
    ui::{self, node_bundles::NodeBundle},
};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill_core::*;

use crate::{
    colors,
    focus::TabIndex,
    hooks::{UseIsFocus, UseIsHover},
    typography,
};

use super::{IsDisabled, ScrollView};

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
        .justify_self(ui::JustifySelf::Stretch)
        .height(ui::Val::Auto)
        .min_width(ui::Val::Percent(100.));
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
            .children(
                Element::<NodeBundle>::new()
                    .named("ListView")
                    .insert(AccessibilityNode::from(NodeBuilder::new(Role::ListBox)))
                    .style(style_listview_inner)
                    .children(self.children.clone()),
            )
            .style((style_listview, self.style.clone()))
            // .content_style(style_listview_inner)
            .scroll_enable_y(true)
    }
}

/// A scrollable list of items.
#[derive(Clone, PartialEq)]
pub struct ListRow<K: PartialEq + Clone> {
    /// Unique key for this row
    pub key: K,

    /// Additional styles to be applied to the list view.
    pub style: StyleHandle,

    /// True if this row is selected
    pub selected: bool,

    /// The content of the dialog header.
    pub children: ViewChild,

    /// Callback called when row clicked
    pub on_click: Option<Callback<K>>,
}

impl<K: PartialEq + Clone> ListRow<K> {
    /// Create a new list view.
    pub fn new(key: K) -> Self {
        Self {
            key,
            style: StyleHandle::default(),
            selected: false,
            children: ViewChild::default(),
            on_click: None,
        }
    }

    /// Set additional styles to be applied to the list view.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set whether this row is selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }

    /// Set callback when clicked
    pub fn on_click(mut self, callback: Callback<K>) -> Self {
        self.on_click = Some(callback);
        self
    }
}

impl<K: PartialEq + Clone + Send + Sync + 'static> ViewTemplate for ListRow<K> {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let hovering = cx.is_hovered(id);
        let focused = cx.is_focused(id);
        let on_click = self.on_click;
        let key = self.key.clone();

        // TODO: Disabled

        Element::<NodeBundle>::for_entity(id)
            .named("ListRow")
            .insert(TabIndex(0))
            .children(self.children.clone())
            .style((typography::text_default, style_listrow, self.style.clone()))
            .style_dyn(
                |(hovering, selected), sb| {
                    sb.background_color(row_bg_color(false, selected, hovering));
                },
                (hovering, self.selected),
            )
            .style_dyn(
                move |focused, sb| {
                    match focused {
                        true => {
                            sb.border_color(colors::FOCUS).border(1).padding((5, 2));
                        }
                        false => {
                            sb.border_color(Option::<Color>::None)
                                .border(0)
                                .padding((6, 3));
                        }
                    };
                },
                focused,
            )
            .insert_dyn(
                move |_| {
                    let key = key.clone();
                    On::<Pointer<Click>>::run(move |world: &mut World| {
                        let mut focus = world.get_resource_mut::<Focus>().unwrap();
                        focus.0 = Some(id);
                        if !world.is_disabled(id) {
                            let mut event = world
                                .get_resource_mut::<ListenerInput<Pointer<Click>>>()
                                .unwrap();
                            event.stop_propagation();
                            if let Some(on_click) = on_click {
                                world.run_callback(on_click, key.clone());
                            }
                        }
                    })
                },
                (),
            )
    }
}

fn style_listrow(ss: &mut StyleBuilder) {
    ss.padding((6, 3));
}

pub(crate) fn row_bg_color(is_disabled: bool, is_selected: bool, is_hovering: bool) -> Srgba {
    match (is_disabled, is_selected, is_hovering) {
        (true, _, _) => Srgba::NONE,
        (_, true, _) => colors::TEXT_SELECT.with_alpha(0.05),
        (_, false, true) => colors::TEXT_SELECT.with_alpha(0.02),
        (_, false, false) => Srgba::NONE,
    }
}

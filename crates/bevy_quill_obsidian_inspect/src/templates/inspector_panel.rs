use bevy::{prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{colors, typography};

fn style_inspector_panel(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch);
}

fn style_inspector_panel_header(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::SpaceBetween)
        .height(24)
        .font_size(16)
        .background_color(colors::U3)
        .border_radius(ui::BorderRadius {
            top_left: ui::Val::Px(4.0),
            top_right: ui::Val::Px(4.0),
            bottom_left: ui::Val::Px(0.0),
            bottom_right: ui::Val::Px(0.0),
        })
        .color(colors::FOREGROUND)
        .padding_left(8)
        .padding_right(3);
}

fn style_inspector_panel_body(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Grid)
        .grid_auto_flow(ui::GridAutoFlow::Row)
        .grid_template_columns(vec![
            ui::RepeatedGridTrack::minmax(
                1,
                ui::MinTrackSizingFunction::Px(64.),
                ui::MaxTrackSizingFunction::Auto,
            ),
            ui::RepeatedGridTrack::flex(1, 1.),
        ])
        .column_gap(4)
        .row_gap(2)
        .border_color(colors::U3)
        .border(ui::UiRect {
            left: ui::Val::Px(1.0),
            right: ui::Val::Px(1.0),
            top: ui::Val::Px(0.0),
            bottom: ui::Val::Px(1.0),
        })
        .border_left(1)
        .border_right(1)
        .border_bottom(1)
        .border_radius(ui::BorderRadius {
            top_left: ui::Val::Px(0.0),
            top_right: ui::Val::Px(0.0),
            bottom_left: ui::Val::Px(4.0),
            bottom_right: ui::Val::Px(4.0),
        })
        .padding_left(6)
        .padding_right(4)
        .padding_top(4)
        .padding_bottom(4);
}

/// Displays a inspector panel card with a title and a body.
#[derive(Clone, PartialEq, Default)]
pub struct InspectorPanel {
    /// The content of the title section.
    pub title: ViewChild,
    /// The content of the body section.
    pub body: ViewChild,
    /// Whether the panel is expanded or not. When collapsed, only the title is shown.
    pub expanded: bool,
}

impl InspectorPanel {
    /// Create a new inspector panel with the given title and body.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title of the inspector panel.
    pub fn title(mut self, title: impl IntoViewChild) -> Self {
        self.title = title.into_view_child();
        self
    }

    /// Set the body of the inspector panel.
    pub fn body(mut self, body: impl IntoViewChild) -> Self {
        self.body = body.into_view_child();
        self
    }

    /// Set the expanded signal of the inspector panel.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
}

impl ViewTemplate for InspectorPanel {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        let expanded = self.expanded;
        let body = self.body.clone();
        Element::<NodeBundle>::new()
            .style(style_inspector_panel)
            .children((
                Element::<NodeBundle>::new()
                    .style((typography::text_default, style_inspector_panel_header))
                    .children(self.title.clone()),
                Cond::new(
                    expanded,
                    Element::<NodeBundle>::new()
                        .style(style_inspector_panel_body)
                        .children(body.clone()),
                    (),
                ),
            ))
    }
}

use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::{
    colors,
    scrolling::{ScrollArea, ScrollBar, ScrollBarThumb, ScrollContent, ScrollWheel},
};

// Style definitions for scrollview widget.

// The combined scroll view with scrolling region and scrollbars.
fn style_scroll_view(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Grid)
        .grid_template_columns(vec![
            ui::RepeatedGridTrack::flex(1, 1.),
            ui::RepeatedGridTrack::auto(1),
        ])
        .grid_template_rows(vec![
            ui::RepeatedGridTrack::flex(1, 1.),
            ui::RepeatedGridTrack::auto(1),
        ])
        .gap(2);
}

/// The scrolling region which defines the clipping bounds.
fn style_scroll_region(ss: &mut StyleBuilder) {
    ss.grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .overflow(ui::OverflowAxis::Clip);
}

/// The scrolling content which is clipped.
fn style_scroll_content(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .height(ui::Val::Auto)
        .min_width(ui::Val::Percent(100.))
        .border(1);
}

fn style_scrollbar_x(ss: &mut StyleBuilder) {
    ss.grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(2, 1))
        .height(8);
}

fn style_scrollbar_x_thumb(ss: &mut StyleBuilder) {
    ss.background_color(colors::U3.with_alpha(0.5))
        .position(ui::PositionType::Absolute)
        .top(1)
        .bottom(1)
        .border_radius(3);
}

fn style_scrollbar_y(ss: &mut StyleBuilder) {
    ss.grid_column(ui::GridPlacement::start_span(2, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .width(8);
}

fn style_scrollbar_y_thumb(ss: &mut StyleBuilder) {
    ss.background_color(colors::U3.with_alpha(0.5))
        .position(ui::PositionType::Absolute)
        .left(1)
        .right(1)
        .border_radius(3);
}

#[derive(Clone, PartialEq, Default, Copy)]
enum DragMode {
    #[default]
    None,
    DragX,
    DragY,
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    mode: DragMode,
    offset: f32,
}

/// The scroll view widget.
#[derive(Default, Clone, PartialEq)]
pub struct ScrollView {
    /// Views for the scrolling content
    pub children: ChildViews,
    /// Style to be applied to the entire scroll view,
    pub style: StyleHandle,
    /// Style to be applied to the content region,
    pub content_style: StyleHandle,
    /// Whether to enable horizontal scrolling.
    pub scroll_enable_x: bool,
    /// Whether to enable vertical scrolling.
    pub scroll_enable_y: bool,
}

impl ScrollView {
    /// Create a new `ScrollView`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoChildViews) -> Self {
        self.children = children.into_child_views();
        self
    }

    /// Set additional styles to be applied to the scroll view.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set additional styles to be applied to the scroll content.
    pub fn content_style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.content_style = style.into_handle();
        self
    }

    /// Enable horizontal scrolling.
    pub fn scroll_enable_x(mut self, enable: bool) -> Self {
        self.scroll_enable_x = enable;
        self
    }

    /// Enable vertical scrolling.
    pub fn scroll_enable_y(mut self, enable: bool) -> Self {
        self.scroll_enable_y = enable;
        self
    }
}

impl ViewTemplate for ScrollView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        // A widget which displays a scrolling view of its children.
        let enable_x = self.scroll_enable_x;
        let enable_y = self.scroll_enable_y;
        let id_scroll_area = cx.create_entity();
        let id_scrollbar_x = cx.create_entity();
        let id_scrollbar_y = cx.create_entity();
        let drag_state = cx.create_mutable::<DragState>(DragState::default());
        Element::<NodeBundle>::new()
            .named("ScrollView")
            .style((style_scroll_view, self.style.clone()))
            .children((
                // Scroll area
                Element::<NodeBundle>::for_entity(id_scroll_area)
                    .named("ScrollView::ScrollArea")
                    .insert_dyn(
                        move |_| {
                            (
                                ScrollArea {
                                    id_scrollbar_x: if enable_x {
                                        Some(id_scrollbar_x)
                                    } else {
                                        None
                                    },
                                    id_scrollbar_y: if enable_y {
                                        Some(id_scrollbar_y)
                                    } else {
                                        None
                                    },
                                    ..default()
                                },
                                On::<ScrollWheel>::listener_component_mut::<ScrollArea>(
                                    move |ev, scrolling| {
                                        ev.stop_propagation();
                                        scrolling.scroll_by(-ev.delta.x, -ev.delta.y);
                                    },
                                ),
                            )
                        },
                        (),
                    )
                    .style(style_scroll_region)
                    .children(
                        Element::<NodeBundle>::new()
                            .named("ScrollView::ScrollRegion")
                            .insert(ScrollContent)
                            .style((style_scroll_content, self.content_style.clone()))
                            .children(self.children.clone()),
                    ),
                // Horizontal scroll bar
                Cond::new(
                    enable_x,
                    Scrollbar::new(ScrollbarProps {
                        id_scroll_area,
                        id_scrollbar: id_scrollbar_x,
                        drag_state,
                        vertical: false,
                    }),
                    (),
                ),
                // Vertical scroll bar
                Cond::new(
                    enable_y,
                    Scrollbar::new(ScrollbarProps {
                        id_scroll_area,
                        id_scrollbar: id_scrollbar_y,
                        drag_state,
                        vertical: true,
                    }),
                    (),
                ),
            ))
    }
}

/// Properties for the `Scrollbar` widget.
#[derive(Clone, PartialEq)]
pub struct ScrollbarProps {
    id_scroll_area: Entity,
    id_scrollbar: Entity,
    drag_state: Mutable<DragState>,
    vertical: bool,
}

/// Scrollbar widget.
#[derive(Clone, PartialEq)]
pub struct Scrollbar(ScrollbarProps);

impl Scrollbar {
    /// Create a new `Scrollbar`.
    pub fn new(props: ScrollbarProps) -> Self {
        Self(props)
    }
}

impl ViewTemplate for Scrollbar {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let vertical = self.0.vertical;
        let drag_state = self.0.drag_state;
        let id_scroll_area = self.0.id_scroll_area;
        let id_thumb = cx.create_entity();
        Element::<NodeBundle>::for_entity(self.0.id_scrollbar)
            .named("Scrollbar")
            .insert_dyn(
                move |_| {
                    (
                        ScrollBar {
                            id_scroll_area,
                            vertical,
                            min_thumb_size: 10.,
                        },
                        // Click outside of thumb
                        On::<Pointer<DragStart>>::run(
                            move |mut ev: ListenerMut<Pointer<DragStart>>,
                                  mut query: Query<&mut ScrollArea>,
                                  query_thumb: Query<(
                                &Node,
                                &mut ScrollBarThumb,
                                &GlobalTransform,
                            )>| {
                                ev.stop_propagation();
                                if let Ok(mut scroll_area) = query.get_mut(id_scroll_area) {
                                    if let Ok((thumb, _, transform)) = query_thumb.get(id_thumb) {
                                        // Get thumb rectangle
                                        let rect = thumb.logical_rect(transform);
                                        handle_track_click(
                                            &mut scroll_area,
                                            vertical,
                                            ev.pointer_location.position,
                                            rect,
                                        );
                                    }
                                };
                            },
                        ),
                    )
                },
                (),
            )
            .style(if vertical {
                style_scrollbar_y
            } else {
                style_scrollbar_x
            })
            .children(
                Element::<NodeBundle>::for_entity(id_thumb)
                    // .class_names(CLS_DRAG.if_true(cx.read_atom(drag_state).mode == mode))
                    .style(if vertical {
                        style_scrollbar_y_thumb
                    } else {
                        style_scrollbar_x_thumb
                    })
                    .insert_dyn(
                        move |_| {
                            (
                                ScrollBarThumb,
                                // Click/Drag on thumb
                                On::<Pointer<DragStart>>::run(move |world: &mut World| {
                                    let mut event = world
                                        .get_resource_mut::<ListenerInput<Pointer<DragStart>>>()
                                        .unwrap();
                                    event.stop_propagation();
                                    if let Some(scroll_area) =
                                        world.get::<ScrollArea>(id_scroll_area)
                                    {
                                        drag_state.set(
                                            world,
                                            DragState {
                                                mode: DragMode::DragY,
                                                offset: if vertical {
                                                    scroll_area.scroll_top
                                                } else {
                                                    scroll_area.scroll_left
                                                },
                                            },
                                        );
                                    }
                                }),
                                On::<Pointer<Drag>>::run(move |world: &mut World| {
                                    let mut event = world
                                        .get_resource_mut::<ListenerInput<Pointer<Drag>>>()
                                        .unwrap();
                                    event.stop_propagation();
                                    let distance = event.distance;
                                    let ds = drag_state.get(world);
                                    if let Some(mut scroll_area) =
                                        world.get_mut::<ScrollArea>(id_scroll_area)
                                    {
                                        handle_thumb_drag(&mut scroll_area, &ds, distance);
                                    }
                                }),
                                On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                                    let mut event = world
                                        .get_resource_mut::<ListenerInput<Pointer<DragEnd>>>()
                                        .unwrap();
                                    event.stop_propagation();
                                    drag_state.set(
                                        world,
                                        DragState {
                                            mode: DragMode::None,
                                            offset: 0.,
                                        },
                                    );
                                }),
                                // On::<Pointer<PointerCancel>>::run(
                                //     move |mut ev: ListenerMut<Pointer<DragEnd>>, mut atoms: AtomStore| {
                                //         ev.stop_propagation();
                                //         handle_thumb_drag_end(&mut atoms, drag_state);
                                //     },
                                // ),
                            )
                        },
                        (),
                    ),
            )
    }
}

fn handle_thumb_drag(scroll_area: &mut ScrollArea, ds: &DragState, distance: Vec2) {
    if ds.mode == DragMode::DragY {
        let left = scroll_area.scroll_left;
        let top = if scroll_area.visible_size.y > 0. {
            ds.offset + distance.y * scroll_area.content_size.y / scroll_area.visible_size.y
        } else {
            0.
        };
        scroll_area.scroll_to(left, top);
    } else if ds.mode == DragMode::DragX {
        let top = scroll_area.scroll_top;
        let left = if scroll_area.visible_size.x > 0. {
            ds.offset + distance.x * scroll_area.content_size.x / scroll_area.visible_size.x
        } else {
            0.
        };
        scroll_area.scroll_to(left, top);
    };
}

fn handle_track_click(scroll_area: &mut ScrollArea, vertical: bool, position: Vec2, rect: Rect) {
    if vertical {
        let page_size = scroll_area.visible_size.y;
        if position.y >= rect.max.y {
            scroll_area.scroll_by(0., page_size);
        } else if position.y < rect.min.y {
            scroll_area.scroll_by(0., -page_size);
        }
    } else {
        let page_size = scroll_area.visible_size.x;
        if position.x >= rect.max.x {
            scroll_area.scroll_by(page_size, 0.);
        } else if position.x < rect.min.x {
            scroll_area.scroll_by(-page_size, 0.);
        }
    }
}

use bevy::{color::Luminance, prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::{colors, cursor::StyleBuilderCursor, hooks::UseIsHover};

/// The direction of the splitter. Represents the direction of the bar, not the items being split.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SplitterDirection {
    /// The splitter bar runs horizontally, and splits the items above and below it.
    Horizontal,

    /// The splitter bar runs vertically, and splits the items to the left and right of it.
    #[default]
    Vertical,
}

#[derive(Component, Clone)]
struct DragState {
    dragging: bool,
    offset: f32,
}

#[derive(Component)]
struct SplitterValue(f32);

fn style_vsplitter(ss: &mut StyleBuilder) {
    ss.align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .gap(8)
        .width(9)
        .background_color(colors::U1)
        .cursor(CursorIcon::ColResize);
}

// The decorative handle inside the splitter.
fn style_vsplitter_inner(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .width(3)
        // .pointer_events(PointerEvents::None)
        .height(ui::Val::Percent(20.));
}

fn style_hsplitter(ss: &mut StyleBuilder) {
    ss.align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .gap(8)
        .height(9)
        .background_color(colors::U2)
        .cursor(CursorIcon::RowResize);
}

// The decorative handle inside the splitter.
fn style_hsplitter_inner(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .height(3)
        // .pointer_events(PointerEvents::None)
        .width(ui::Val::Percent(20.));
}

/// Splitter bar which can be dragged
#[derive(Clone, PartialEq)]
pub struct Splitter {
    /// The current split value.
    pub value: f32,

    /// Whether the splitter bar runs horizontally or vertically.
    pub direction: SplitterDirection,

    /// Callback involved with the new split value.
    pub on_change: Option<Callback<f32>>,
}

impl Splitter {
    /// Create a new splitter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current split value.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the direction of the splitter.
    pub fn direction(mut self, direction: SplitterDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the callback to be invoked when the split value changes.
    pub fn on_change(mut self, on_change: Callback<f32>) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

impl Default for Splitter {
    fn default() -> Self {
        Self {
            value: 0.,
            direction: SplitterDirection::Vertical,
            on_change: None,
        }
    }
}

impl ViewTemplate for Splitter {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let hovering = cx.is_hovered(id);
        let on_change = self.on_change;
        let direction = self.direction;
        let style_splitter = match self.direction {
            SplitterDirection::Horizontal => style_hsplitter,
            SplitterDirection::Vertical => style_vsplitter,
        };
        let style_splitter_inner = match self.direction {
            SplitterDirection::Horizontal => style_hsplitter_inner,
            SplitterDirection::Vertical => style_vsplitter_inner,
        };

        // Ensure the entity has a DragState component before we render anything.
        let mut entt = cx.world_mut().entity_mut(id);
        if !entt.contains::<DragState>() {
            entt.insert(DragState {
                dragging: false,
                offset: 0.,
            });
        }

        Element::<NodeBundle>::for_entity(id)
            .named("Splitter")
            .style(style_splitter)
            .insert_dyn(SplitterValue, self.value)
            .insert_dyn(
                move |_| {
                    (
                        On::<Pointer<DragStart>>::run(move |world: &mut World| {
                            let current_offset = world.get::<SplitterValue>(id).unwrap().0;
                            let mut drag_state = world.get_mut::<DragState>(id).unwrap();
                            drag_state.dragging = true;
                            drag_state.offset = current_offset;
                        }),
                        On::<Pointer<DragEnd>>::listener_component_mut::<DragState>(
                            move |_, drag_state| {
                                drag_state.dragging = false;
                            },
                        ),
                        On::<Pointer<Drag>>::run({
                            move |world: &mut World| {
                                let event = world
                                    .get_resource::<ListenerInput<Pointer<Drag>>>()
                                    .unwrap();
                                let ev = event.distance;
                                let drag_state = world.get_mut::<DragState>(id).unwrap().clone();
                                if let Some(on_change) = on_change {
                                    if drag_state.dragging {
                                        match direction {
                                            SplitterDirection::Horizontal => {
                                                world.run_callback(
                                                    on_change,
                                                    drag_state.offset - ev.y,
                                                );
                                            }
                                            SplitterDirection::Vertical => {
                                                world.run_callback(
                                                    on_change,
                                                    ev.x + drag_state.offset,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }),
                        On::<Pointer<PointerCancel>>::listener_component_mut::<DragState>(
                            move |_, drag_state| {
                                drag_state.dragging = false;
                            },
                        ),
                    )
                },
                (),
            )
            .children(
                Element::<NodeBundle>::new()
                    .style(style_splitter_inner)
                    .style_dyn(
                        move |(is_hovering, dragging), sb| {
                            // Color change on hover / drag
                            let color = match (dragging, is_hovering) {
                                (true, _) => colors::U3.lighter(0.05),
                                (false, true) => colors::U3.lighter(0.02),
                                (false, false) => colors::U3,
                            };
                            sb.background_color(color);
                        },
                        (
                            hovering,
                            cx.use_component::<DragState>(id).unwrap().dragging,
                        ),
                    ),
            )
    }
}

use super::IconButton;
use crate::{colors, cursor::StyleBuilderCursor, hooks::UseElementRect, RoundedCorners};
use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use std::ops::RangeInclusive;

#[derive(Clone, PartialEq, Default, Copy)]
enum DragType {
    #[default]
    None = 0,
    Dragging,
}

#[derive(Clone, PartialEq, Default, Copy, Component)]
struct DragState {
    dragging: DragType,
    offset: f32,
    was_dragged: bool,
}

fn style_spinbox(ss: &mut StyleBuilder) {
    ss.min_width(24)
        .height(20)
        .background_color(colors::U1)
        .border_radius(5);
}

fn style_overlay(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .position(ui::PositionType::Absolute)
        .left(0)
        .top(0)
        .bottom(0)
        .right(0)
        .cursor(CursorIcon::ColResize);
}

fn style_spinbox_label(ss: &mut StyleBuilder) {
    ss.flex_grow(1.)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexEnd)
        .height(ui::Val::Percent(100.))
        .font("obsidian_ui://fonts/Open_Sans/static/OpenSans-Medium.ttf")
        .font_size(16)
        .overflow(ui::OverflowAxis::Hidden)
        .padding((3, 0))
        .color(colors::FOREGROUND);
}

fn style_spinbox_button(ss: &mut StyleBuilder) {
    ss.height(20.).padding(0).max_width(12).flex_grow(0.2);
}

/// Component used to hold the spinbox params so that they can be accessed by the callbacks
/// without capturing.
#[derive(Component, Copy, Clone)]
struct SpinBoxState {
    value: f32,
    min: f32,
    max: f32,
    precision: usize,
    step: f32,
}

/// A numeric spinbox. This is a widget that allows the user to input a number by typing, using
/// arrow buttons, or dragging. It is preferred over a slider in two cases:
/// * The range of values is large or unbounded, making it difficult to select a specific value
///   with a slider.
/// * There is limited horizontal space available.
#[derive(Clone, PartialEq)]
pub struct SpinBox {
    /// Current slider value.
    pub value: f32,

    /// Minimum slider value.
    pub min: f32,

    /// Maximum slider value.
    pub max: f32,

    /// Number of decimal places to round to (0 = integer).
    pub precision: usize,

    /// Amount to increment when using arrow buttons.
    pub step: f32,

    /// Whether the slider is disabled.
    pub disabled: bool,

    /// Signal which returns the value formatted as a string. It `None`, then a default
    /// formatter will be used.
    pub formatted_value: Option<String>,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<Callback<f32>>,
}

impl SpinBox {
    /// Create a new spinbox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current spinbox value.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the minimum spinbox value.
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum spinbox value.
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Set the minimum and maximum spinbox values from a range.
    pub fn range(mut self, range: RangeInclusive<f32>) -> Self {
        self.min = *range.start();
        self.max = *range.end();
        self
    }

    /// Set the number of decimal places to round to (0 = integer).
    pub fn precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Set the amount to increment when using arrow buttons.
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set whether the spinbox is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the signal which returns the value formatted as a string. If `None`, then a default
    /// formatter will be used.
    pub fn formatted_value(mut self, formatted_value: String) -> Self {
        self.formatted_value = Some(formatted_value);
        self
    }

    /// Set the style handle for the spinbox root element.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when value changes.
    pub fn on_change(mut self, on_change: Callback<f32>) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

impl Default for SpinBox {
    fn default() -> Self {
        Self {
            value: 0.,
            min: f32::MIN,
            max: f32::MAX,
            precision: 0,
            step: 1.,
            disabled: false,
            formatted_value: None,
            style: StyleHandle::default(),
            on_change: None,
        }
    }
}

impl ViewTemplate for SpinBox {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let spinbox_id = cx.create_entity();
        let rect = cx.use_element_rect(spinbox_id);
        let show_buttons = rect.width() >= 48.;
        let on_change = self.on_change;

        let dec_disabled = self.value <= self.min;
        let dec_click = cx.create_callback(move |world: &mut World| {
            let entt = world.entity(spinbox_id);
            let state = entt.get::<SpinBoxState>().unwrap();
            let next_value = state.value - state.step;
            if let Some(on_change) = on_change {
                world.run_callback(on_change, next_value.clamp(state.min, state.max));
            }
        });
        let inc_disabled = self.value >= self.max;
        let inc_click = cx.create_callback(move |world: &mut World| {
            let entt = world.entity(spinbox_id);
            let state = entt.get::<SpinBoxState>().unwrap();
            let next_value = state.value + state.step;
            if let Some(on_change) = on_change {
                world.run_callback(on_change, next_value.clamp(state.min, state.max));
            }
        });

        // Ensure DragState component exists before rendering.
        let mut entt = cx.world_mut().entity_mut(spinbox_id);
        if !entt.contains::<DragState>() {
            entt.insert(DragState {
                dragging: DragType::None,
                was_dragged: false,
                offset: 0.,
            });
        }

        Element::<NodeBundle>::for_entity(spinbox_id)
            .style((style_spinbox, self.style.clone()))
            .insert_dyn(
                |(value, min, max, precision, step)| SpinBoxState {
                    value,
                    min,
                    max,
                    precision,
                    step,
                },
                (self.value, self.min, self.max, self.precision, self.step),
            )
            .children((Element::<NodeBundle>::new()
                .named("SpinBox")
                .style(style_overlay)
                .children((
                    Cond::new(
                        show_buttons,
                        IconButton::new("obsidian_ui://icons/chevron_left.png")
                            .corners(RoundedCorners::Left)
                            .style(style_spinbox_button)
                            .minimal(true)
                            .disabled(dec_disabled)
                            .on_click(dec_click),
                        (),
                    ),
                    Element::<NodeBundle>::new()
                        .style(style_spinbox_label)
                        .insert_dyn(
                            move |_| {
                                (
                                    On::<Pointer<DragStart>>::run(move |world: &mut World| {
                                        // Save initial value to use as drag offset.
                                        let mut event = world
                                            .get_resource_mut::<ListenerInput<Pointer<DragStart>>>()
                                            .unwrap();
                                        event.stop_propagation();
                                        let mut entt = world.entity_mut(spinbox_id);
                                        let value = entt.get::<SpinBoxState>().unwrap().value;
                                        entt.insert(DragState {
                                            dragging: DragType::Dragging,
                                            offset: value,
                                            was_dragged: false,
                                        });
                                    }),
                                    On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                                        let entt = world.entity(spinbox_id);
                                        let ds = entt.get::<DragState>().unwrap();
                                        if ds.dragging == DragType::Dragging {
                                            if !ds.was_dragged {
                                                // We want to know if it was a click or a drag.
                                                // This will let us display a text input field later.
                                                // Once we have text input fields.
                                                println!("was not dragged");
                                            }
                                            let mut entt = world.entity_mut(spinbox_id);
                                            let state = entt.get::<SpinBoxState>().unwrap();
                                            entt.insert(DragState {
                                                dragging: DragType::None,
                                                offset: state.value,
                                                was_dragged: false,
                                            });
                                        }
                                    }),
                                    On::<Pointer<Drag>>::run(move |world: &mut World| {
                                        let entt = world.entity(spinbox_id);
                                        let ds = *entt.get::<DragState>().unwrap();
                                        if ds.dragging == DragType::Dragging {
                                            let event = world
                                                .get_resource::<ListenerInput<Pointer<Drag>>>()
                                                .unwrap();
                                            let delta = (event.distance.x - event.distance.y) * 0.1;
                                            let mut entt = world.entity_mut(spinbox_id);
                                            let state = entt.get::<SpinBoxState>().unwrap();
                                            // Rate of change increases with drag distance
                                            let new_value = ds.offset
                                                + (delta.abs().powf(1.3)
                                                    * delta.signum()
                                                    * state.step);
                                            let rounding = f32::powi(10., state.precision as i32);
                                            let value = state.value;
                                            let min = state.min;
                                            let max = state.max;
                                            let new_value =
                                                (new_value * rounding).round() / rounding;
                                            if value != new_value {
                                                if !ds.was_dragged {
                                                    entt.insert(DragState {
                                                        was_dragged: true,
                                                        ..ds
                                                    });
                                                }
                                                if let Some(on_change) = on_change {
                                                    world.run_callback(
                                                        on_change,
                                                        new_value.clamp(min, max),
                                                    );
                                                }
                                            }
                                        }
                                    }),
                                )
                            },
                            (self.min, self.max),
                        )
                        .children(match self.formatted_value {
                            Some(ref formatted_value) => formatted_value.clone(),
                            None => format!("{:.*}", self.precision, self.value),
                        }),
                    Cond::new(
                        show_buttons,
                        IconButton::new("obsidian_ui://icons/chevron_right.png")
                            .corners(RoundedCorners::Right)
                            .minimal(true)
                            .style(style_spinbox_button)
                            .disabled(inc_disabled)
                            .on_click(inc_click),
                        (),
                    ),
                )),))
    }
}

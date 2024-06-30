use std::ops::RangeInclusive;

use bevy::{color::LinearRgba, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::{
    colors, cursor::StyleBuilderCursor, hooks::UseElementRect, materials::SliderRectMaterial,
    RoundedCorners,
};

use super::{IconButton, Spacer};

#[derive(Clone, PartialEq, Default, Copy)]
enum DragType {
    #[default]
    None = 0,
    Dragging,
}

#[derive(Component, Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: DragType,
    offset: f32,
    was_dragged: bool,
}

fn style_slider(ss: &mut StyleBuilder) {
    ss.min_width(64).height(20);
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

fn style_label(ss: &mut StyleBuilder) {
    ss.flex_grow(1.)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .height(ui::Val::Percent(100.))
        .font("embedded://quill_obsidian/assets/fonts/Open_Sans/static/OpenSans-Medium.ttf")
        .font_size(16)
        .padding((6, 0))
        .color(colors::FOREGROUND);
}

fn style_slider_button(ss: &mut StyleBuilder) {
    ss.height(20.).padding(0).max_width(12).flex_grow(0.2);
}

/// Component used to hold the slider params so that they can be accessed by the callbacks
/// without capturing.
#[derive(Component, Copy, Clone)]
struct SliderState {
    value: f32,
    min: f32,
    max: f32,
    precision: usize,
    step: f32,
}

/// Horizontal slider widget
#[derive(Clone, PartialEq)]
pub struct Slider {
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

    /// Optional label to be displayed inside the slider.
    pub label: Option<String>,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<Callback<f32>>,
}

impl Slider {
    /// Create a new slider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current slider value.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the minimum slider value.
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum slider value.
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

    /// Set whether the slider is disabled.
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

    /// Set the optional label to be displayed inside the slider.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the style handle for the slider root element.
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

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: 0.,
            min: 0.,
            max: 1.,
            precision: 0,
            step: 1.,
            disabled: false,
            formatted_value: None,
            style: StyleHandle::default(),
            label: None,
            on_change: None,
        }
    }
}

impl ViewTemplate for Slider {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let slider_id = cx.create_entity();
        // let hovering = cx.is_hovered(slider_id);
        let rect = cx.use_element_rect(slider_id);
        let show_buttons = rect.width() >= 70.;

        let on_change = self.on_change;

        let dec_disabled = self.value <= self.min;
        let dec_click = cx.create_callback(move |world: &mut World| {
            let entt = world.entity(slider_id);
            let state = entt.get::<SliderState>().unwrap();
            let next_value = state.value - state.step;
            if let Some(on_change) = on_change {
                world.run_callback(on_change, next_value.clamp(state.min, state.max));
            }
        });
        let inc_disabled = self.value >= self.max;
        let inc_click = cx.create_callback(move |world: &mut World| {
            let entt = world.entity(slider_id);
            let state = entt.get::<SliderState>().unwrap();
            let next_value = state.value + state.step;
            if let Some(on_change) = on_change {
                world.run_callback(on_change, next_value.clamp(state.min, state.max));
            }
        });

        // Wrap material creation in a memo, we only want to create the material once.
        let material = cx.create_memo(
            |world, _| {
                let mut ui_materials = world
                    .get_resource_mut::<Assets<SliderRectMaterial>>()
                    .unwrap();
                ui_materials.add(SliderRectMaterial {
                    color_lo: LinearRgba::from(colors::U1).to_vec4(),
                    color_hi: LinearRgba::from(colors::U3).to_vec4(),
                    value: 0.5,
                    radius: RoundedCorners::All.to_vec(4.),
                })
            },
            (),
        );

        // Ensure DragState component exists before rendering.
        let mut entt = cx.world_mut().entity_mut(slider_id);
        if !entt.contains::<DragState>() {
            entt.insert(DragState {
                dragging: DragType::None,
                was_dragged: false,
                offset: 0.,
            });
        }

        Element::<MaterialNodeBundle<SliderRectMaterial>>::for_entity(slider_id)
            .style((style_slider, self.style.clone()))
            .insert(material.clone())
            .insert_dyn(
                |(value, min, max, precision, step)| SliderState {
                    value,
                    min,
                    max,
                    precision,
                    step,
                },
                (self.value, self.min, self.max, self.precision, self.step),
            )
            .insert_dyn(
                move |_| {
                    (
                        On::<Pointer<DragStart>>::run(move |world: &mut World| {
                            // Save initial value to use as drag offset.
                            let mut event = world
                                .get_resource_mut::<ListenerInput<Pointer<DragStart>>>()
                                .unwrap();
                            event.stop_propagation();
                            let mut entt = world.entity_mut(slider_id);
                            let value = entt.get::<SliderState>().unwrap().value;
                            entt.insert(DragState {
                                dragging: DragType::Dragging,
                                offset: value,
                                was_dragged: false,
                            });
                        }),
                        On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                            let entt = world.entity(slider_id);
                            let ds = entt.get::<DragState>().unwrap();
                            if ds.dragging == DragType::Dragging {
                                if !ds.was_dragged {
                                    // We want to know if it was a click or a drag.
                                    // This will let us display a text input field later.
                                    // Once we have text input fields.
                                    println!("was not dragged");
                                }
                                let mut entt = world.entity_mut(slider_id);
                                let state = entt.get::<SliderState>().unwrap();
                                entt.insert(DragState {
                                    dragging: DragType::None,
                                    offset: state.value,
                                    was_dragged: false,
                                });
                            }
                        }),
                        On::<Pointer<Drag>>::run(move |world: &mut World| {
                            let entt = world.entity(slider_id);
                            let ds = *entt.get::<DragState>().unwrap();
                            if ds.dragging == DragType::Dragging {
                                let event = world
                                    .get_resource::<ListenerInput<Pointer<Drag>>>()
                                    .unwrap();
                                let delta = event.distance.x;
                                let mut entt = world.entity_mut(slider_id);
                                let node = entt.get::<Node>();
                                let transform = entt.get::<GlobalTransform>();
                                if let (Some(node), Some(transform)) = (node, transform) {
                                    let state = *entt.get::<SliderState>().unwrap();
                                    // Measure node width and slider value.
                                    let slider_width = node.logical_rect(transform).width();
                                    let range = state.max - state.min;
                                    let new_value = if range > 0. {
                                        ds.offset + (delta * range) / slider_width
                                    } else {
                                        state.min + range * 0.5
                                    };
                                    let rounding = f32::powi(10., state.precision as i32);
                                    let new_value = (new_value * rounding).round() / rounding;
                                    if state.value != new_value {
                                        if !ds.was_dragged {
                                            entt.insert(DragState {
                                                was_dragged: true,
                                                ..ds
                                            });
                                        }
                                        if let Some(on_change) = on_change {
                                            world.run_callback(
                                                on_change,
                                                new_value.clamp(state.min, state.max),
                                            );
                                        }
                                    }
                                }
                            }
                        }),
                    )
                },
                (),
            )
            .effect(
                move |cx, _ent, (min, max, value, material)| {
                    let pos = if max > min {
                        (value - min) / (max - min)
                    } else {
                        0.
                    };

                    let mut ui_materials = cx
                        .world_mut()
                        .get_resource_mut::<Assets<SliderRectMaterial>>()
                        .unwrap();
                    let material = ui_materials.get_mut(material.id()).unwrap();
                    material.value = pos;
                },
                (self.min, self.max, self.value, material.clone()),
            )
            .children((Element::<NodeBundle>::new()
                .named("Slider")
                .style(style_overlay)
                .children((
                    Cond::new(
                        show_buttons,
                        IconButton::new("embedded://quill_obsidian/assets/icons/chevron_left.png")
                            .corners(RoundedCorners::Left)
                            .style(style_slider_button)
                            .minimal(true)
                            .disabled(dec_disabled)
                            .on_click(dec_click),
                        (),
                    ),
                    Element::<NodeBundle>::new().style(style_label).children((
                        Cond::new(
                            self.label.is_some(),
                            (self.label.clone().unwrap_or_default(), Spacer),
                            (),
                        ),
                        match self.formatted_value {
                            Some(ref formatted_value) => formatted_value.clone(),
                            None => format!("{:.*}", self.precision, self.value),
                        },
                    )),
                    Cond::new(
                        show_buttons,
                        IconButton::new("embedded://quill_obsidian/assets/icons/chevron_right.png")
                            .corners(RoundedCorners::Right)
                            .minimal(true)
                            .style(style_slider_button)
                            .disabled(inc_disabled)
                            .on_click(inc_click),
                        (),
                    ),
                )),))
    }
}

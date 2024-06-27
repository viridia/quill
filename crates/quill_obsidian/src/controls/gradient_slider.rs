use std::ops::RangeInclusive;

use bevy::{color::Srgba, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::materials::GradientRectMaterial;

const THUMB_WIDTH: f32 = 12.;

/// Component used to hold the slider params so that they can be accessed by the callbacks
/// without capturing.
#[derive(Component, Copy, Clone)]
struct SliderState {
    value: f32,
    min: f32,
    max: f32,
    precision: usize,
}

/// Struct representing a sequence of color stops, evenly spaced. Up to 8 stops are supported.
#[derive(Debug, Copy, Clone)]
pub struct ColorGradient {
    /// Number of color stops.
    pub num_colors: usize,

    /// Array of color stops.
    pub colors: [Srgba; 8],
}

impl ColorGradient {
    /// Construct a new color gradient from an array of colors.
    pub fn new(colors: &[Srgba]) -> Self {
        assert!(colors.len() <= 8);
        let mut result = Self {
            num_colors: colors.len(),
            colors: [Srgba::default(); 8],
        };
        for (i, color) in colors.iter().enumerate() {
            result.colors[i] = *color;
        }
        result
    }

    /// Return the first color in the gradient, if any.
    pub fn first(&self) -> Option<Srgba> {
        if self.num_colors > 0 {
            Some(self.colors[0])
        } else {
            None
        }
    }

    /// Return the last color in the gradient, if any.
    pub fn last(&self) -> Option<Srgba> {
        if self.num_colors > 0 {
            Some(self.colors[self.num_colors - 1])
        } else {
            None
        }
    }

    /// Return the number of color stops in the gradient.
    pub fn len(&self) -> usize {
        self.num_colors
    }

    /// Check if the gradient is empty.
    pub fn is_empty(&self) -> bool {
        self.num_colors == 0
    }
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self {
            num_colors: 1,
            colors: [Srgba::BLACK; 8],
        }
    }
}

impl PartialEq for ColorGradient {
    fn eq(&self, other: &Self) -> bool {
        self.num_colors == other.num_colors
            && self.colors[0..self.num_colors] == other.colors[0..other.num_colors]
    }
}

#[derive(Component, Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: bool,
    offset: f32,
}

fn style_slider(ss: &mut StyleBuilder) {
    ss.min_width(32)
        .height(14)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Stretch);
}

fn style_gradient(ss: &mut StyleBuilder) {
    ss.flex_grow(1.);
}

fn style_track(ss: &mut StyleBuilder) {
    ss.position(ui::PositionType::Absolute)
        .top(1)
        .bottom(1)
        .left(1)
        .right(THUMB_WIDTH + 1.);
}

fn style_thumb(ss: &mut StyleBuilder) {
    ss.background_image("obsidian_ui://textures/gradient_thumb.png")
        .position(ui::PositionType::Absolute)
        .top(0)
        .bottom(0)
        .width(THUMB_WIDTH);
}

/// Horizontal slider widget that displays a gradient bar and a draggable button.
#[derive(Clone, PartialEq)]
pub struct GradientSlider {
    /// Gradient to display.
    pub gradient: ColorGradient,

    /// Current slider value.
    pub value: f32,

    /// Minimum slider value.
    pub min: f32,

    /// Maximum slider value.
    pub max: f32,

    /// Number of decimal places to round to (0 = integer).
    pub precision: usize,

    /// Whether the slider is disabled.
    pub disabled: bool,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<Callback<f32>>,
}

impl GradientSlider {
    /// Create a new gradient slider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the gradient to display.
    pub fn gradient(mut self, gradient: ColorGradient) -> Self {
        self.gradient = gradient;
        self
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

    /// Set whether the slider is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the style handle for the slider root element.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when the value changes.
    pub fn on_change(mut self, on_change: Callback<f32>) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

impl Default for GradientSlider {
    fn default() -> Self {
        Self {
            gradient: ColorGradient::default(),
            value: 0.,
            min: 0.,
            max: 1.,
            precision: 0,
            disabled: false,
            style: StyleHandle::default(),
            on_change: None,
        }
    }
}

impl ViewTemplate for GradientSlider {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let slider_id = cx.create_entity();
        let on_change = self.on_change;

        let color_stops = cx.create_memo(
            move |_, g| {
                let mut result: [Vec4; 8] = [Vec4::default(); 8];
                let num_color_stops = g.len();
                for (i, color) in g.colors[0..num_color_stops].iter().enumerate() {
                    // Note that we do *not* convert to linear here, because interpolating
                    // linear looks bad. That gets done in the shader.
                    result[i] = Vec4::new(color.red, color.green, color.blue, color.alpha);
                }
                (g.len(), result)
            },
            self.gradient,
        );

        let gradient_material = cx.create_memo(
            |world, _| {
                let mut gradient_material_assets = world
                    .get_resource_mut::<Assets<GradientRectMaterial>>()
                    .unwrap();
                gradient_material_assets.add(GradientRectMaterial {
                    color_stops: [Srgba::default().to_vec4(); 8],
                    num_color_stops: 2,
                    cap_size: THUMB_WIDTH * 0.5,
                })
            },
            (),
        );

        // Effect to update the material handle.
        cx.create_effect(
            move |world, (material, color_stops)| {
                let (num_color_stops, color_stops) = color_stops;
                let mut ui_materials = world
                    .get_resource_mut::<Assets<GradientRectMaterial>>()
                    .unwrap();
                let material = ui_materials.get_mut(material.id()).unwrap();
                material.num_color_stops = num_color_stops as i32;
                material.color_stops = color_stops;
            },
            (gradient_material.clone(), color_stops),
        );

        // Ensure DragState component exists before rendering.
        let mut entt = cx.world_mut().entity_mut(slider_id);
        if !entt.contains::<DragState>() {
            entt.insert(DragState {
                dragging: false,
                offset: 0.,
            });
        }

        Element::<NodeBundle>::for_entity(slider_id)
            .named("GradientSlider")
            .style((style_slider, self.style.clone()))
            .insert_dyn(
                |(value, min, max, precision)| SliderState {
                    value,
                    min,
                    max,
                    precision,
                },
                (self.value, self.min, self.max, self.precision),
            )
            .insert_dyn(
                move |_| {
                    (
                        On::<Pointer<Down>>::run(move |world: &mut World| {
                            let mut event = world
                                .get_resource_mut::<ListenerInput<Pointer<Down>>>()
                                .unwrap();
                            event.stop_propagation();
                            let hit_x = event.pointer_location.position.x;
                            let ent = world.entity(slider_id);
                            let node = ent.get::<Node>();
                            let transform = ent.get::<GlobalTransform>();
                            if let (Some(node), Some(transform)) = (node, transform) {
                                // If not clicking on thumb, then snap thumb to new location.
                                let rect = node.logical_rect(transform);
                                let slider_width = rect.width() - THUMB_WIDTH;
                                let state = *ent.get::<SliderState>().unwrap();
                                let range = state.max - state.min;
                                let pointer_pos = hit_x - rect.min.x - THUMB_WIDTH / 2.;
                                let thumb_pos = state.value - state.min * slider_width / range
                                    + THUMB_WIDTH / 2.;
                                if range > 0. && (pointer_pos - thumb_pos).abs() >= THUMB_WIDTH / 2.
                                {
                                    let new_value =
                                        state.min + (pointer_pos * range) / slider_width;
                                    if let Some(on_change) = on_change {
                                        world.run_callback(
                                            on_change,
                                            new_value.clamp(state.min, state.max),
                                        );
                                    }
                                };
                            }
                        }),
                        On::<Pointer<DragStart>>::run(move |world: &mut World| {
                            // Save initial value to use as drag offset.
                            let mut event = world
                                .get_resource_mut::<ListenerInput<Pointer<DragStart>>>()
                                .unwrap();
                            event.stop_propagation();
                            let mut entt = world.entity_mut(slider_id);
                            let value = entt.get::<SliderState>().unwrap().value;
                            entt.insert(DragState {
                                dragging: true,
                                offset: value,
                            });
                        }),
                        On::<Pointer<DragEnd>>::listener_component_mut::<DragState>(
                            |_, drag_state| {
                                drag_state.dragging = false;
                            },
                        ),
                        On::<Pointer<Drag>>::run(move |world: &mut World| {
                            let ent = world.entity(slider_id);
                            let ds = *ent.get::<DragState>().unwrap();
                            if ds.dragging {
                                let event = world
                                    .get_resource::<ListenerInput<Pointer<Drag>>>()
                                    .unwrap();
                                let ent = world.entity(slider_id);
                                let node = ent.get::<Node>();
                                let transform = ent.get::<GlobalTransform>();
                                if let (Some(node), Some(transform)) = (node, transform) {
                                    // Measure node width and slider value.
                                    let state = *ent.get::<SliderState>().unwrap();
                                    let slider_width = node.logical_rect(transform).width();
                                    let range = state.max - state.min;
                                    let new_value = if range > 0. {
                                        ds.offset + (event.distance.x * range) / slider_width
                                    } else {
                                        state.min + range * 0.5
                                    };
                                    let rounding = f32::powi(10., state.precision as i32);
                                    let new_value = (new_value * rounding).round() / rounding;
                                    if let Some(on_change) = on_change {
                                        world.run_callback(
                                            on_change,
                                            new_value.clamp(state.min, state.max),
                                        );
                                    }
                                }
                            }
                        }),
                    )
                },
                (),
            )
            .children((
                Element::<MaterialNodeBundle<GradientRectMaterial>>::new()
                    .insert(gradient_material.clone())
                    .style(style_gradient),
                Element::<NodeBundle>::new()
                    .named("GradientSlider::Track")
                    .style(style_track)
                    .children(
                        Element::<NodeBundle>::new()
                            .named("GradientSlider::Thumb")
                            .style(style_thumb)
                            .style_dyn(
                                move |(min, max, value), sb| {
                                    let percent = if max > min {
                                        ((value - min) / (max - min)).clamp(0., 1.)
                                    } else {
                                        0.
                                    };

                                    sb.left(ui::Val::Percent(percent * 100.));
                                },
                                (self.min, self.max, self.value),
                            ),
                    ),
            ))
    }
}

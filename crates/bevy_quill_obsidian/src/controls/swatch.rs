use bevy::{color::Srgba, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill_core::*;

use crate::materials::SwatchRectMaterial;

use crate::colors;

fn style_swatch(ss: &mut StyleBuilder) {
    ss.min_width(8)
        .min_height(8)
        .display(ui::Display::Flex)
        .color(colors::FOREGROUND)
        .border_radius(3)
        .padding(2);
}

fn style_selection(ss: &mut StyleBuilder) {
    ss.border(1)
        .border_color(colors::U1)
        .outline_color(colors::FOREGROUND)
        .outline_width(2)
        .outline_offset(0)
        .align_self(ui::AlignSelf::Stretch)
        .justify_self(ui::JustifySelf::Stretch)
        .flex_grow(1.);
}

/// Color swatch widget. This displays a solid color, and can also display a checkerboard
/// pattern behind the color if it has an alpha of less than 1.
#[derive(Clone, PartialEq, Default)]
pub struct Swatch {
    /// Color to display
    pub color: Srgba,

    /// For swatch grids, whether this swatch is selected.
    pub selected: bool,

    /// Additional styles to be applied to the widget.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Option<Callback<Srgba>>,
}

impl Swatch {
    /// Create a new swatch.
    pub fn new(color: impl Into<Srgba>) -> Self {
        Self::default().color(color.into())
    }

    /// Set the color to display.
    pub fn color(mut self, color: impl Into<Srgba>) -> Self {
        self.color = color.into();
        self
    }

    /// Set additional styles to be applied to the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when clicked.
    pub fn on_click(mut self, on_click: Callback<Srgba>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    /// Set whether the swatch should be rendered in a 'selected' state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ViewTemplate for Swatch {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let color = self.color;
        let selected = self.selected;

        // Wrap material creation in a memo, we only want to create the material once.
        let material = cx.create_memo(
            |world, _| {
                let mut ui_materials = world
                    .get_resource_mut::<Assets<SwatchRectMaterial>>()
                    .unwrap();
                ui_materials.add(SwatchRectMaterial {
                    color: LinearRgba::from(colors::U1).to_vec4(),
                    border_radius: Vec4::splat(0.),
                })
            },
            (),
        );

        // Update material color
        cx.create_effect(
            |world, (material, color)| {
                let mut ui_materials = world
                    .get_resource_mut::<Assets<SwatchRectMaterial>>()
                    .unwrap();
                let material = ui_materials.get_mut(material.id()).unwrap();
                material.color = color.to_vec4();
            },
            (material.clone(), self.color),
        );

        Element::<MaterialNodeBundle<SwatchRectMaterial>>::new()
            .named("Swatch")
            .style((style_swatch, self.style.clone()))
            .insert_dyn(|m| m, material.clone())
            .insert_dyn(
                |(color, on_click)| {
                    On::<Pointer<Click>>::run(move |world: &mut World| {
                        if let Some(on_click) = on_click {
                            world.run_callback(on_click, color);
                        }
                    })
                },
                (color, self.on_click),
            )
            .effect(
                move |cx, ent, _| {
                    let radius = cx.use_component::<BorderRadius>(ent);
                    if let Some(radius) = radius {
                        let radius = Vec4::from_array(resolve_border_radius(radius));
                        let mut ui_materials = cx
                            .world_mut()
                            .get_resource_mut::<Assets<SwatchRectMaterial>>()
                            .unwrap();
                        let material = ui_materials.get_mut(material.id()).unwrap();
                        material.border_radius = radius;
                    }
                },
                (),
            )
            .children(Cond::new(
                selected,
                Element::<NodeBundle>::new().style(style_selection),
                (),
            ))
    }
}

// For now we only support pixel units.
fn resolve_border_radius(&values: &BorderRadius) -> [f32; 4] {
    [
        values.top_left,
        values.top_right,
        values.bottom_right,
        values.bottom_left,
    ]
    .map(|value| match value {
        Val::Px(px) => px,
        _ => 0.,
    })
}

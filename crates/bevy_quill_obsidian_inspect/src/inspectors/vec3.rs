use std::sync::Arc;

use bevy::{
    math::Vec3,
    prelude::{In, World},
    reflect::Reflect,
    ui::{self, node_bundles::NodeBundle},
};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::controls::SpinBox;

use crate::{templates::field_label::FieldLabel, Inspectable, Precision, Step};

#[derive(Clone)]
pub struct Vec3FieldInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for Vec3FieldInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug)]
struct Vec3Attrs {
    precision: usize,
    step: f32,
}

fn style_spinbox_group(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::FlexStart)
        .column_gap(3);
}

fn style_spinbox(ss: &mut StyleBuilder) {
    ss.flex_grow(1.);
}

impl ViewTemplate for Vec3FieldInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let value = match field.reflect(cx) {
            Some(value) if value.is::<Vec3>() => *value.downcast_ref::<Vec3>().unwrap(),
            _ => Vec3::splat(0.),
        };
        let value_capture = cx.create_capture(value);

        let field = self.0.clone();
        let mut slider_params = Vec3Attrs {
            precision: 2,
            step: 0.1,
        };

        if let Some(attrs) = field.attributes {
            if let Some(precision) = attrs.get::<Precision>() {
                slider_params.precision = precision.0;
            }
            if let Some(step) = attrs.get::<Step<f32>>() {
                slider_params.step = step.0;
            } else {
                slider_params.step = 10.0f32.powi(-(slider_params.precision as i32));
            }
        }

        (
            FieldLabel {
                field: field.clone(),
            },
            // Don't need `Cond` here because condition is not reactive; reflection data
            // is constant.
            Element::<NodeBundle>::new()
                .style(style_spinbox_group)
                .children((
                    // "x",
                    SpinBox::new()
                        .style(style_spinbox)
                        .precision(slider_params.precision)
                        .step(slider_params.step)
                        .value(value.x)
                        .on_change(cx.create_callback({
                            let field = self.0.clone();
                            move |x: In<f32>, world: &mut World| {
                                let value = value_capture.get(world).with_x(*x);
                                field.update(world, &|reflect| {
                                    reflect.apply(value.as_reflect());
                                });
                            }
                        })),
                    // "y",
                    SpinBox::new()
                        .style(style_spinbox)
                        .precision(slider_params.precision)
                        .step(slider_params.step)
                        .value(value.y)
                        .on_change(cx.create_callback({
                            let field = self.0.clone();
                            move |y: In<f32>, world: &mut World| {
                                let value = value_capture.get(world).with_y(*y);
                                field.update(world, &|reflect| {
                                    reflect.apply(value.as_reflect());
                                });
                            }
                        })),
                    // "z",
                    SpinBox::new()
                        .style(style_spinbox)
                        .precision(slider_params.precision)
                        .step(slider_params.step)
                        .value(value.z)
                        .on_change(cx.create_callback({
                            let field = self.0.clone();
                            move |z: In<f32>, world: &mut World| {
                                let value = value_capture.get(world).with_z(*z);
                                field.update(world, &|reflect| {
                                    reflect.apply(value.as_reflect());
                                });
                            }
                        })),
                )),
        )
    }
}

use std::{ops::Range, sync::Arc};

use bevy::{
    prelude::{In, World},
    reflect::Reflect,
};
use bevy_quill::*;
use bevy_quill_obsidian::controls::{Slider, SpinBox};

use crate::{templates::field_label::FieldLabel, Inspectable, Precision, Step, ValueRange};

#[derive(Clone, Debug)]
struct F32Attrs {
    range: Option<Range<f32>>,
    precision: usize,
    step: f32,
}

#[derive(Clone)]
pub struct F32FieldInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for F32FieldInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for F32FieldInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let value = match field.reflect(cx) {
            Some(value) if value.is::<f32>() => *value.downcast_ref::<f32>().unwrap(),
            _ => 0.0,
        };

        let field = self.0.clone();
        let mut slider_params = F32Attrs {
            range: None,
            precision: 0,
            step: 1.0,
        };

        if let Some(attrs) = field.attributes {
            if let Some(range) = attrs.get::<ValueRange<f32>>() {
                slider_params.range = Some(range.0.clone());
                slider_params.precision =
                    (2. - (range.0.end - range.0.start).log10().ceil()).max(0.) as usize;
            }
            if let Some(precision) = attrs.get::<Precision>() {
                slider_params.precision = precision.0;
            }
            if let Some(step) = attrs.get::<Step<f32>>() {
                slider_params.step = step.0;
            } else {
                slider_params.step = 10.0f32.powi(-(slider_params.precision as i32));
            }
        }

        // let field = self.field.clone();
        (
            FieldLabel {
                field: field.clone(),
            },
            // Don't need `Cond` here because condition is not reactive; reflection data
            // is constant.
            match slider_params.range {
                Some(range) => Slider::new()
                    .min(range.start)
                    .max(range.end)
                    .precision(slider_params.precision)
                    .step(slider_params.step)
                    .value(value)
                    .on_change(
                        cx.create_callback(move |value: In<f32>, world: &mut World| {
                            field.update(world, &|reflect| {
                                reflect.apply(value.as_reflect());
                            });
                        }),
                    )
                    .into_view_child(),
                None => SpinBox::new()
                    .precision(slider_params.precision)
                    .step(slider_params.step)
                    .value(value)
                    .on_change(
                        cx.create_callback(move |value: In<f32>, world: &mut World| {
                            field.update(world, &|reflect| {
                                reflect.apply(value.as_reflect());
                            });
                        }),
                    )
                    .into_view_child(),
            },
        )
    }
}

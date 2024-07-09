use std::sync::Arc;

use crate::{
    inspectors::{
        bool::BooleanFieldInspector, color::SrgbaInspector, f32::F32FieldInspector,
        fallback::FallbackInspector, list::ListInspector, r#enum::EnumInspector,
        r#struct::NestedStruct, tuple_struct::NestedTupleStruct, vec3::Vec3FieldInspector,
    },
    templates::{field_label::FieldLabel, field_readonly_value::FieldReadonlyValue},
    Inspectable, InspectorFactory,
};
use bevy::reflect::ReflectRef;
use bevy_quill_core::*;

#[derive(Default)]
pub struct DefaultInspectorFactory;

impl InspectorFactory for DefaultInspectorFactory {
    fn create_inspector(&self, cx: &Cx, field: Arc<Inspectable>) -> Option<ViewChild> {
        let reflect = field.reflect(cx)?;
        match reflect.reflect_ref() {
            ReflectRef::Struct(s) => match s.reflect_type_path() {
                "bevy_color::srgba::Srgba" => Some(SrgbaInspector(field.clone()).into_view_child()),
                "glam::Vec3" => Some(Vec3FieldInspector(field.clone()).into_view_child()),
                _ => Some(NestedStruct(field.clone()).into_view_child()),
            },
            ReflectRef::TupleStruct(_) => Some(NestedTupleStruct(field.clone()).into_view_child()),
            ReflectRef::Tuple(_) => Some(
                (
                    FieldLabel {
                        field: field.clone(),
                    },
                    FieldReadonlyValue::new().children("Tuple:TODO"),
                )
                    .into_view_child(),
            ),
            ReflectRef::List(_) => Some(ListInspector(field.clone()).into_view_child()),
            ReflectRef::Array(_) => Some(
                (
                    FieldLabel {
                        field: field.clone(),
                    },
                    FieldReadonlyValue::new().children("Array:TODO"),
                )
                    .into_view_child(),
            ),
            ReflectRef::Map(_) => Some(
                (
                    FieldLabel {
                        field: field.clone(),
                    },
                    FieldReadonlyValue::new().children("Map:TODO"),
                )
                    .into_view_child(),
            ),
            ReflectRef::Enum(_) => Some(EnumInspector(field.clone()).into_view_child()),
            ReflectRef::Value(v) => match v.reflect_type_path() {
                "bool" => Some(BooleanFieldInspector(field.clone()).into_view_child()),
                "f32" => Some(F32FieldInspector(field.clone()).into_view_child()),
                _ => Some(FallbackInspector(field.clone()).into_view_child()),
            },
        }
    }
}

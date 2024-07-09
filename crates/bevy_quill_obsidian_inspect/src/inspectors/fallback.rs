use std::sync::Arc;

use bevy_quill_core::*;

use crate::{
    templates::{field_label::FieldLabel, field_readonly_value::FieldReadonlyValue},
    Inspectable,
};

/// Field editor for when no specific editor is available.
#[derive(Clone)]
pub struct FallbackInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for FallbackInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for FallbackInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let Some(reflect) = field.reflect(cx) else {
            return ().into_view_child();
        };
        (
            FieldLabel {
                field: self.0.clone(),
            },
            FieldReadonlyValue::new()
                .children(format!("Fallback: {}", reflect.reflect_type_path())),
        )
            .into_view_child()
    }
}

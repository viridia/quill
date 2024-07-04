use std::sync::Arc;

use bevy::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::controls::Checkbox;

use crate::{templates::field_label::FieldLabel, Inspectable};

#[derive(Clone)]
pub struct BooleanFieldInspector(pub(crate) Arc<Inspectable>);

impl PartialEq for BooleanFieldInspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewTemplate for BooleanFieldInspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let field = self.0.clone();
        let is_checked = match field.reflect(cx) {
            Some(value) if value.is::<bool>() => *value.downcast_ref::<bool>().unwrap(),
            _ => false,
        };

        let field = self.0.clone();
        (
            FieldLabel {
                field: field.clone(),
            },
            Checkbox {
                checked: is_checked,
                on_change: Some(
                    cx.create_callback(move |value: In<bool>, world: &mut World| {
                        field.set_value(world, value.as_reflect());
                    }),
                ),
                style: StyleHandle::new(|ss: &mut StyleBuilder| {
                    ss.justify_self(JustifySelf::Start);
                }),
                ..default()
            },
        )
    }
}

use std::sync::Arc;

use bevy::reflect::{ParsedPath, ReflectKind};
use bevy_quill::*;
use quill_obsidian::controls::Spacer;

use crate::{
    inspectors::{
        r#struct::{StructFieldList, StructInspectorHeaderControls},
        tuple_struct::TupleStructElements,
    },
    templates::inspector_panel::InspectorPanel,
    Inspectable, InspectableRoot,
};

#[derive(Clone)]
pub struct Inspector {
    // Reference to the entity being inspected
    target: Arc<dyn InspectableRoot>,
}

impl PartialEq for Inspector {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.target, &other.target)
    }
}

impl Inspector {
    pub fn new(target: Arc<dyn InspectableRoot>) -> Self {
        Self { target }
    }

    fn create_fields(&self, cx: &mut Cx, inspectable: Arc<Inspectable>) -> ViewChild {
        let access = inspectable.clone();
        let field_type = access.reflect(cx).unwrap().reflect_kind().to_owned();
        match field_type {
            ReflectKind::Struct => StructFieldList(inspectable.clone()).into_view_child(),
            ReflectKind::TupleStruct => TupleStructElements(inspectable.clone()).into_view_child(),
            ReflectKind::Tuple => todo!(),
            ReflectKind::List => todo!(),
            ReflectKind::Array => todo!(),
            ReflectKind::Map => todo!(),
            ReflectKind::Enum => todo!(),
            ReflectKind::Value => todo!(),
        }
    }
}

impl ViewTemplate for Inspector {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let path = ParsedPath(vec![]);
        let inspectable = Arc::new(Inspectable {
            root: self.target.clone(),
            name: self.target.name(cx).clone(),
            value_path: path.clone(),
            field_path: path,
            can_remove: true,
            attributes: None,
        });
        InspectorPanel::new()
            .title((
                self.target.name(cx),
                Spacer,
                StructInspectorHeaderControls {
                    target: inspectable.clone(),
                },
            ))
            .body(self.create_fields(cx, inspectable))
            .expanded(true)
    }
}

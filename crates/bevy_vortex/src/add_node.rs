use bevy::{
    math::IVec2,
    prelude::{AppTypeRegistry, World},
    reflect::std_traits::ReflectDefault,
};
use bevy_quill::prelude::*;
use quill_obsidian::prelude::*;

use crate::{
    catalog::SelectedCatalogEntry,
    graph::{GraphResource, UndoAction},
    operator::ReflectOperator,
};

/// Displays the list of available operators, by category.
#[derive(Clone, PartialEq)]
pub struct AddNodeButton;

impl ViewTemplate for AddNodeButton {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let on_add = cx.create_callback(|world: &mut World| {
            let selected = world
                .get_resource::<SelectedCatalogEntry>()
                .unwrap()
                .0
                .unwrap();
            let registry = world.get_resource::<AppTypeRegistry>().unwrap();
            let registry_lock = registry.read();
            if let Some(operator_type) = registry_lock.get_with_type_path(selected) {
                let rd = operator_type.data::<ReflectDefault>().unwrap();
                let mut value = rd.default();
                let reflect_operator = registry_lock
                    .get_type_data::<ReflectOperator>(operator_type.type_id())
                    .unwrap();
                let operator = reflect_operator.get(value.as_ref());
                println!("Gotcha");

                // let graph = world.get_resource_mut::<GraphResource>().unwrap();
                let mut action = UndoAction::new("Add Node");
                // graph.0.create_node(operator, IVec2::default(), &mut action);
                // graph.0.add_undo_action(action);
            }
        });
        let selected = cx.use_resource::<SelectedCatalogEntry>();
        Button::new()
            .disabled(selected.0.is_none())
            .on_click(on_add)
            .children("Add node")
    }
}

use bevy::{
    math::IVec2,
    prelude::{AppTypeRegistry, Commands, Res, ResMut},
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
        let on_add = cx.create_callback(
            |selected: Res<SelectedCatalogEntry>,
             registry: Res<AppTypeRegistry>,
             mut graph: ResMut<GraphResource>,
             mut commands: Commands| {
                let registry_lock = registry.read();
                if let Some(operator_type) = registry_lock.get_with_type_path(selected.0.unwrap()) {
                    let rd = operator_type.data::<ReflectDefault>().unwrap();
                    let value = rd.default();
                    let reflect_operator = registry_lock
                        .get_type_data::<ReflectOperator>(operator_type.type_id())
                        .unwrap();
                    let operator = reflect_operator.get_boxed(value).unwrap();
                    let mut action = UndoAction::new("Add Node");
                    graph
                        .0
                        .create_node(&mut commands, operator, IVec2::default(), &mut action);
                    // graph.0.add_undo_action(action);
                }
            },
        );
        let selected = cx.use_resource::<SelectedCatalogEntry>();
        Button::new()
            .disabled(selected.0.is_none())
            .on_click(on_add)
            .children("Add node")
    }
}

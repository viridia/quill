use bevy::{
    math::{IVec2, Vec2},
    prelude::{AppTypeRegistry, Commands, Query, Res, ResMut},
    reflect::std_traits::ReflectDefault,
};
use bevy_quill::prelude::*;
use quill_obsidian::{prelude::*, scrolling::ScrollArea};

use crate::{
    catalog::SelectedCatalogEntry,
    graph::{GraphResource, UndoAction},
    graph_view::GraphViewId,
    operator::ReflectOperator,
};

/// Displays the list of available operators, by category.
#[derive(Clone, PartialEq)]
pub struct AddNodeButton;

impl ViewTemplate for AddNodeButton {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph_view_id = cx.use_inherited_component::<GraphViewId>().unwrap().0;

        let on_add = cx.create_callback(
            move |selected: Res<SelectedCatalogEntry>,
                  registry: Res<AppTypeRegistry>,
                  scrollarea_query: Query<&ScrollArea>,
                  mut graph: ResMut<GraphResource>,
                  mut commands: Commands| {
                // Need access to scroll region center.
                let registry_lock = registry.read();
                if let Some(operator_type) = registry_lock.get_with_type_path(selected.0.unwrap()) {
                    let rd = operator_type.data::<ReflectDefault>().unwrap();
                    let value = rd.default();
                    let reflect_operator = registry_lock
                        .get_type_data::<ReflectOperator>(operator_type.type_id())
                        .unwrap();
                    let operator = reflect_operator.get_boxed(value).unwrap();
                    let mut action = UndoAction::new("Add Node");
                    // Find the center of the current graph view, based on scroll position and size.
                    let position = if let Ok(scrollarea) = scrollarea_query.get(graph_view_id) {
                        let pos = scrollarea.scroll_position();
                        let size = scrollarea.visible_size();
                        Vec2::new(pos.x + size.x * 0.5, pos.y + size.y * 0.5).as_ivec2()
                    } else {
                        IVec2::default()
                    };
                    graph
                        .0
                        .create_node(&mut commands, operator, position, &mut action);
                    graph.0.add_undo_action(action);
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

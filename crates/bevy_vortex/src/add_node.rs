use bevy::{
    log::warn,
    math::{IVec2, Vec2},
    prelude::*,
    reflect::std_traits::ReflectDefault,
};
use bevy_quill::prelude::*;
use quill_obsidian::{controls::Button, scrolling::ScrollArea};

use crate::{
    catalog::SelectedCatalogEntry,
    graph::{GraphNode, GraphResource, Selected, UndoAction},
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
            move |selection: Res<SelectedCatalogEntry>,
                  registry: Res<AppTypeRegistry>,
                  scrollarea_query: Query<&ScrollArea>,
                  mut graph: ResMut<GraphResource>,
                  mut commands: Commands| {
                let registry_lock = registry.read();
                let Some(selected_operator_path) = selection.0 else {
                    warn!("No selection");
                    return;
                };
                if let Some(operator_type) =
                    registry_lock.get_with_type_path(selected_operator_path)
                {
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

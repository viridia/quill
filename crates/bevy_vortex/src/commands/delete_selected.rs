use bevy::{
    ecs::{system::SystemState, world::Command},
    prelude::*,
    utils::HashSet,
};

use crate::graph::*;

pub(crate) struct DeleteSelectedCmd;

impl Command for DeleteSelectedCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(
            Query<(Entity, &mut Connection)>,
            Query<(Entity, &GraphNode, &NodeSelected)>,
        )> = SystemState::new(world);
        let (mut connections, nodes) = st.get_mut(world);

        let mut action = UndoAction::new("Delete");
        let mut connections_to_remove = HashSet::<Entity>::default();
        for (ent, conn) in connections.iter_mut() {
            let (_, _, input_selected) = nodes.get(conn.input.node_id).unwrap();
            let (_, _, output_selected) = nodes.get(conn.output.node_id).unwrap();
            if input_selected.0 || output_selected.0 {
                connections_to_remove.insert(ent);
            }
        }

        let selected_nodes = nodes
            .iter()
            .filter_map(|(entity, node, selected)| {
                if selected.0 {
                    Some((entity, node.index))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        // Despawn old nodes
        world.resource_scope(|world, mut graph: Mut<GraphResource>| {
            for (_node_entity, node_index) in selected_nodes.iter() {
                graph.0.delete_node(world, *node_index, &mut action);
            }
        });

        // Despawn old connections
        for conn_id in connections_to_remove.drain() {
            world.entity_mut(conn_id).despawn();
        }
    }
}

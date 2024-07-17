use bevy::{
    ecs::{system::SystemState, world::Command},
    prelude::*,
    utils::HashSet,
};

use crate::graph::*;

/// Mark the given node, and all downstream nodes, has modified.
pub(crate) struct MarkModifiedCmd {
    /// Entity for the input terminal.
    pub(crate) start: Entity,
}

impl Command for MarkModifiedCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(
            Query<(&GraphNode, Option<&NodeModified>)>,
            Query<&Terminal>,
            Query<&Connection>,
        )> = SystemState::new(world);
        let (nodes, terminals, connections) = st.get_mut(world);
        let mut to_visit = HashSet::<Entity>::with_capacity(64);
        let mut to_mark = HashSet::<Entity>::with_capacity(64);
        to_visit.insert(self.start);
        while let Some(node_id) = to_visit.iter().next().copied() {
            to_visit.remove(&node_id);
            let Ok((node, modified)) = nodes.get(node_id) else {
                continue;
            };
            if modified.is_some() || to_mark.contains(&node_id) {
                continue;
            }
            to_mark.insert(node_id);
            for (_, output_terminal_id) in node.outputs.iter() {
                let terminal = terminals.get(*output_terminal_id).unwrap();
                for conn_id in terminal.connections.iter() {
                    let conn = connections.get(*conn_id).unwrap();
                    to_visit.insert(conn.input.node_id);
                }
            }
        }

        for node_id in to_mark.iter() {
            world.entity_mut(*node_id).insert(NodeModified);
        }
    }
}

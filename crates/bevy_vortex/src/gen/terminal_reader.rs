use bevy::{ecs::system::SystemParam, prelude::*};

use crate::graph::{Connection, GraphNode, GraphNodeId, Terminal};

use super::{Expr, ShaderAssembly};

#[derive(SystemParam)]
pub struct TerminalReader<'w, 's> {
    /// Query for looking up nodes by id
    pub(crate) nodes: Query<'w, 's, &'static GraphNode>,

    /// Query for looking up terminals by id
    terminals: Query<'w, 's, &'static Terminal>,

    /// Query for looking up connections by id
    connections: Query<'w, 's, &'static Connection>,
}

impl<'w, 's> TerminalReader<'w, 's> {
    /// Read the value of an input terminal.
    pub fn read_input_terminal(
        &self,
        assembly: &mut ShaderAssembly,
        dst_node: Entity,
        terminal_name: &'static str,
    ) -> Option<Expr> {
        let node = self.nodes.get(dst_node).expect("Node not found");
        let dst_terminal_id = node.get_input_terminal(terminal_name)?;
        let dst_terminal = self
            .terminals
            .get(dst_terminal_id)
            .expect("Dest terminal not found");
        if dst_terminal.is_connected() {
            assert_eq!(dst_terminal.connections.len(), 1);
            let connection_id = dst_terminal.connections.iter().next().unwrap();
            let connection = self
                .connections
                .get(*connection_id)
                .expect("Connection not found");
            let src_node_id = connection.output.node_id;
            let src_node = self
                .nodes
                .get(src_node_id)
                .expect("Source terminal not found");
            let src_terminal_id = connection.output.terminal_id;
            let src_terminal = self
                .terminals
                .get(src_terminal_id)
                .expect("Source terminal not found");
            // TODO: If terminal has multiple connections, cache the result.
            if src_terminal.connections.len() > 1 {
                unimplemented!("Multiple connections not supported yet");
            }
            Some(src_node.gen(assembly, self, src_node_id, connection.output.terminal_name))
        } else {
            None
        }
    }

    pub fn get_node_index(&self, node_id: Entity) -> GraphNodeId {
        self.nodes.get(node_id).unwrap().index
    }
}

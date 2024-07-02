use std::sync::{Arc, RwLock};

use bevy::{
    math::IVec2,
    prelude::Resource,
    utils::{HashMap, HashSet},
};

use crate::operator::Operator;

#[derive(Resource)]
pub struct GraphResource(Graph);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphNodeId(usize);

/// A Vortex node graph.
pub struct Graph {
    next_id: usize,
    nodes: HashMap<GraphNodeId, GraphNode>,
    connections: HashSet<Connection>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
}

impl Graph {
    /// Add a node to the graph.
    pub fn add_node(&mut self, node: GraphNode, action: &mut UndoAction) -> GraphNodeId {
        self.next_id += 1;
        let id = GraphNodeId(self.next_id);
        action
            .actions
            .push(GraphUndoAction::AddNode(id, node.clone()));
        // self.add_undo_action(GraphUndoAction::AddNode(id, node.clone()));
        self.nodes.insert(id, node);
        id
    }

    /// Remove a node from the graph.
    pub fn delete_node(&mut self, node_id: GraphNodeId, action: &mut UndoAction) {
        if let Some(node) = self.nodes.remove(&node_id) {
            // Verify that there are no connections to this node.
            assert!(self
                .connections
                .iter()
                .find(|c| c.0.node == node_id || c.1.node == node_id)
                .is_none());
            action
                .actions
                .push(GraphUndoAction::RemoveNode(node_id, node.clone()));
        }
    }

    /// Add a connection to the graph.
    pub fn add_connection(&mut self, connection: Connection, action: &mut UndoAction) {
        action
            .actions
            .push(GraphUndoAction::AddConnection(connection));
        // self.add_undo_action(GraphUndoAction::AddConnection(connection));
        self.connections.insert(connection);
    }

    /// Remove a connection from the graph.
    pub fn remove_connection(&mut self, connection: Connection, action: &mut UndoAction) {
        action
            .actions
            .push(GraphUndoAction::RemoveConnection(connection));
        self.connections.remove(&connection);
    }

    // fn add_undo_action(&mut self, action: GraphUndoAction) {
    //     self.redo_stack.clear();
    //     self.undo_stack.push(action);
    // }
}

/// A node within a node graph. The behavior and attributes of the node are determined by the
/// operator.
// #[derive(Clone)]
#[derive(Clone)]
pub struct GraphNode {
    /// Position of node relative to graph origin.
    position: IVec2,
    /// Operator for this node.
    op: Arc<RwLock<dyn Operator>>,
    /// List of input terminals, derived from operator, with computed positions.
    inputs: Vec<InputTerminal>,
    /// List of output terminatesl, derived from operator, with computed positions.
    outputs: Vec<OutputTerminal>,
}

#[derive(Clone)]
struct InputTerminal {
    /// Data type for this connection
    data_type: ConnectionDataType,
    /// Y-position along left edge of node.
    position: f32,
}

#[derive(Clone)]
struct OutputTerminal {
    /// Data type for this connection
    data_type: ConnectionDataType,
    /// Y-position along right edge of node.
    position: f32,
    /// Text label for this terminal
    label: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputTerminalId {
    node: GraphNodeId,
    terminal: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputTerminalId {
    node: GraphNodeId,
    terminal: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection(pub OutputTerminalId, pub InputTerminalId);

/// The type of an input or output terminal. If the data type does not match, then
/// values will be coerced to the proper type.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionDataType {
    /// A boolean value
    Scalar,
    /// A vector with 2-4 components (Vec2, Vec3 or Vec4).
    Vector,
    /// An RGBA color in either sRGBA or Linear RGBA color space.
    Color,
}

pub struct UndoAction {
    label: &'static str,
    actions: Vec<GraphUndoAction>,
}

pub enum GraphUndoAction {
    AddNode(GraphNodeId, GraphNode),
    RemoveNode(GraphNodeId, GraphNode),
    AddConnection(Connection),
    RemoveConnection(Connection),
}

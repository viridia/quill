use bevy::{
    math::IVec2,
    prelude::{default, Commands, Component, Entity, Resource, World},
    utils::{HashMap, HashSet},
};
use smallvec::SmallVec;

use crate::operator::Operator;

#[derive(Resource, Default)]
pub struct GraphResource(pub(crate) Graph);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphNodeId(usize);

/// A Vortex node graph.
#[derive(Default)]
pub struct Graph {
    nodes: HashMap<GraphNodeId, Entity>,
    next_id: usize,
    connections: HashSet<Connection>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
}

impl Graph {
    /// Return an iterator of the nodes in the graph.
    pub fn iter_nodes(&self) -> bevy::utils::hashbrown::hash_map::Iter<GraphNodeId, Entity> {
        self.nodes.iter()
    }

    /// Return an iterator of the connections in the graph.
    pub fn iter_connections(&self) -> bevy::utils::hashbrown::hash_set::Iter<Connection> {
        self.connections.iter()
    }

    /// Create a new node, given an operator.
    pub fn create_node(
        &mut self,
        commands: &mut Commands,
        operator: Box<dyn Operator>,
        position: IVec2,
        action: &mut UndoAction,
    ) -> GraphNodeId {
        self.next_id += 1;
        let id = GraphNodeId(self.next_id);
        let node = GraphNode {
            id,
            position,
            operator,
            inputs: default(),
            outputs: default(),
        };
        let entity = commands.spawn(node).id();
        action.actions.push(UndoMutation::AddNode(id, entity));
        self.nodes.insert(id, entity);
        id
    }

    /// Remove a node from the graph. The node's connections must be removed first, it will
    /// panic if this has not been done.
    pub fn delete_node(
        &mut self,
        world: &mut World,
        node_id: GraphNodeId,
        action: &mut UndoAction,
    ) {
        assert!(!self
            .connections
            .iter()
            .any(|c| c.0.node == node_id || c.1.node == node_id));
        if let Some(entity) = self.nodes.remove(&node_id) {
            // Verify that there are no connections to this node.
            let mut node_entity = world.entity_mut(entity);
            // Remove the node from the world and put it on the undo stack.
            let node = node_entity.take::<GraphNode>().unwrap();
            node_entity.despawn();
            action.actions.push(UndoMutation::RemoveNode(node_id, node));
        }
    }

    /// Add a connection to the graph.
    pub fn add_connection(&mut self, connection: Connection, action: &mut UndoAction) {
        action.actions.push(UndoMutation::AddConnection(connection));
        // self.add_undo_action(GraphUndoAction::AddConnection(connection));
        self.connections.insert(connection);
    }

    /// Remove a connection from the graph.
    pub fn remove_connection(&mut self, connection: Connection, action: &mut UndoAction) {
        action
            .actions
            .push(UndoMutation::RemoveConnection(connection));
        self.connections.remove(&connection);
    }

    /// Add a new unfo action to the undo stack. Also clears the redo stack.
    pub fn add_undo_action(&mut self, action: UndoAction) {
        self.redo_stack.clear();
        self.undo_stack.push(action);
    }
}

/// A node within a node graph. The behavior and attributes of the node are determined by the
/// operator.
// #[derive(Clone)]
#[derive(Component)]
pub struct GraphNode {
    /// Id of this node. This is used in serialization and undo/redo entries.
    id: GraphNodeId,
    /// Position of node relative to graph origin.
    pub(crate) position: IVec2,
    /// Operator for this node.
    operator: Box<dyn Operator>,
    /// List of input terminals, derived from operator, with computed positions.
    inputs: SmallVec<[InputTerminal; 4]>,
    /// List of output terminals, derived from operator, with computed positions.
    outputs: SmallVec<[OutputTerminal; 1]>,
}

impl GraphNode {
    pub fn title(&self) -> &str {
        self.operator.reflect_short_type_path()
    }
}

impl Clone for GraphNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            position: self.position,
            operator: self.operator.to_boxed_clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
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

/// Represents a user-level action which can be undone or redone.
pub struct UndoAction {
    label: &'static str,
    actions: Vec<UndoMutation>,
}

impl UndoAction {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            actions: default(),
        }
    }
}

/// Represents a single mutation within an [`UndoAction`].
pub enum UndoMutation {
    AddNode(GraphNodeId, Entity),
    RemoveNode(GraphNodeId, GraphNode),
    AddConnection(Connection),
    RemoveConnection(Connection),
}

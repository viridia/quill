use bevy::{
    ecs::{system::SystemState, world::Command},
    hierarchy::BuildChildren,
    math::IVec2,
    prelude::*,
    reflect::{Reflect, TypeInfo},
    utils::{HashMap, HashSet},
};
use smallvec::SmallVec;

use crate::operator::{Operator, OperatorInput, OperatorOutput};

#[derive(Resource, Default)]
pub struct GraphResource(pub(crate) Graph);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphNodeId(usize);

/// A Vortex node graph.
#[derive(Default)]
pub struct Graph {
    nodes: HashMap<GraphNodeId, Entity>,
    next_id: usize,
    connections: HashSet<Entity>,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
}

impl Graph {
    /// Return an iterator of the nodes in the graph.
    pub fn iter_nodes(&self) -> bevy::utils::hashbrown::hash_map::Iter<GraphNodeId, Entity> {
        self.nodes.iter()
    }

    /// Return an iterator of the connections in the graph.
    pub fn iter_connections(&self) -> bevy::utils::hashbrown::hash_set::Iter<Entity> {
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
        let entity = commands.spawn_empty().id();
        let mut node = GraphNode {
            id,
            position,
            operator,
            inputs: default(),
            outputs: default(),
        };
        node.create_terminals(commands, entity);
        commands.entity(entity).insert((node, Selected(false)));
        action.mutations.push(UndoMutation::AddNode(id, entity));
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
        // assert!(!self
        //     .connections
        //     .iter()
        //     .any(|c| c.0.node == node_id || c.1.node == node_id));
        if let Some(entity) = self.nodes.remove(&node_id) {
            // Verify that there are no connections to this node.
            let mut node_entity = world.entity_mut(entity);
            // Remove the node from the world and put it on the undo stack.
            let node = node_entity.take::<GraphNode>().unwrap();
            node_entity.despawn_recursive();
            action
                .mutations
                .push(UndoMutation::RemoveNode(node_id, node));
        }
    }

    /// Add a connection to the graph.
    pub fn add_connection(
        &mut self,
        world: &mut World,
        connection: Connection,
        action: &mut UndoAction,
    ) -> Entity {
        action
            .mutations
            .push(UndoMutation::AddConnection(connection));
        let id = world.spawn(connection).id();
        self.connections.insert(id);
        id
    }

    /// Remove a connection from the graph.
    pub fn remove_connection(
        &mut self,
        world: &mut World,
        connection: Entity,
        action: &mut UndoAction,
    ) {
        if let Some(connection) = world.get_entity_mut(connection) {
            self.connections.remove(&connection.id());
            action.mutations.push(UndoMutation::RemoveConnection(
                *connection.get::<Connection>().unwrap(),
            ));
            connection.despawn();
        }
    }

    /// Add a new unfo action to the undo stack. Also clears the redo stack.
    pub fn add_undo_action(&mut self, action: UndoAction) {
        self.redo_stack.clear();
        self.undo_stack.push(action);
    }
}

/// Component indicating whether a graph node is selected.
/// Note: this used to be a marker, but currently we don't support reactions on markers.
#[derive(Component)]
pub struct Selected(pub bool);

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
    inputs: SmallVec<[(&'static str, Entity); 4]>,
    /// List of output terminals, derived from operator, with computed positions.
    outputs: SmallVec<[(&'static str, Entity); 1]>,
}

impl GraphNode {
    pub fn title(&self) -> &str {
        self.operator.reflect_short_type_path()
    }

    pub fn operator_reflect(&self) -> &dyn Reflect {
        self.operator.as_reflect()
    }

    /// For each node input or output, create an entry which holds the entity used to position
    /// that terminal on the graph view.
    fn create_terminals(&mut self, commands: &mut Commands, parent: Entity) {
        assert!(self.inputs.is_empty());
        assert!(self.outputs.is_empty());
        let reflect = self.operator_reflect();
        let info = reflect.get_represented_type_info().unwrap();
        let TypeInfo::Struct(st_info) = info else {
            panic!("Expected StructInfo");
        };

        let num_fields = st_info.field_len();
        for findex in 0..num_fields {
            let field = st_info.field_at(findex).unwrap();
            let attrs = field.custom_attributes();
            let name = field.name();
            if attrs.contains::<OperatorInput>() {
                let id = commands
                    .spawn(InputTerminal {
                        node_id: parent,
                        name,
                        data_type: ConnectionDataType::Scalar,
                    })
                    .set_parent(parent)
                    .id();
                self.inputs.push((name, id));
            } else if attrs.contains::<OperatorOutput>() {
                let id = commands
                    .spawn(OutputTerminal {
                        node_id: parent,
                        name,
                        data_type: ConnectionDataType::Scalar,
                    })
                    .set_parent(parent)
                    .id();
                self.outputs.push((name, id));
            }
        }
    }

    /// Locate the input terminal with the specified name.
    pub fn get_input_terminal(&self, name: &'static str) -> Option<Entity> {
        self.inputs.iter().find(|t| t.0 == name).map(|t| t.1)
    }

    /// Locate the output terminal with the specified name.
    pub fn get_output_terminal(&self, name: &'static str) -> Option<Entity> {
        self.outputs.iter().find(|t| t.0 == name).map(|t| t.1)
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

/// Component used to store the position of a node while dragging.
#[derive(Component)]
pub struct NodeBasePosition(pub IVec2);

#[derive(Component, Clone)]
pub struct InputTerminal {
    /// Entity id of the node that owns this terminal.
    node_id: Entity,
    /// Entity used to position the terminal.
    // id: Entity,
    /// Name of this field
    name: &'static str,
    /// Data type for this connection
    data_type: ConnectionDataType,
}

#[derive(Component, Clone)]
pub struct OutputTerminal {
    /// Entity id of the node that owns this terminal.
    node_id: Entity,
    /// Name of this field
    name: &'static str,
    /// Entity used to position the terminal.
    // id: Entity,
    /// Data type for this connection
    data_type: ConnectionDataType,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputTerminalId {
    node_index: usize,
    terminal_name: &'static str,
    pub(crate) terminal_id: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputTerminalId {
    node_index: usize,
    terminal_name: &'static str,
    pub(crate) terminal_id: Entity,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
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
    mutations: Vec<UndoMutation>,
}

impl UndoAction {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            mutations: default(),
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

pub(crate) struct AddConnectionCmd {
    pub(crate) input: Entity,
    pub(crate) output: Entity,
}

impl Command for AddConnectionCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(
            ResMut<GraphResource>,
            Query<&InputTerminal>,
            Query<&OutputTerminal>,
            Query<&GraphNode>,
        )> = SystemState::new(world);
        let (_, inputs, outputs, nodes) = st.get_mut(world);
        let input_terminal = inputs.get(self.input).unwrap();
        let output_terminal = outputs.get(self.output).unwrap();
        let input_node = nodes.get(input_terminal.node_id).unwrap();
        let output_node = nodes.get(output_terminal.node_id).unwrap();

        let connection = Connection(
            OutputTerminalId {
                node_index: output_node.id.0,
                terminal_name: output_terminal.name,
                terminal_id: self.output,
            },
            InputTerminalId {
                node_index: input_node.id.0,
                terminal_name: input_terminal.name,
                terminal_id: self.input,
            },
        );

        let mut action = UndoAction::new("Add Connection");
        action
            .mutations
            .push(UndoMutation::AddConnection(connection));
        let id = world.spawn(connection).id();
        let (mut graph, _, _, _) = st.get_mut(world);
        graph.0.connections.insert(id);
        graph.0.add_undo_action(action);
    }
}

pub(crate) struct ValidateConnectionCmd {
    pub(crate) input: Entity,
    pub(crate) output: Entity,
}

impl Command for ValidateConnectionCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(
            ResMut<GraphResource>,
            Query<&InputTerminal>,
            Query<&OutputTerminal>,
            Query<&GraphNode>,
        )> = SystemState::new(world);
        // TODO: Need to inject DragState here
        let (_, inputs, outputs, nodes) = st.get_mut(world);
        // TODO: Validate connection:
        // - can't connect to self
        // - can't connect outputs to outputs
        // - can't connect intputs to inputs
        // - can't connect if data is incompatible
        // - can't create loops
    }
}

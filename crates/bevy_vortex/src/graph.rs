use bevy::{
    ecs::{system::SystemState, world::Command},
    hierarchy::BuildChildren,
    math::IVec2,
    prelude::*,
    reflect::{Reflect, TypeInfo},
    utils::{hashbrown::HashSet, HashMap},
};
use smallvec::SmallVec;

use crate::{
    gen::{Expr, ShaderAssembly, TerminalReader},
    operator::{Operator, OperatorInput, OperatorOutput},
};

#[derive(Resource, Default)]
pub struct GraphResource(pub(crate) Graph);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphNodeId(pub(crate) usize);

/// A component that indicates that a particular view is displaying the output of a node,
/// which means that it needs the material handle generated from the node.
#[derive(Component, Default)]
pub struct NodeObserver {
    pub(crate) node: Option<Entity>,
}

/// Defines whether the material output of a node has changed, and whether it's being rebuilt.
#[derive(PartialEq, Clone, Copy, Default)]
pub enum NodeOutputState {
    /// Material handle is up to date.
    Ready,
    /// Material has changed, but is not rebuilding.
    #[default]
    Modified,
    /// Material is being rebuilt in an async task.
    Building,
}

/// A Vortex node graph.
#[derive(Default)]
pub struct Graph {
    pub(crate) nodes: HashMap<GraphNodeId, Entity>,
    next_id: usize,
    pub(crate) connections: HashSet<Entity>,
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
            index: id,
            position,
            size: IVec2::ZERO,
            operator,
            inputs: default(),
            outputs: default(),
        };
        node.create_terminals(commands, entity);
        commands
            .entity(entity)
            .insert((node, NodeModified, NodeSelected(true)));
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
            // world.commands().entity(entity).despawn_recursive();
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
/// Note: this used to be a marker component, but currently we don't support reactions on markers,
/// because we can't react to removals or additions. Maybe once we start using observers...?
#[derive(Component)]
pub struct NodeSelected(pub bool);

/// A node within a node graph. The behavior and attributes of the node are determined by the
/// operator.
// #[derive(Clone)]
#[derive(Component)]
pub struct GraphNode {
    /// Id of this node. This is used in serialization and undo/redo entries.
    pub(crate) index: GraphNodeId,
    /// Position of node relative to graph origin.
    pub(crate) position: IVec2,
    /// Size of the node, this is calculated by the display code.
    pub(crate) size: IVec2,
    /// Operator for this node.
    operator: Box<dyn Operator>,
    /// List of input terminals, derived from operator, with computed positions.
    pub(crate) inputs: SmallVec<[(&'static str, Entity); 4]>,
    /// List of output terminals, derived from operator, with computed positions.
    pub(crate) outputs: SmallVec<[(&'static str, Entity); 1]>,
}

impl GraphNode {
    pub fn title(&self) -> &str {
        self.operator.reflect_short_type_path()
    }

    pub fn operator_reflect(&self) -> &dyn Reflect {
        self.operator.as_reflect()
    }

    pub fn operator_reflect_mut(&mut self) -> &mut dyn Reflect {
        self.operator.as_reflect_mut()
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
            let type_name = field.type_path();
            let mut data_type = ConnectionDataType::Scalar;
            if type_name.contains("color") {
                data_type = ConnectionDataType::Color;
            } else if type_name == "Vec2" || type_name == "Vec3" || type_name == "Vec4" {
                data_type = ConnectionDataType::Vector;
            }
            // println!("Field: {} ({})", name, type_name);
            if attrs.contains::<OperatorInput>() {
                let id = commands
                    .spawn(Terminal {
                        node_id: parent,
                        name,
                        data_type,
                        connections: HashSet::new(),
                    })
                    .set_parent(parent)
                    .id();
                self.inputs.push((name, id));
            } else if attrs.contains::<OperatorOutput>() {
                let id = commands
                    .spawn(Terminal {
                        node_id: parent,
                        name,
                        data_type,
                        connections: HashSet::new(),
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

    pub fn name(&self) -> &'static str {
        self.operator.name()
    }

    pub fn gen(
        &self,
        assembly: &mut ShaderAssembly,
        reader: &TerminalReader,
        node_id: Entity,
        out_id: &str,
    ) -> Expr {
        self.operator.gen(assembly, reader, node_id, out_id)
    }
}

impl Clone for GraphNode {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            position: self.position,
            size: IVec2::ZERO,
            operator: self.operator.to_boxed_clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
}

/// Marker component that indicates that a graph node has been modified, and it's built shader is
/// out of date.
#[derive(Component)]
pub struct NodeModified;

/// Component used to store the position of a node while dragging.
#[derive(Component)]
pub struct NodeBasePosition(pub IVec2);

#[derive(Component, Clone, Debug)]
pub struct Terminal {
    /// Entity id of the node that owns this terminal.
    pub(crate) node_id: Entity,
    /// Name of this field
    pub(crate) name: &'static str,
    /// Data type for this connection
    pub(crate) data_type: ConnectionDataType,
    /// List of connections to this terminal.
    pub(crate) connections: HashSet<Entity>,
}

impl Terminal {
    pub fn is_connected(&self) -> bool {
        !self.connections.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputTerminalId {
    pub(crate) node_id: Entity,
    pub(crate) terminal_name: &'static str,
    pub(crate) terminal_id: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputTerminalId {
    pub(crate) node_id: Entity,
    pub(crate) terminal_name: &'static str,
    pub(crate) terminal_id: Entity,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection {
    pub output: OutputTerminalId,
    pub input: InputTerminalId,
}

/// The type of an input or output terminal. If the data type does not match, then
/// values will be coerced to the proper type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    pub(crate) mutations: Vec<UndoMutation>,
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

pub(crate) struct ValidateConnectionCmd {
    pub(crate) input: Entity,
    pub(crate) output: Entity,
}

impl Command for ValidateConnectionCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(ResMut<GraphResource>, Query<&Terminal>, Query<&GraphNode>)> =
            SystemState::new(world);
        // TODO: Need to inject DragState here
        let (_, terminals, _nodes) = st.get_mut(world);
        // TODO: Validate connection:
        // - can't connect to self
        // - can't connect outputs to outputs
        // - can't connect intputs to inputs
        // - can't connect if data is incompatible
        // - can't create loops
    }
}

pub(crate) fn sync_connection_refs(world: &mut World) {
    // Hook that watches for changes to connections and updates the back-references in the
    // terminals.
    world
        .register_component_hooks::<Connection>()
        .on_remove(|mut world, entity, _component| {
            if let Some(conn) = world.entity(entity).get::<Connection>() {
                let output_id = conn.output.terminal_id;
                let input_id = conn.input.terminal_id;
                if let Some(mut output_terminal) = world.get_mut::<Terminal>(output_id) {
                    output_terminal.connections.remove(&entity);
                }
                if let Some(mut input_terminal) = world.get_mut::<Terminal>(input_id) {
                    input_terminal.connections.remove(&entity);
                }
            }
        });
}

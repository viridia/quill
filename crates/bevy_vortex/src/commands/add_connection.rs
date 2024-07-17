use bevy::{
    ecs::{system::SystemState, world::Command},
    prelude::*,
};

use crate::{commands::mark_modified::MarkModifiedCmd, graph::*};

pub(crate) struct AddConnectionCmd {
    /// Entity for the input terminal.
    pub(crate) input: Entity,
    /// Entity for the output terminal.
    pub(crate) output: Entity,
    // If set, the entity of the connection to replace.
    pub(crate) replace: Option<Entity>,
}

impl Command for AddConnectionCmd {
    fn apply(self, world: &mut World) {
        let mut st: SystemState<(ResMut<GraphResource>, Query<&mut Terminal>)> =
            SystemState::new(world);
        let (_, mut terminals) = st.get_mut(world);
        assert_ne!(
            self.input, self.output,
            "Cannot connect a terminal to itself"
        );
        let input_terminal = terminals.get(self.input).unwrap();
        let output_terminal = terminals.get(self.output).unwrap();

        let connection = Connection {
            output: OutputTerminalId {
                node_id: output_terminal.node_id,
                terminal_name: output_terminal.name,
                terminal_id: self.output,
            },
            input: InputTerminalId {
                node_id: input_terminal.node_id,
                terminal_name: input_terminal.name,
                terminal_id: self.input,
            },
        };

        let mut input_terminal = terminals.get_mut(self.input).unwrap();
        // Remove any previous connections from input terminal. There can be only one.
        let mut connections_to_remove = std::mem::take(&mut input_terminal.connections);
        // If we're replacing a connection, despawn the old one.
        if let Some(replace) = self.replace {
            connections_to_remove.insert(replace);
        }

        let mut action = UndoAction::new("Add Connection");
        action
            .mutations
            .push(UndoMutation::AddConnection(connection));
        let id = world.spawn(connection).id();
        let (mut graph, _) = st.get_mut(world);
        for conn_id in connections_to_remove.iter() {
            graph.0.connections.remove(conn_id);
        }
        graph.0.connections.insert(id);
        graph.0.add_undo_action(action);

        // Insert the new connection.
        let (_, mut terminals) = st.get_mut(world);
        let mut input_terminal = terminals.get_mut(self.input).unwrap();
        input_terminal.connections.insert(id);
        let input_node = input_terminal.node_id;

        let mut output_terminal = terminals.get_mut(self.output).unwrap();
        output_terminal.connections.insert(id);

        // Mark input node as modified.
        world.commands().add(MarkModifiedCmd { start: input_node });

        // Despawn old connections
        for conn_id in connections_to_remove.drain() {
            world.entity_mut(conn_id).despawn();
        }
    }
}

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

/// For a connection drag, the current drop location.
#[derive(Clone, Debug)]
pub enum DropLocation {
    InputTerminal(Entity),
    OutputTerminal(Entity),
    Position(Vec2),
}

#[derive(Clone, Debug)]
pub enum Gesture {
    /// Drag one or more nodes (ones that are currently selected).
    Move(Vec2),

    /// Drag a node onto the graph to create it.
    Create(Vec2),

    /// Drag an existing connection from either end
    ReconnectStart {
        edge: Entity,
        to: DropLocation,
    },
    ReconnectEnd {
        edge: Entity,
        to: DropLocation,
    },

    /// Create a connection by dragging from an input terminal
    ConnectInput {
        terminal: Entity,
        to: DropLocation,
    },
    /// Create a connection by dragging from an output terminal
    ConnectEnd {
        terminal: Entity,
        to: DropLocation,
    },
    /// Cancel the current connection operation
    ConnectCancel,

    /// Option-click to scroll the view.
    Scroll(Vec2),

    /// Select a rectangular region
    SelectRect(Rect),

    /// Add a node to the selection.
    SelectAdd(Entity),

    /// Remove a node from the selection.
    SelectRemove(Entity),

    /// Toggle the selection state of a node.
    SelectToggle(Entity),

    /// Remove all nodes from the selection.
    SelectClear,
}

#[derive(Clone, Debug)]
pub enum GestureAction {
    Start,
    Move,
    End,
    Cancel,
}

/// Mouse wheel entity event
#[derive(Clone, Event, EntityEvent, Debug)]
#[can_bubble]
pub struct GraphEvent {
    /// Event target
    #[target]
    pub target: Entity,
    /// The type of gesture.
    pub gesture: Gesture,
    /// Whether this is the start, middle or end of a gesture.
    pub action: GestureAction,
}

#[derive(Resource, Default)]
pub(crate) struct GestureState {
    /// The type of gesture.
    pub(crate) gesture: Option<Gesture>,
}

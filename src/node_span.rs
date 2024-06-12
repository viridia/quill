use bevy::prelude::*;

/// Hierarchical data structure representing an ordered sequence of entities. This is the
/// rendered output of a `View`. This gets flattened before attaching to the parent UiNode.
#[derive(Debug, Clone)]
pub enum NodeSpan {
    /// Means that nothing was rendered. This can represent either an initial state
    /// before the first render, or a conditional render operation.
    Empty,

    /// Template rendered a single node
    Node(Entity),

    /// Template rendered a fragment or a list of nodes.
    Fragment(Box<[NodeSpan]>),
}

impl NodeSpan {
    /// Returns the number of actual entities contained in this span.
    pub fn count(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node(_) => 1,
            Self::Fragment(nodes) => nodes.iter().map(|node| node.count()).sum(),
        }
    }

    /// Flattens the list of entities into a vector.
    pub fn flatten(&self, out: &mut Vec<Entity>) {
        match self {
            Self::Empty => {}
            Self::Node(entity) => out.push(*entity),
            Self::Fragment(nodes) => nodes.iter().for_each(|node| node.flatten(out)),
        }
    }

    /// Returns a vector of all entities in this span.
    pub fn to_vec(&self) -> Vec<Entity> {
        let mut out = Vec::new();
        self.flatten(&mut out);
        out
    }

    // Despawn all entities held.
    // pub(crate) fn despawn(&self, world: &mut World) {
    //     match self {
    //         Self::Empty => {}
    //         Self::Node(entity) => {
    //             let mut entt = world.entity_mut(*entity);
    //             entt.remove_parent();
    //             entt.despawn();
    //         }
    //         Self::Fragment(nodes) => nodes.iter().for_each(|node| node.despawn(world)),
    //     }
    // }
}

impl PartialEq for NodeSpan {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Node(l0), Self::Node(r0)) => l0 == r0,
            (Self::Fragment(l0), Self::Fragment(r0)) => {
                l0.len() == r0.len() && l0.iter().zip(r0.as_ref()).all(|(a, b)| a == b)
            }
            _ => false,
        }
    }
}

impl Default for NodeSpan {
    fn default() -> Self {
        Self::Empty
    }
}

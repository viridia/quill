use bevy::ecs::world::World;

use crate::{BuildContext, View, ViewTuple};

use crate::node_span::NodeSpan;

/// A View which renders a sequence of nodes which are inserted into the parent view.
pub struct Fragment<A: ViewTuple> {
    items: A,
}

impl<A: ViewTuple> Fragment<A> {
    /// Construct a new [`Fragment`] from a tuple of views.
    pub fn new(items: A) -> Self {
        Self { items }
    }
}

impl<A: ViewTuple> View for Fragment<A> {
    type State = A::State;

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.items.span_nodes(bc, state)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        self.items.build_spans(bc)
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.items.update_spans(bc, state);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.items.assemble_spans(bc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.items.raze_spans(world, state);
    }
}

impl<A: ViewTuple + Clone> Clone for Fragment<A> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<A: ViewTuple + PartialEq> PartialEq for Fragment<A> {
    fn eq(&self, other: &Self) -> bool {
        self.items.eq(&other.items)
    }
}

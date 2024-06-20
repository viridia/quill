use bevy::ecs::world::World;

use crate::{View, ViewTuple};

use crate::node_span::NodeSpan;

/// A View which renders a sequence of nodes which are inserted into the parent view.
pub struct Fragment<A: ViewTuple> {
    children: A,
}

impl<A: ViewTuple> Fragment<A> {
    /// Construct a new [`Fragment`] from a tuple of views.
    pub fn new(children: A) -> Self {
        Self { children }
    }
}

impl<A: ViewTuple> View for Fragment<A> {
    type State = A::State;

    fn nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        self.children.span_nodes(world, state)
    }

    fn build(&self, cx: &mut crate::Cx) -> Self::State {
        self.children.build_spans(cx)
    }

    fn rebuild(&self, cx: &mut crate::Cx, state: &mut Self::State) -> bool {
        self.children.rebuild_spans(cx, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.children.raze_spans(world, state)
    }
}

impl<A: ViewTuple + Clone> Clone for Fragment<A> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl<A: ViewTuple + PartialEq> PartialEq for Fragment<A> {
    fn eq(&self, other: &Self) -> bool {
        self.children.eq(&other.children)
    }
}

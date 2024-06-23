use bevy::ecs::world::World;

use crate::{View, ViewTuple};

use crate::node_span::NodeSpan;

/// A `Portal` represents a view that is displayed with no parent, causing it's location to
/// be relative to the window rather than any parent view.
pub struct Portal<A: ViewTuple> {
    children: A,
}

impl<A: ViewTuple> Portal<A> {
    /// Construct a new [`Portal`] from a tuple of views.
    pub fn new(children: A) -> Self {
        Self { children }
    }
}

impl<A: ViewTuple> View for Portal<A> {
    type State = A::State;

    fn nodes(&self, _world: &World, _state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
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

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        self.children.attach_descendants(world, state);
        false
    }
}

impl<A: ViewTuple + Clone> Clone for Portal<A> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl<A: ViewTuple + PartialEq> PartialEq for Portal<A> {
    fn eq(&self, other: &Self) -> bool {
        self.children.eq(&other.children)
    }
}

use bevy::ecs::world::World;

use crate::View;

use crate::node_span::NodeSpan;

/// A `Portal` represents a UI node that is displayed with no parent node, causing it's location to
/// be relative to the window rather than any parent node. This only affects the display hierarchy,
/// the [`View`] hierarchy is unaffected.
pub struct Portal<A: View> {
    children: A,
}

impl<A: View> Portal<A> {
    /// Construct a new [`Portal`] view.
    pub fn new(children: A) -> Self {
        Self { children }
    }
}

impl<A: View> View for Portal<A> {
    type State = A::State;

    fn nodes(&self, _world: &World, _state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    fn build(&self, cx: &mut crate::Cx) -> Self::State {
        self.children.build(cx)
    }

    fn rebuild(&self, cx: &mut crate::Cx, state: &mut Self::State) -> bool {
        self.children.rebuild(cx, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.children.raze(world, state)
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        self.children.attach_children(world, state);
        false
    }
}

impl<A: View + Clone> Clone for Portal<A> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl<A: View + PartialEq> PartialEq for Portal<A> {
    fn eq(&self, other: &Self) -> bool {
        self.children.eq(&other.children)
    }
}

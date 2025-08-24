use bevy::ecs::world::{DeferredWorld, World};
use bevy::prelude::Entity;
use bevy::ui::UiTargetCamera;

use crate::View;

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
    type State = (A::State, Option<Entity>);

    fn nodes(&self, _world: &World, _state: &Self::State, _out: &mut Vec<Entity>) {}

    fn build(&self, cx: &mut crate::Cx) -> Self::State {
        let camera = cx
            .use_inherited_component::<UiTargetCamera>()
            .map(|c| c.entity());
        (self.children.build(cx), camera)
    }

    fn rebuild(&self, cx: &mut crate::Cx, state: &mut Self::State) -> bool {
        self.children.rebuild(cx, &mut state.0)
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        self.children.raze(world, &mut state.0)
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        self.children.attach_children(world, &mut state.0);
        // Make sure all children are on the correct camera.
        if let Some(camera) = state.1 {
            let mut nodes: Vec<Entity> = Vec::new();
            self.children.nodes(world, &state.0, &mut nodes);
            for node in nodes.to_vec().iter() {
                world.entity_mut(*node).insert(UiTargetCamera(camera));
            }
        }
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

use std::sync::Arc;

use bevy::ecs::world::{DeferredWorld, World};
use bevy::prelude::Entity;

use crate::{AnyView, BoxedState, View, ViewChild};

/// A view which understands that it's children may change type. When this happens,
/// the old children are razed and the new children are built.
///
/// Dynamic detects when the view type of the children via the method [`View::view_type_id`].
/// When this id changes, the old children are razed and the new children built.
pub struct Dynamic {
    children: ViewChild,
}

impl Dynamic {
    /// Construct a new [`Dynamic`]. This requires a [`ViewChild`], which is a type-erased
    /// view that can be cloned. You can create a [`ViewChild`] by calling [`View::into_view_child`].
    pub fn new(children: ViewChild) -> Self {
        Self { children }
    }
}

impl View for Dynamic {
    type State = (Arc<dyn AnyView>, BoxedState);

    fn nodes(&self, world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        state.0.nodes(world, &state.1, out);
    }

    fn build(&self, cx: &mut crate::Cx) -> Self::State {
        let view = self.children.0.clone();
        let state = view.build(cx);
        (view, state)
    }

    fn rebuild(&self, cx: &mut crate::Cx, state: &mut Self::State) -> bool {
        if state.0.view_type_id() == self.children.0.view_type_id() {
            state.0 = self.children.0.clone();
            state.0.rebuild(cx, &mut state.1)
        } else {
            state
                .0
                .raze(&mut DeferredWorld::from(cx.world_mut()), &mut state.1);
            let view = self.children.0.clone();
            let new_state = view.build(cx);
            state.0 = view;
            state.1 = new_state;
            true
        }
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        state.0.raze(world, &mut state.1)
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        state.0.attach_children(world, &mut state.1)
    }
}

impl Clone for Dynamic {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl PartialEq for Dynamic {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.children.0, &other.children.0)
    }
}

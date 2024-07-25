use std::any::Any;
use std::sync::Arc;

use crate::{AnyView, Cx, View};
use bevy::ecs::world::{DeferredWorld, World};
use bevy::prelude::Entity;

type BoxedState = Box<dyn Any + Send + Sync>;

/// A wrapper around a type-erased view. This is useful when passing views as parameters.
pub struct ViewChild(pub(crate) Arc<dyn AnyView>);

impl ViewChild {
    pub fn new<V: View + 'static>(views: V) -> Self {
        Self(Arc::new(views))
    }
}

impl View for ViewChild {
    type State = Box<dyn Any + Send + Sync + 'static>;

    fn nodes(&self, world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        AnyView::nodes(self.0.as_ref(), world, state, out);
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        AnyView::build(self.0.as_ref(), cx)
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        AnyView::rebuild(self.0.as_ref(), cx, state)
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        AnyView::attach_children(self.0.as_ref(), world, state)
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        AnyView::raze(self.0.as_ref(), world, state)
    }

    fn view_type_id(&self) -> std::any::TypeId {
        AnyView::view_type_id(self.0.as_ref())
    }
}

impl Clone for ViewChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Default for ViewChild {
    fn default() -> Self {
        Self(Arc::new(()))
    }
}

impl PartialEq for ViewChild {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

pub trait IntoViewChild {
    fn into_view_child(self) -> ViewChild;
}

impl<V: View> IntoViewChild for V {
    fn into_view_child(self) -> ViewChild {
        ViewChild::new(self)
    }
}

// TODO: Figure out how to specialize for ChildViews so that we don't double-wrap when
// calling into_child_views on a ChildViews instance.
// impl IntoChildViews for ChildViews {
//     fn into_child_views(self) -> ChildViews {
//         self
//     }
// }

impl View for Vec<ViewChild> {
    type State = Vec<BoxedState>;

    fn nodes(&self, world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        for (view, state) in self.iter().zip(state.iter()) {
            View::nodes(view, world, state, out);
        }
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        self.iter().map(|view| View::build(view, cx)).collect()
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let mut changed = false;
        for (view, state) in self.iter().zip(state.iter_mut()) {
            changed |= View::rebuild(view, cx, state);
        }
        changed
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        for (view, state) in self.iter().zip(state.iter_mut()) {
            View::raze(view, world, state);
        }
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        let mut changed = false;
        for (view, state) in self.iter().zip(state.iter_mut()) {
            changed |= View::attach_children(view, world, state);
        }
        changed
    }
}

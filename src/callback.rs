use std::any::TypeId;

use bevy::{ecs::system::SystemId, prelude::*};

use crate::Cx;

/// Contains a reference to a callback. `P` is the type of the props.
#[derive(PartialEq, Debug)]
pub struct Callback<P = ()> {
    pub(crate) id: SystemId<P, ()>,
}

pub trait AnyCallback: 'static {
    fn remove(&self, world: &mut World);
    fn type_id(&self) -> TypeId;
}

impl dyn AnyCallback + Send + Sync {
    /// Get the original typed callback.
    pub fn downcast<P: 'static>(&self) -> Callback<P> {
        if TypeId::of::<P>() == self.type_id() {
            // Safe because we just checked the type.
            unsafe { *(self as *const dyn AnyCallback as *const Callback<P>) }
        } else {
            panic!("downcast failed")
        }
    }
}

impl<P: 'static> AnyCallback for Callback<P> {
    fn remove(&self, world: &mut World) {
        println!("Removing callback");
        world.remove_system(self.id).unwrap();
    }
    fn type_id(&self) -> TypeId {
        TypeId::of::<P>()
    }
}

impl<P> Copy for Callback<P> {}
impl<P> Clone for Callback<P> {
    fn clone(&self) -> Self {
        *self
    }
}

pub trait RunCallback {
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P);
}

/// A mutable reactive context. This allows write access to reactive data sources.
impl RunCallback for World {
    /// Invoke a callback with the given props.
    ///
    /// Arguments:
    /// * `callback` - The callback to invoke.
    /// * `props` - The props to pass to the callback.
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P) {
        self.run_system_with_input(callback.id, props).unwrap();
    }
}

impl<'p, 'w> RunCallback for Cx<'p, 'w> {
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P) {
        self.world_mut().run_callback(callback, props);
    }
}

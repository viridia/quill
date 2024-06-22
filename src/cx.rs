use std::{cell::RefCell, marker::PhantomData, sync::Arc};

use bevy::{
    hierarchy::BuildWorldChildren,
    prelude::{Component, Entity, IntoSystem, Resource, World},
};

use crate::{mutable::Mutable, tracking_scope::HookState, Callback, MutableCell, WriteMutable};
use crate::{tracking_scope::TrackingScope, ReadMutable};

/// A context parameter that is passed to views and callbacks. It contains the reactive
/// tracking scope, which is used to manage reactive dependencies, as well as a reference to
/// the Bevy world.
pub struct Cx<'p, 'w> {
    /// Bevy World
    world: &'w mut World,

    /// The entity that owns the tracking scope (or will own it).
    pub(crate) owner: Entity,

    /// Set of reactive resources referenced by the presenter.
    pub(crate) tracking: RefCell<&'p mut TrackingScope>,
}

impl<'p, 'w> Cx<'p, 'w> {
    /// Construct a new reactive context.
    pub fn new(world: &'w mut World, owner: Entity, tracking: &'p mut TrackingScope) -> Self {
        Self {
            world,
            owner,
            tracking: RefCell::new(tracking),
        }
    }

    /// Access to world from reactive context.
    pub fn world(&self) -> &World {
        self.world
    }

    /// Access to mutable world from reactive context.
    pub fn world_mut(&mut self) -> &mut World {
        self.world
    }

    /// Returns the id of the entity that owns the tracking scope.
    pub fn owner(&self) -> Entity {
        self.owner
    }

    /// Spawn an empty [`Entity`]. The caller is responsible for despawning the entity.
    pub fn create_entity_untracked(&mut self) -> Entity {
        self.world_mut().spawn_empty().id()
    }

    /// Spawn an empty [`Entity`]. The entity will be despawned when the tracking scope is dropped.
    pub fn create_entity(&mut self) -> Entity {
        let hook = self.tracking.borrow_mut().next_hook();
        match hook {
            Some(HookState::Entity(entity)) => entity,
            Some(_) => {
                panic!("Expected create_entity() hook, found something else");
            }
            None => {
                let entity = self.world_mut().spawn_empty().id();
                self.tracking
                    .borrow_mut()
                    .push_hook(HookState::Entity(entity));
                entity
            }
        }
    }

    /// Create a new [`Mutable`] in this context.
    pub fn create_mutable<T>(&mut self, init: T) -> Mutable<T>
    where
        T: Send + Sync + 'static,
    {
        let hook = self.tracking.borrow_mut().next_hook();
        match hook {
            Some(HookState::Mutable(cell, component)) => Mutable {
                cell,
                component,
                marker: PhantomData,
            },

            Some(_) => {
                panic!("Expected create_mutable() hook, found something else");
            }
            None => {
                let owner = self.owner();
                let cell = self
                    .world_mut()
                    .spawn(MutableCell::<T>(init))
                    .set_parent(owner)
                    .id();
                let component = self.world_mut().init_component::<MutableCell<T>>();
                self.tracking
                    .borrow_mut()
                    .push_hook(HookState::Mutable(cell, component));
                Mutable {
                    cell,
                    component,
                    marker: PhantomData,
                }
            }
        }
    }

    /// Create an effect which runs each time the reactive context is executed, *and* the given
    /// dependencies change.
    ///
    /// Arguments:
    /// - `effect_fn`: The effect function to run.
    /// - `deps`: The dependencies which trigger the effect.
    pub fn create_effect<
        S: Fn(&mut Cx, D) + Send + Sync,
        D: PartialEq + Clone + Send + Sync + 'static,
    >(
        &mut self,
        effect_fn: S,
        deps: D,
    ) {
        let hook = self.tracking.borrow_mut().next_hook();
        match hook {
            Some(HookState::Effect(prev_deps)) => match prev_deps.downcast_ref::<D>() {
                Some(prev_deps) => {
                    if *prev_deps != deps {
                        effect_fn(self, deps.clone());
                        self.tracking
                            .borrow_mut()
                            .replace_hook(HookState::Effect(Arc::new(deps)));
                    }
                }
                None => {
                    panic!("Effect dependencies type mismatch");
                }
            },
            Some(_) => {
                panic!("Expected create_effect() hook, found something else");
            }
            None => {
                effect_fn(self, deps.clone());
                self.tracking
                    .borrow_mut()
                    .push_hook(HookState::Effect(Arc::new(deps)));
            }
        }
    }

    /// Create a new callback in this context. This registers a one-shot system with the world.
    /// The callback will be unregistered when the tracking scope is dropped.
    ///
    /// Note: This function takes no deps argument, the callback is only registered once the first
    /// time it is called. Subsequent calls will return the original callback.
    pub fn create_callback<P: Send + Sync + 'static, M, S: IntoSystem<P, (), M> + 'static>(
        &mut self,
        callback: S,
    ) -> Callback<P> {
        let hook = self.tracking.borrow_mut().next_hook();
        match hook {
            Some(HookState::Callback(cb)) => cb.as_ref().downcast::<P>(),
            Some(_) => {
                panic!("Expected create_callback() hook, found something else");
            }
            None => {
                let id = self.world_mut().register_system(callback);
                let result = Callback {
                    id,
                    marker: PhantomData,
                };
                self.tracking
                    .borrow_mut()
                    .push_hook(HookState::Callback(Arc::new(result)));
                result
            }
        }
    }

    /// Return a reference to the resource of the given type. Calling this function
    /// adds the resource as a dependency of the current presenter invocation.
    pub fn use_resource<T: Resource>(&self) -> &T {
        self.tracking.borrow_mut().track_resource::<T>(self.world);
        self.world.resource::<T>()
    }

    /// Return a reference to the resource of the given type. Calling this function
    /// does not add the resource as a dependency of the current presenter invocation.
    pub fn use_resource_untracked<T: Resource>(&self) -> &T {
        self.world.resource::<T>()
    }

    /// Return a reference to the Component `C` on the given entity.
    pub fn use_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        match self.world.get_entity(entity) {
            Some(c) => {
                self.tracking
                    .borrow_mut()
                    .track_component::<C>(entity, self.world);
                c.get::<C>()
            }
            None => None,
        }
    }

    /// Return a reference to the Component `C` on the given entity. This version does not
    /// add the component to the tracking scope, and is intended for components that update
    /// frequently.
    pub fn use_component_untracked<C: Component>(&self, entity: Entity) -> Option<&C> {
        match self.world.get_entity(entity) {
            Some(c) => c.get::<C>(),
            None => None,
        }
    }
}

impl<'p, 'w> ReadMutable for Cx<'p, 'w> {
    fn read_mutable<T>(&self, mutable: &Mutable<T>) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        self.tracking
            .borrow_mut()
            .track_component_id(mutable.cell, mutable.component);
        self.world.read_mutable(mutable)
    }

    fn read_mutable_clone<T>(&self, mutable: &Mutable<T>) -> T
    where
        T: Send + Sync + Clone + 'static,
    {
        self.tracking
            .borrow_mut()
            .track_component_id(mutable.cell, mutable.component);
        self.world.read_mutable_clone(mutable)
    }

    fn read_mutable_as_ref<T>(&self, mutable: &Mutable<T>) -> &T
    where
        T: Send + Sync + 'static,
    {
        self.tracking
            .borrow_mut()
            .track_component_id(mutable.cell, mutable.component);
        self.world.read_mutable_as_ref(mutable)
    }

    fn read_mutable_map<T, U, F: Fn(&T) -> U>(&self, mutable: &Mutable<T>, f: F) -> U
    where
        T: Send + Sync + 'static,
    {
        self.tracking
            .borrow_mut()
            .track_component_id(mutable.cell, mutable.component);
        self.world.read_mutable_map(mutable, f)
    }
}

impl<'p, 'w> WriteMutable for Cx<'p, 'w> {
    fn write_mutable<T>(&mut self, mutable: Entity, value: T)
    where
        T: Send + Sync + Copy + PartialEq + 'static,
    {
        self.world.write_mutable(mutable, value);
    }

    fn write_mutable_clone<T>(&mut self, mutable: Entity, value: T)
    where
        T: Send + Sync + Clone + PartialEq + 'static,
    {
        self.world.write_mutable_clone(mutable, value);
    }
}

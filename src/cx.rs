use std::cell::RefCell;

use bevy::prelude::{Entity, Resource, World};

use crate::tracking_scope::TrackingScope;

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
        let entity = self.world_mut().spawn_empty().id();
        self.tracking.borrow_mut().add_owned(entity);
        entity
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
}

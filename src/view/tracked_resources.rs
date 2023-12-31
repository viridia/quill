use std::marker::PhantomData;

use bevy::ecs::{component::Component, system::Resource, world::World};

pub trait AnyResource: Send + Sync {
    fn is_changed(&self, world: &World) -> bool;
}

#[derive(PartialEq, Eq)]
pub struct TrackedResource<T> {
    pub pdata: PhantomData<T>,
}

impl<T> TrackedResource<T> {
    pub(crate) fn new() -> Self {
        Self { pdata: PhantomData }
    }
}

impl<T> AnyResource for TrackedResource<T>
where
    T: Resource,
{
    fn is_changed(&self, world: &World) -> bool {
        world.is_resource_changed::<T>()
    }
}

/// List of resources used by a presenter.
pub(crate) type TrackedResourceList = Vec<Box<dyn AnyResource>>;

/// Tracks resources used by each View tree entity
#[derive(Component, Default)]
pub struct TrackedResources {
    pub data: TrackedResourceList,
}

use std::sync::atomic::AtomicBool;

use bevy::{
    ecs::component::{ComponentId, Tick},
    prelude::*,
    utils::HashSet,
};

/// Tracks the sequence of hook calls within a reaction.
#[derive(Clone, Copy)]
pub(crate) enum HookState {
    CreateEntity(Entity),
    CreateMutable(Entity, ComponentId),
}

/// A component that tracks the dependencies of a reactive task.
#[derive(Component)]
pub struct TrackingScope {
    /// List of entities that are owned by this scope.
    hook_states: Vec<HookState>,

    /// During rebuilds, the index of the hook that is currently being processed.
    next_hook_index: usize,

    /// Set of components that we are currently subscribed to.
    component_deps: HashSet<(Entity, ComponentId)>,

    /// Set of resources that we are currently subscribed to.
    resource_deps: HashSet<ComponentId>,

    /// Allows a tracking scope to be explictly marked as changed for reasons other than
    /// a component or resource dependency mutation.
    changed: AtomicBool,

    /// Engine tick used for determining if components have changed. This represents the
    /// time of the previous reaction.
    pub(crate) tick: Tick,

    /// List of cleanup functions to call when the scope is dropped.
    #[allow(clippy::type_complexity)]
    pub(crate) cleanups: Vec<Box<dyn FnOnce(&mut World) + 'static + Sync + Send>>,
}

/// A resource which, if inserted, displays the view entities that have reacted this frame.
#[derive(Resource)]
pub struct TrackingScopeTracing(pub Vec<Entity>);

impl FromWorld for TrackingScopeTracing {
    fn from_world(_world: &mut World) -> Self {
        Self(Vec::new())
    }
}

impl TrackingScope {
    /// Create a new tracking scope.
    pub fn new(tick: Tick) -> Self {
        Self {
            hook_states: Vec::new(),
            next_hook_index: 0,
            component_deps: HashSet::default(),
            resource_deps: HashSet::default(),
            changed: AtomicBool::new(false),
            tick,
            cleanups: Vec::new(),
        }
    }

    pub(crate) fn push_hook(&mut self, hook: HookState) {
        assert!(self.next_hook_index == self.hook_states.len());
        self.hook_states.push(hook);
        self.next_hook_index += 1;
    }

    pub(crate) fn next_hook(&mut self) -> Option<HookState> {
        if self.next_hook_index < self.hook_states.len() {
            let hook = self.hook_states[self.next_hook_index];
            self.next_hook_index += 1;
            Some(hook)
        } else {
            None
        }
    }

    // pub(crate) fn add_owned(&mut self, owned: Entity) {
    //     if self.next_hook_index < self.hook_states.len() {
    //         match &self.hook_states[self.next_hook_index] {
    //             HookState::CreateEntity(entity) => {
    //                 assert!(*entity == owned, "Hook state mismatch");
    //             }
    //             _ => panic!("Expected CreateEntity hook"),
    //         }
    //     } else {
    //         self.hook_states.push(HookState::CreateEntity(owned));
    //     }
    //     // self.hook_states.push(HookState::CreateEntity(owned));
    // }

    /// Add a cleanup function which will be run once before the next reaction.
    pub(crate) fn add_cleanup(&mut self, cleanup: impl FnOnce(&mut World) + 'static + Sync + Send) {
        self.cleanups.push(Box::new(cleanup));
    }

    /// Convenience method for adding a resource dependency.
    pub(crate) fn track_resource<T: Resource>(&mut self, world: &World) {
        self.resource_deps.insert(
            world
                .components()
                .resource_id::<T>()
                .expect("Unknown resource type"),
        );
    }

    /// Convenience method for adding a component dependency.
    pub(crate) fn track_component<C: Component>(&mut self, entity: Entity, world: &World) {
        self.track_component_id(
            entity,
            world
                .components()
                .component_id::<C>()
                .expect("Unknown component type"),
        );
    }

    /// Convenience method for adding a component dependency by component id.
    pub(crate) fn track_component_id(&mut self, entity: Entity, component: ComponentId) {
        self.component_deps.insert((entity, component));
    }

    /// Mark the scope as changed for reasons other than a component or resource dependency.
    pub(crate) fn set_changed(&self) {
        self.changed
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Returns true if any of the dependencies of this scope have been updated since
    /// the previous reaction.
    pub(crate) fn dependencies_changed(&self, world: &World, tick: Tick) -> bool {
        self.components_changed(world, tick)
            || self.resources_changed(world, tick)
            || self.changed.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn components_changed(&self, world: &World, tick: Tick) -> bool {
        self.component_deps.iter().any(|(e, c)| {
            world.get_entity(*e).map_or(false, |e| {
                e.get_change_ticks_by_id(*c)
                    .map(|ct| ct.is_changed(self.tick, tick))
                    .unwrap_or(false)
            })
        })
    }

    fn resources_changed(&self, world: &World, tick: Tick) -> bool {
        self.resource_deps.iter().any(|c| {
            world
                .get_resource_change_ticks_by_id(*c)
                .map(|ct| ct.is_changed(self.tick, tick))
                .unwrap_or(false)
        })
    }

    /// Take the dependencies from another scope. Typically the other scope is a temporary
    /// scope that is used to compute the next set of dependencies.
    pub(crate) fn take_deps(&mut self, other: &mut Self) {
        self.component_deps = std::mem::take(&mut other.component_deps);
        self.resource_deps = std::mem::take(&mut other.resource_deps);
        self.cleanups = std::mem::take(&mut other.cleanups);
        self.hook_states = std::mem::take(&mut other.hook_states);
        self.changed.store(
            other.changed.load(std::sync::atomic::Ordering::Relaxed),
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    pub(crate) fn take_hooks(&mut self, other: &mut Self) {
        self.hook_states = std::mem::take(&mut other.hook_states)
    }
}

/// Trait which allows despawning of any owned objects or reactions in the tracking scope
/// associated with an entity. This operation is recursive in that an owned object may itself
/// own other objects.
pub trait DespawnScopes {
    /// Despawn all owned objects and reactions associated with the given entity.
    fn despawn_owned_recursive(&mut self, scope_entity: Entity);
}

impl DespawnScopes for World {
    fn despawn_owned_recursive(&mut self, scope_entity: Entity) {
        let mut entt = self.entity_mut(scope_entity);
        let Some(mut scope) = entt.get_mut::<TrackingScope>() else {
            return;
        };
        // Run any cleanups
        let mut cleanups = std::mem::take(&mut scope.cleanups);
        // Recursively despawn owned objects
        let hooks = std::mem::take(&mut scope.hook_states);
        entt.despawn();
        for cleanup_fn in cleanups.drain(..) {
            cleanup_fn(self);
        }
        for hook in hooks {
            match hook {
                HookState::CreateEntity(owned) => {
                    self.entity_mut(owned).despawn_recursive();
                }
                _ => panic!("Unexpected hook state"),
            }
        }
    }
}

/// Run reactions whose dependencies have changed.
// pub fn run_reactions(world: &mut World) {
//     let mut scopes = world.query::<(Entity, &mut TrackingScope, &ReactionCell)>();
//     let mut changed = HashSet::<Entity>::default();
//     let tick = world.change_tick();
//     for (entity, scope, _) in scopes.iter(world) {
//         if scope.dependencies_changed(world, tick) {
//             changed.insert(entity);
//         }
//     }

//     // Record the changed entities for debugging purposes.
//     if let Some(mut tracing) = world.get_resource_mut::<TrackingScopeTracing>() {
//         // Check for empty first to avoid setting mutation flag.
//         if !tracing.0.is_empty() {
//             tracing.0.clear();
//         }
//         if !changed.is_empty() {
//             tracing.0.extend(changed.iter().copied());
//         }
//     }

//     for scope_entity in changed.iter() {
//         // Call registered cleanup functions
//         let (_, mut scope, _) = scopes.get_mut(world, *scope_entity).unwrap();
//         let mut cleanups = std::mem::take(&mut scope.cleanups);
//         for cleanup_fn in cleanups.drain(..) {
//             cleanup_fn(world);
//         }

//         // Run the reaction
//         let (_, _, reaction_cell) = scopes.get_mut(world, *scope_entity).unwrap();
//         let mut next_scope = TrackingScope::new(tick);
//         let inner = reaction_cell.0.clone();
//         inner
//             .lock()
//             .unwrap()
//             .react(*scope_entity, world, &mut next_scope);

//         // Replace deps and cleanups in the current scope with the next scope.
//         let (_, mut scope, _) = scopes.get_mut(world, *scope_entity).unwrap();
//         scope.take_deps(&mut next_scope);
//         scope.tick = tick;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Default)]
    struct TestResource(bool);

    #[test]
    fn test_resource_deps_changed() {
        let mut world = World::default();
        let tick = world.change_tick();
        let mut scope = TrackingScope::new(tick);

        // No dependencies, so the result should be false
        assert!(!scope.dependencies_changed(&world, tick));

        world.increment_change_tick();
        world.insert_resource(TestResource(false));
        scope.track_resource::<TestResource>(&world);
        assert!(scope.resource_deps.len() == 1);

        // Resource added
        let tick = world.change_tick();
        assert!(scope.dependencies_changed(&world, tick));

        // Reset scope tick
        scope.tick = tick;
        assert!(!scope.dependencies_changed(&world, tick));

        // Mutate the resource
        world.increment_change_tick();
        world.get_resource_mut::<TestResource>().unwrap().0 = true;
        let tick = world.change_tick();
        assert!(scope.dependencies_changed(&world, tick));
    }
}

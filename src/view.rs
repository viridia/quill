use crate::{cx::Cx, tracking_scope::TrackingScope, NodeSpan};
use bevy::prelude::{Added, Component, Entity, World};
use std::sync::{Arc, Mutex};

#[allow(unused)]
/// An object which produces one or more display nodes.
pub trait View: Sync + Send + 'static {
    /// The external state for this View.
    type State: Send + Sync;

    /// Return the span of entities produced by this View.
    fn nodes(&self, state: &Self::State) -> NodeSpan;

    /// Construct and patch the tree of UiNodes produced by this view.
    /// This may also spawn child entities representing nested components.
    fn build(&self, cx: &mut Cx) -> Self::State;

    /// Update the internal state of this view, re-creating any UiNodes.
    /// Returns true if the output changed, that is, if `nodes()` would return a different value
    /// than it did before the rebuild.
    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool;

    /// Instructs the view to attach any child entities to the parent entity. This is called
    /// whenever we know that one or more child entities have changed.
    fn attach_children(&self, cx: &mut Cx, state: &mut Self::State) {}

    /// Recursively despawn any child entities that were created as a result of calling `.build()`.
    /// This calls `.raze()` for any nested views within the current view state.
    fn raze(&self, world: &mut World, state: &mut Self::State);

    // / Build a ViewRoot from this view.
    fn to_root(self) -> (ViewStateHolder<Self>, ViewThunk, ViewRoot)
    where
        Self: Sized,
    {
        let holder = ViewStateHolder::new(self);
        let thunk = holder.create_thunk();
        (holder, thunk, ViewRoot)
    }
}

/// Combination of a [`View`] and it's built state.
pub struct ViewState<S, V: View<State = S>> {
    pub(crate) view: V,
    pub(crate) state: Option<S>,
}

impl<S, V: View<State = S>> ViewState<S, V> {
    fn rebuild(&mut self, cx: &mut Cx) -> bool {
        if let Some(state) = self.state.as_mut() {
            self.view.rebuild(cx, state)
        } else {
            let state = self.view.build(cx);
            self.state = Some(state);
            true
        }
    }
}

#[derive(Component)]
pub struct ViewStateHolder<V: View>(pub Arc<Mutex<ViewState<V::State, V>>>);

impl<V: View> ViewStateHolder<V> {
    pub fn new(view: V) -> Self {
        Self(Arc::new(Mutex::new(ViewState { view, state: None })))
    }

    pub fn create_thunk(&self) -> ViewThunk {
        ViewThunk(&ViewAdapter::<V> {
            marker: std::marker::PhantomData,
        })
    }
}

pub struct ViewAdapter<V: View> {
    marker: std::marker::PhantomData<V>,
}

/// Type-erased trait for a [`ViewState`].
pub trait AnyViewAdapter: Sync + Send + 'static {
    /// Return the span of entities produced by this View.
    fn nodes(&self, world: &mut World, entity: Entity) -> NodeSpan;

    /// Update the internal state of this view, re-creating any UiNodes. Returns true if the output
    /// changed, that is, if `nodes()` would return a different value than it did before the
    /// rebuild.
    fn rebuild(&self, world: &mut World, entity: Entity, scope: &mut TrackingScope) -> bool;

    /// Recursively despawn any child entities that were created as a result of calling `.build()`.
    /// This calls `.raze()` for any nested views within the current view state.
    fn raze(&self, world: &mut World, entity: Entity);
}

impl<V: View> AnyViewAdapter for ViewAdapter<V> {
    fn nodes(&self, world: &mut World, entity: Entity) -> NodeSpan {
        match world.entity(entity).get::<ViewStateHolder<V>>() {
            Some(view_cell) => {
                let vstate = view_cell.0.lock().unwrap();
                match &vstate.state {
                    Some(state) => vstate.view.nodes(state),
                    None => NodeSpan::Empty,
                }
            }
            None => NodeSpan::Empty,
        }
    }

    fn rebuild(&self, world: &mut World, entity: Entity, scope: &mut TrackingScope) -> bool {
        let mut cx = Cx::new(world, entity, scope);
        if let Some(view_cell) = cx
            .world_mut()
            .entity_mut(entity)
            .get_mut::<ViewStateHolder<V>>()
        {
            let inner = view_cell.0.clone();
            let mut vstate = inner.lock().unwrap();
            vstate.rebuild(&mut cx)
        } else {
            false
        }
    }

    fn raze(&self, world: &mut World, entity: Entity) {
        if let Some(view_cell) = world.entity_mut(entity).take::<ViewStateHolder<V>>() {
            let mut vstate = view_cell.0.lock().unwrap();
            if let Some(mut state) = vstate.state.take() {
                vstate.view.raze(world, &mut state);
            }
        }
    }
}

#[derive(Component)]
pub struct ViewThunk(&'static dyn AnyViewAdapter);

/// An ECS component which holds a reference to the root of a view hierarchy.
#[derive(Component)]
pub struct ViewRoot;

/// A reference to a [`View`] which can be passed around as a parameter.
// pub struct ViewHandle(pub(crate) Arc<Mutex<dyn AnyViewState>>);

// impl ViewHandle {
/// Construct a new [`ViewRef`] from a [`View`].
// pub fn new(view: impl View) -> Self {
//     Self(Arc::new(Mutex::new(ViewState { view, state: None })))
// }

// /// Given a view template, construct a new view. This creates an entity to hold the view
// /// and the view handle, and then calls [`View::build`] on the view. The resuling entity
// /// is part of the template invocation hierarchy, it is not a display node.
// pub fn spawn(view: &ViewHandle, parent: Entity, world: &mut World) -> Entity {
//     todo!("spawn view");
//     // let mut child_ent = world.spawn(ViewCell(view.0.clone()));
//     // child_ent.set_parent(parent);
//     // let id = child_ent.id();
//     // view.0.lock().unwrap().build(child_ent.id(), world);
//     // id
// }

/// Returns the display nodes produced by this `View`.
//     pub fn nodes(&self) -> NodeSpan {
//         self.0.lock().unwrap().nodes()
//     }

//     /// Destroy the view, including the display nodes, and all descendant views.
//     pub fn raze(&self, world: &mut World) {
//         self.0.lock().unwrap().raze(world);
//     }
// }

// impl Clone for ViewHandle {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

// impl Default for ViewHandle {
//     fn default() -> Self {
//         Self(Arc::new(Mutex::new(EmptyView)))
//     }
// }

/// View which renders nothing.
pub struct EmptyView;

// #[allow(unused_variables)]
// impl AnyViewState for EmptyView {
//     fn nodes(&self) -> NodeSpan {
//         NodeSpan::Empty
//     }

//     fn rebuild(&mut self, cx: &mut Cx) -> bool {
//         false
//     }

//     fn raze(&mut self, world: &mut World) {}

//     // fn owner(&self) -> Option<Entity> {
//     //     None
//     // }
// }

/// Trait that defines a factory object that can construct a [`View`] from a reactive context.
/// View factories are themselves views, when they are built or rebuild they call `create()` to
/// create a new, temporary [`View`] which is then immediately invoked. Note that the view
/// is not memoized, and shares the same reactive context as the caller; this means that when
/// the root view is rebuilt, the entire tree will be rebuilt as well.
pub trait ViewFactory {
    type View: View;

    /// Create the view for the control.
    fn create(&self, cx: &mut Cx) -> Self::View;
}

impl<Factory: ViewFactory + Send + Sync + 'static> View for Factory {
    type State = (Factory::View, <Factory::View as View>::State);

    fn nodes(&self, state: &Self::State) -> NodeSpan {
        state.0.nodes(&state.1)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let view = self.create(cx);
        let state = view.build(cx);
        (view, state)
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        state.0 = self.create(cx);
        state.0.rebuild(cx, &mut state.1)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        state.0.raze(world, &mut state.1)
    }
}

// This doesn't work. ViewTemplates can't be Views, not directly. They need to be transformed
// into views so that they can be wrapped in an Arc. This means that ViewTuple has to take
// an IntoView. This means we need something like an IntoViewTuple.
//
// The basic issue is that unlike in bevy_reactor, create() gets called multiple times -
// that is, each time the parent view is rebuild, The ViewTemplate might be different, and in
// fact might be a different instance.
//

// impl<VT: ViewTemplate + Send + Sync + 'static> View for ViewTemplateView<VT> {
//     type State = (Entity, NodeSpan);

//     fn nodes(&self, state: &Self::State) -> NodeSpan {
//         state.1.clone()
//     }

//     fn build(&self, cx: &mut Cx) -> Self::State {
//         let tick = cx.world_mut().change_tick();
//         let owner = cx.world_mut().spawn_empty().id();
//         let mut scope = TrackingScope::new(tick);
//         let mut cxi = Cx::new(cx.world_mut(), owner, &mut scope);
//         let view = self.view.create(&mut cxi);
//         let state = view.build(&mut cxi);
//         let nodes = view.nodes(&state);
//         // Need to insert a view cell with the view state into the entity.
//         cx.world_mut().entity_mut(owner).insert(scope);
//         (owner, nodes)
//     }

//     fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
//         // Note: This doesn't rebuild the content of the view, only syncs the `NodeSpan`s
//         // that have already been generated. The rebuilding of the view happens asynchronously via
//         // the tracking scope update.
//         // TODO: This doesn't work because the view might have different parameters; we'd want to
//         // update the view state props.
//         let vc = cx.world_mut().entity(state.0).get::<ViewCell>().unwrap();
//         let nodes = vc.0.lock().unwrap().nodes();
//         if nodes != state.1 {
//             state.1 = nodes;
//             return true;
//         }
//         false
//     }

//     fn raze(&self, world: &mut World, state: &mut Self::State) {
//         let vc = world.entity(state.0).get::<ViewCell>().unwrap();
//         let cell = vc.0.clone();
//         let mut view = cell.lock().unwrap();
//         view.raze(world);
//         world.entity_mut(state.0).remove_parent();
//         world.entity_mut(state.0).despawn();
//     }
// }

// / Holds a [`ViewTemplate`], and the entity and output nodes created by the [`View`] produced
// / by the factory.
// pub struct ViewTemplateState<VT: ViewTemplate, V: View> {
//     /// A reference to the generating template.
//     template: Arc<VT>,

//     /// A reference to the view.
//     view: V,

//     /// The view's state.
//     state: V::State,

//     /// The entity that holds the tracking context for the view.
//     owner: Entity,
// }

// impl<V: View, VT: ViewTemplate + Send + Sync + 'static> AnyViewState for ViewTemplateState<VT, V> {
//     fn nodes(&self) -> NodeSpan {
//         self.view.nodes(&self.state)
//     }

//     fn rebuild(&mut self, cx: &mut Cx) -> bool {
//         let view = self.template.create(cx);
//         todo!();
//         // unsafe { view.rebuild(cx, &mut self.state) }
//     }

//     fn raze(&mut self, world: &mut World) {
//         self.view.raze(world, &mut self.state)
//     }

//     fn owner(&self) -> Option<Entity> {
//         Some(self.owner)
//     }
// }

// impl<W: ViewTemplate> ViewTemplateState<W> {
//     /// Construct a new `WidgetInstance`.
//     pub fn new(template: W) -> Self {
//         Self {
//             template,
//             template_entity: None,
//             state: None,
//             nodes: NodeSpan::Empty,
//         }
//     }
// }

// impl<VT: ViewTemplate + Send + Sync + 'static> AnyViewState for ViewTemplateState<VT> {
//     fn nodes(&self) -> NodeSpan {
//         self.nodes.clone()
//     }

//     fn rebuild(&mut self, cx: &mut Cx) -> bool {
//         match self.template_entity {
//             Some(entity) => {
//                 let mut entt = cx.world_mut().get_entity_mut(entity).unwrap();
//                 let mut scope = entt.get_mut::<TrackingScope>().unwrap();
//                 // let mut cxi = Cx::new(cx.world_mut(), entity, &mut scope);
//                 // let state = self.state.as_mut().unwrap();
//                 // let view = self.template.create(&mut cxi);
//                 // view.rebuild(&mut cx, &mut state.state);
//                 true
//             }
//             None => {
//                 // Entity does not exist
//                 let tick = cx.world_mut().change_tick();
//                 let template_entity = cx.world_mut().spawn_empty().id();
//                 self.template_entity = Some(template_entity);
//                 let mut scope = TrackingScope::new(tick);
//                 let mut cxi = Cx::new(cx.world_mut(), template_entity, &mut scope);
//                 let view = self.template.create(&mut cxi);
//                 let state = view.build(&mut cxi);
//                 cx.world_mut().entity_mut(template_entity).insert(scope);
//                 self.state = Some(Box::new(ViewState {
//                     view,
//                     state: Some(state),
//                 }));
//                 // self.nodes = self.template.create(&mut cx).nodes(&());
//                 true
//             }
//         }
//         // let mut view = self.template.create(cx);
//         // let changed = view.rebuild(cx, &mut self.state);
//         // self.nodes = view.nodes(&self.state);
//         // changed
//     }

//     fn raze(&mut self, world: &mut World) {
//         // self.nodes.raze(world);
//         assert!(self.state.is_some());
//         match self.state {
//             Some(ref mut state) => {
//                 state.raze(world);
//             }
//             None => {}
//         }
//         self.state = None;
//         // let mut entt = world.entity_mut(self.output_entity.unwrap());
//         // if let Some(handle) = entt.get_mut::<ViewCell>() {
//         //     // Despawn the inner view.
//         //     handle.0.clone().lock().unwrap().raze(entt.id(), world);
//         // };
//         // self.output_entity = None;
//         // world.despawn_owned_recursive(view_entity);
//     }
// }

pub(crate) fn build_views(world: &mut World) {
    let mut roots = world.query_filtered::<(Entity, &ViewThunk), Added<ViewRoot>>();
    let roots_copy: Vec<Entity> = roots.iter(world).map(|(e, _)| e).collect();
    let tick = world.change_tick();
    for root_entity in roots_copy.iter() {
        let Ok((_, root)) = roots.get(world, *root_entity) else {
            continue;
        };
        let mut scope = TrackingScope::new(tick);
        root.0.rebuild(world, *root_entity, &mut scope);
        world.entity_mut(*root_entity).insert(scope);
    }
}

pub(crate) fn rebuild_views(world: &mut World) {
    // let mut divergence_ct: usize = 0;
    // let mut prev_change_ct: usize = 0;
    let this_run = world.change_tick();

    // let mut v = HashSet::new();

    // Scan changed resources
    let mut scopes = world.query::<(Entity, &mut TrackingScope, &ViewThunk)>();
    let changed = scopes
        .iter(world)
        .filter_map(|(e, scope, _)| {
            if scope.dependencies_changed(world, this_run) {
                Some(e)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // for (e, scope) in q.iter(world) {
    //     if scope.dependencies_changed(world, this_run) {
    //         v.insert(e);
    //     }
    // }

    // Record the changed entities for debugging purposes.
    // if let Some(mut tracing) = world.get_resource_mut::<TrackingScopeTracing>() {
    //     // Check for empty first to avoid setting mutation flag.
    //     if !tracing.0.is_empty() {
    //         tracing.0.clear();
    //     }
    //     if !changed.is_empty() {
    //         tracing.0.extend(changed.iter().copied());
    //     }
    // }

    for scope_entity in changed.iter() {
        // println!("Rebuilding view {:?}", scope_entity);
        // Call registered cleanup functions
        let (_, mut scope, _) = scopes.get_mut(world, *scope_entity).unwrap();
        let mut cleanups = std::mem::take(&mut scope.cleanups);
        for cleanup_fn in cleanups.drain(..) {
            cleanup_fn(world);
        }

        // Run the reaction
        let (_, _, view_cell) = scopes.get_mut(world, *scope_entity).unwrap();
        let mut next_scope = TrackingScope::new(this_run);
        view_cell.0.rebuild(world, *scope_entity, &mut next_scope);

        // Replace deps and cleanups in the current scope with the next scope.
        let (_, mut scope, _) = scopes.get_mut(world, *scope_entity).unwrap();
        scope.take_deps(&mut next_scope);
        scope.tick = this_run;
    }

    // // force build every view that just got spawned
    // let mut qf = world.query_filtered::<Entity, Added<ViewHandle>>();
    // for e in qf.iter(world) {
    //     v.insert(e);
    // }

    // loop {
    //     // This is inside a loop because rendering may trigger further changes.

    //     // This means that either a presenter was just added, or its props got modified by a parent.
    //     let mut qf =
    //         world.query_filtered::<Entity, (With<ViewHandle>, With<PresenterStateChanged>)>();
    //     for e in qf.iter_mut(world) {
    //         v.insert(e);
    //     }

    //     for e in v.iter() {
    //         world.entity_mut(*e).remove::<PresenterStateChanged>();
    //     }

    //     // Most of the time changes will converge, that is, the number of changed presenters
    //     // decreases each time through the loop. A "divergence" is when that fails to happen.
    //     // We tolerate a maximum number of divergences before giving up.
    //     let change_ct = v.len();
    //     if change_ct >= prev_change_ct {
    //         divergence_ct += 1;
    //         if divergence_ct > MAX_DIVERGENCE_CT {
    //             panic!("Reactions failed to converge, num changes: {}", change_ct);
    //         }
    //     }
    //     prev_change_ct = change_ct;

    //     // phase 2
    //     if change_ct > 0 {
    //         for e in v.drain() {
    //             let mut entt = world.entity_mut(e);
    //             // Clear tracking lists for presenters to be re-rendered.
    //             if let Some(mut tracked_resources) = entt.get_mut::<TrackedResources>() {
    //                 tracked_resources.data.clear();
    //             }
    //             if let Some(mut tracked_components) = entt.get_mut::<TrackedComponents>() {
    //                 tracked_components.data.clear();
    //             }

    //             // Clone the ViewHandle so we can call build() on it.
    //             let Some(view_handle) = entt.get_mut::<ViewHandle>() else {
    //                 continue;
    //             };
    //             let inner = view_handle.inner.clone();
    //             let mut ec = BuildContext::new(world, e);
    //             inner.lock().unwrap().build(&mut ec, e);
    //         }
    //     } else {
    //         break;
    //     }
    // }

    // // phase 3
    // loop {
    //     let mut qf = world.query_filtered::<Entity, With<PresenterGraphChanged>>();
    //     let changed_entities: Vec<Entity> = qf.iter(world).collect();
    //     if changed_entities.is_empty() {
    //         break;
    //     }
    //     // println!("Entities changed: {}", changed_entities.len());
    //     for e in changed_entities {
    //         // println!("PresenterGraphChanged {:?}", e);
    //         let mut ent = world.entity_mut(e);
    //         ent.remove::<PresenterGraphChanged>();
    //         let Some(view_handle) = world.get_mut::<ViewHandle>(e) else {
    //             continue;
    //         };
    //         let inner = view_handle.inner.clone();
    //         let mut bc = BuildContext::new(world, e);
    //         inner.lock().unwrap().attach(&mut bc, e);
    //     }
    // }
}

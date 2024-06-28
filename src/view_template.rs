use crate::{cx::Cx, tracking_scope::TrackingScope, AnyViewAdapter, NodeSpan, View, ViewThunk};
use bevy::{
    hierarchy::BuildWorldChildren,
    prelude::{Component, Entity, World},
};
use std::sync::{Arc, Mutex};

#[cfg(feature = "verbose")]
use bevy::log::info;

/// Trait that defines a template object that can construct a [`View`] from a reactive context.
/// View templates are themselves views, when they are built or rebuild they call `create()` to
/// create a new, temporary [`View`] which is then immediately invoked.
///
/// When the `.build()` method of the template is invoked, it creates an ECS enity what contains
/// a clone of the template, the generated view (as produced by `.create()`), and the state produced
/// by that view (as produced by `.build()`). The entity also contains a tracking scope and a
/// thunk, so that the template content can be rebuilt independently from the parent view.
pub trait ViewTemplate: Send + Sync + 'static {
    type View: View;

    /// Create the view for the control.
    fn create(&self, cx: &mut Cx) -> Self::View;
}

impl<VT: ViewTemplate + Clone + PartialEq> View for VT {
    type State = (Entity, NodeSpan);

    fn nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        #[cfg(feature = "verbose")]
        info!("nodes() {}", state.0);

        // TODO: This is not the same as ViewTemplateState::nodes() ??
        // One uses the cached copy, the other goes via the inner View.
        // state.1.clone()

        let entity = state.0;
        let entt = world.entity(entity);
        let cell = entt.get::<ViewTemplateStateCell<VT>>().unwrap();
        let inner = cell.0.lock().unwrap();
        inner.nodes(world)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let tick = cx.world_mut().change_tick();
        let parent = cx.owner();
        let child_entity = cx.world_mut().spawn_empty().set_parent(parent).id();

        #[cfg(feature = "verbose")]
        info!("build() {}", child_entity);

        let mut scope = TrackingScope::new(tick);
        let mut cx_inner = Cx::new(cx.world_mut(), child_entity, &mut scope);
        let view = self.create(&mut cx_inner);
        let state = view.build(&mut cx_inner);
        let nodes = view.nodes(cx.world(), &state);
        let cell = ViewTemplateState::new(self.clone(), view, state);
        let thunk = cell.create_thunk();
        cx.world_mut().entity_mut(child_entity).insert((
            ViewTemplateStateCell(Arc::new(Mutex::new(cell))),
            scope,
            thunk,
        ));
        (child_entity, nodes)
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let entity = state.0;

        #[cfg(feature = "verbose")]
        info!("rebuild() {}", entity);

        let mut entt = cx.world_mut().entity_mut(entity);
        let cell = entt.get::<ViewTemplateStateCell<VT>>().unwrap();
        let mut inner = cell.0.lock().unwrap();
        if inner.template != *self {
            // Update the template and trigger a rebuild on the child template.
            inner.template = self.clone();
            drop(inner);
            let scope = entt.get_mut::<TrackingScope>().unwrap();
            scope.set_changed();
        }

        // False because we haven't changed the output yet.
        false
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        let entity = state.0;

        #[cfg(feature = "verbose")]
        info!("attach_children() {}", entity);
        assert!(world.get_entity(entity).is_some());

        let entt = world.entity_mut(entity);
        let cell = entt.get::<ViewTemplateStateCell<VT>>().unwrap();
        let inner = cell.0.clone();
        let nodes = inner.lock().unwrap().nodes(world);
        if state.1 != nodes {
            state.1 = nodes;
            true
        } else {
            false
        }
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let entity = state.0;

        #[cfg(feature = "verbose")]
        info!("raze() {}", entity);

        let mut entt = world.entity_mut(entity);
        let cell = entt.take::<ViewTemplateStateCell<VT>>().unwrap();
        let mut inner = cell.0.lock().unwrap();
        inner.raze(world);
        let mut entt = world.entity_mut(entity);
        let mut scope = entt.take::<TrackingScope>().unwrap();
        scope.raze(world);
        world.entity_mut(entity).remove_parent();
        world.entity_mut(entity).despawn();
    }
}

struct ViewTemplateState<VT: ViewTemplate> {
    template: VT,
    view: VT::View,
    state: <VT::View as View>::State,
}

impl<VT: ViewTemplate> ViewTemplateState<VT> {
    fn new(template: VT, view: VT::View, state: <VT::View as View>::State) -> Self {
        Self {
            template,
            view,
            state,
        }
    }

    fn nodes(&self, world: &World) -> NodeSpan {
        self.view.nodes(world, &self.state)
    }

    fn rebuild(&mut self, cx: &mut Cx) -> bool {
        self.view = self.template.create(cx);
        self.view.rebuild(cx, &mut self.state)
    }

    fn raze(&mut self, world: &mut World) {
        // println!("Razing View Template: {}", std::any::type_name::<VT>());
        self.view.raze(world, &mut self.state);
    }

    fn attach_children(&mut self, world: &mut World) -> bool {
        self.view.attach_children(world, &mut self.state)
    }

    pub fn create_thunk(&self) -> ViewThunk {
        ViewThunk(&ViewTemplateAdapter::<VT> {
            marker: std::marker::PhantomData,
        })
    }
}

#[derive(Component)]
pub struct ViewTemplateStateCell<VF: ViewTemplate>(Arc<Mutex<ViewTemplateState<VF>>>);

impl<VT: ViewTemplate> ViewTemplateStateCell<VT> {
    fn nodes(&self, world: &World) -> NodeSpan {
        self.0.lock().unwrap().nodes(world)
    }

    pub fn raze(&self, world: &mut World) {
        self.0.lock().unwrap().raze(world);
    }

    pub fn attach_children(&self, world: &mut World) -> bool {
        self.0.lock().unwrap().attach_children(world)
    }
}

pub struct ViewTemplateAdapter<VF: ViewTemplate> {
    marker: std::marker::PhantomData<VF>,
}

impl<VF: ViewTemplate> AnyViewAdapter for ViewTemplateAdapter<VF> {
    fn nodes(&self, world: &mut World, entity: Entity) -> NodeSpan {
        match world.entity(entity).get::<ViewTemplateStateCell<VF>>() {
            Some(view_cell) => view_cell.nodes(world),
            None => NodeSpan::Empty,
        }
    }

    fn rebuild(&self, world: &mut World, entity: Entity, scope: &mut TrackingScope) -> bool {
        let mut cx = Cx::new(world, entity, scope);
        if let Some(view_cell) = cx
            .world_mut()
            .entity(entity)
            .get::<ViewTemplateStateCell<VF>>()
        {
            let vs = view_cell.0.clone();
            let mut inner = vs.lock().unwrap();
            inner.rebuild(&mut cx)
        } else {
            false
        }
    }

    fn attach_children(&self, world: &mut World, entity: Entity) -> bool {
        if let Some(view_cell) = world.entity(entity).get::<ViewTemplateStateCell<VF>>() {
            let vs = view_cell.0.clone();
            let mut inner = vs.lock().unwrap();
            inner.attach_children(world)
        } else {
            false
        }
    }

    fn raze(&self, world: &mut World, entity: Entity) {
        if let Some(view_cell) = world.entity_mut(entity).take::<ViewTemplateStateCell<VF>>() {
            view_cell.raze(world);
        }
    }
}

use bevy::{
    hierarchy::BuildWorldChildren,
    prelude::{Component, Entity, World},
};

use crate::{tracking_scope::TrackingScope, Cx, NodeSpan, View};

pub struct Memoize<V: View, Props> {
    props: Props,
    factory: fn(&mut Cx, &Props) -> V,
}

// Notes:
// There are two bundles of state we need to store:
// - the one in the memo
// - the one in the entity
//
// The one in the memo has:
// - the factory
// - the props
// This has an associated state which is the entity and the nodespan.
//
// The one in the entity has:
// The view
// The associated view state.

impl<V: View, Props> Memoize<V, Props> {
    pub fn new(props: Props, f: fn(&mut Cx, &Props) -> V) -> Self {
        Self { props, factory: f }
    }
}

#[derive(Component)]
pub struct MemoCell<V: View, Props> {
    props: Props,
    factory: fn(&mut Cx, &Props) -> V,
    view: V,
    state: V::State,
}

// impl<V: View, Props: Send + Sync + 'static + PartialEq> AnyViewState for Memo<V, Props> {
//     fn nodes(&self) -> NodeSpan {
//         match self.state {
//             Some(ref state) => state.0.nodes(&state.1),
//             None => NodeSpan::Empty,
//         }
//     }

//     fn rebuild(&mut self, cx: &mut Cx) -> bool {
//         let owner = cx.owner;
//         let mut owner_entt = cx.world_mut().entity_mut(owner);
//         let mut old_cell = owner_entt.get_mut::<Memo<V, Props>>().unwrap();
//         if old_cell.factory != self.factory || old_cell.props != self.props {
//             old_cell.factory = self.factory;
//             std::mem::swap(&mut self.props, &mut old_cell.props);
//             return true;
//         }
//         false
//     }

//     fn raze(&mut self, world: &mut World) {
//         if let Some(state) = &mut self.state {
//             state.0.raze(world, &mut state.1);
//             self.state = None;
//         }
//     }
// }

impl<V: View, Props: Send + Sync + 'static + Clone + PartialEq> View for Memoize<V, Props> {
    type State = (Entity, NodeSpan);

    fn nodes(&self, state: &Self::State) -> NodeSpan {
        state.1.clone()
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let tick = cx.world_mut().change_tick();
        let template_entity = cx.world_mut().spawn_empty().id();
        let mut scope = TrackingScope::new(tick);
        let mut cx_inner = Cx::new(cx.world_mut(), template_entity, &mut scope);
        let view = (self.factory)(&mut cx_inner, &self.props);
        let state = view.build(&mut cx_inner);
        let nodes = view.nodes(&state);
        let cell = MemoCell {
            props: self.props.clone(),
            factory: self.factory,
            view,
            state,
        };
        cx.world_mut()
            .entity_mut(template_entity)
            .insert((cell, scope));
        (template_entity, nodes)
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let world = cx.world_mut();
        // let tick = world.change_tick();
        // TODO: This is a bit of a hack. We should be able to get lookup multiple components
        // on a single entity without having to query all entities of that type.
        let mut query = world.query::<(&mut MemoCell<V, Props>, &mut TrackingScope)>();
        if let Ok((mut cell, mut scope)) = query.get_mut(world, state.0) {
            // if scope.dependencies_changed(world, tick) {
            //     return true;
            // }
            // let mut cx_inner = Cx::new(world, state.0, &mut scope);
            if self.props != cell.props {
                cell.props = self.props.clone();
                // let mut cxi = Cx::new(world, state.0, &mut scope);
                // let view = (cell.factory)(&mut cxi, &self.props);
                // cell.view = view;
                // if view.rebuild(&mut cxi, &mut cell.state) {
                // Mark tracking scope for rebuild.
                // cell.view.rebuild(cx.world_mut(), &mut cell.state);
                // cell.state = cell.view.build(cx);
                // cell.view.nodes(&cell.state);

                // let nodes = cell.view.nodes(&cell.state);
                // let mut entt = world.entity_mut(state.0);
                // let mut cell = entt.get_mut::<MemoCell<V, Props>>().unwrap();
                // cell.state = cell.view.build(&mut cx_inner);
                // cell.view.nodes(&cell.state);
                // return true;
                // }
            }
        }
        // self.0.rebuild(state.0, cx.world_mut())
        // let owner = cx.owner;
        // let mut owner_entt = cx.world_mut().entity_mut(state.0);
        // let mut old_cell = owner_entt.get_mut::<Memo<V, Props>>().unwrap();
        // if old_cell.factory != self.factory || old_cell.props != self.props {
        //     old_cell.factory = self.factory;
        //     std::mem::swap(&mut self.props, &mut old_cell.props);
        //     return true;
        // }
        false
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let entity = state.0;
        let mut entt = world.entity_mut(entity);
        let mut cell = entt.take::<MemoCell<V, Props>>().unwrap();
        cell.view.raze(world, &mut cell.state);
        world.entity_mut(entity).remove_parent();
        world.entity_mut(entity).despawn();
    }
}

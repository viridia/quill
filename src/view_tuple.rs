use crate::node_span::NodeSpan;
use crate::{Cx, View};
use bevy::ecs::world::World;
use impl_trait_for_tuples::*;

// ViewTuple

#[doc(hidden)]
pub trait ViewTuple: Send + Sync + 'static {
    /// Aggregate View::State for all tuple members.
    type State: Send + Sync;

    /// Return the number of child views.
    fn len(&self) -> usize;

    /// True if the tuple is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the output nodes for all spans.
    fn span_nodes(&self, world: &World, state: &Self::State) -> NodeSpan;

    /// Build the child views.
    fn build_spans(&self, cx: &mut Cx) -> Self::State;

    /// Update the child views.
    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool;

    /// Despawn the child views.
    fn raze_spans(&self, world: &mut World, state: &mut Self::State);

    /// Call `attach_descendants` on all child views.
    fn attach_descendants(&self, world: &mut World, state: &mut Self::State) -> bool;
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    #[inline(always)]
    fn span_nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        self.nodes(world, state)
    }

    #[inline(always)]
    fn build_spans(&self, cx: &mut Cx) -> Self::State {
        self.build(cx)
    }

    #[inline(always)]
    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        self.rebuild(cx, state)
    }

    #[inline(always)]
    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {
        self.raze(world, state)
    }

    #[inline(always)]
    fn attach_descendants(&self, world: &mut World, state: &mut Self::State) -> bool {
        self.attach_children(world, state)
    }
}

#[allow(unused)]
impl ViewTuple for () {
    type State = ();

    #[inline(always)]
    fn len(&self) -> usize {
        0
    }

    #[inline(always)]
    fn span_nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    #[inline(always)]
    fn build_spans(&self, cx: &mut Cx) -> Self::State {}

    #[inline(always)]
    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        false
    }

    #[inline(always)]
    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {}

    #[inline(always)]
    fn attach_descendants(&self, world: &mut World, state: &mut Self::State) -> bool {
        false
    }
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(View)]
impl ViewTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    #[inline(always)]
    fn len(&self) -> usize {
        for_tuples!((#( 1 )+*))
    }

    #[rustfmt::skip]
    fn span_nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.nodes(world, &state.Tuple) ),*)
        ]))
    }

    fn build_spans(&self, cx: &mut Cx) -> Self::State {
        for_tuples!((#( self.Tuple.build(cx) ),*))
    }

    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let mut changed = false;
        for_tuples!(#( changed |= self.Tuple.rebuild(cx, &mut state.Tuple); )*);
        changed
    }

    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.raze(world, &mut state.Tuple); )*)
    }

    fn attach_descendants(&self, world: &mut World, state: &mut Self::State) -> bool {
        let mut changed = false;
        for_tuples!(#( changed |= self.Tuple.attach_children(world, &mut state.Tuple); )*);
        changed
    }
}

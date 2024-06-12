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
    fn span_nodes(&self, state: &Self::State) -> NodeSpan;

    /// Build the child views.
    fn build_spans(&self, cx: &mut Cx) -> Self::State;

    /// Update the child views.
    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool;

    /// Assemble the child views.
    // fn assemble_spans(&self, cx: &mut Cx, state: &mut Self::State) -> NodeSpan;

    /// Despawn the child views.
    fn raze_spans(&self, world: &mut World, state: &mut Self::State);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn span_nodes(&self, state: &Self::State) -> NodeSpan {
        self.nodes(state)
    }

    fn build_spans(&self, cx: &mut Cx) -> Self::State {
        self.build(cx)
    }

    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        self.rebuild(cx, state)
    }

    // fn assemble_spans(&self, cx: &mut Cx, state: &mut Self::State) -> NodeSpan {
    //     self.assemble(cx, state)
    // }

    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {
        self.raze(world, state)
    }
}

#[allow(unused)]
impl ViewTuple for () {
    type State = ();

    fn len(&self) -> usize {
        0
    }

    fn span_nodes(&self, state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    fn build_spans(&self, cx: &mut Cx) -> Self::State {}

    fn rebuild_spans(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        false
    }

    // fn assemble_spans(&self, cx: &mut Cx, state: &mut Self::State) -> NodeSpan {
    //     self.assemble(cx, state)
    // }

    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {}
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(View)]
impl ViewTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn len(&self) -> usize {
        for_tuples!((#( 1 )+*))
    }

    #[rustfmt::skip]
    fn span_nodes(&self, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.nodes(&state.Tuple) ),*)
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
}

use crate::node_span::NodeSpan;
use crate::{BuildContext, View};
use bevy::ecs::world::World;
use impl_trait_for_tuples::*;

// ViewTuple

#[doc(hidden)]
pub trait ViewTuple: Send {
    /// Aggregate View::State for all tuple members.
    type State: Send;

    /// Return the number of child views.
    fn len(&self) -> usize;

    /// True if the tuple is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the output nodes for all spans.
    fn span_nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan;

    /// Build the child views.
    fn build_spans(&self, bc: &mut BuildContext) -> Self::State;

    /// Update the child views.
    fn update_spans(&self, bc: &mut BuildContext, state: &mut Self::State);

    /// Assemble the child views.
    fn assemble_spans(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan;

    /// Despawn the child views.
    fn raze_spans(&self, world: &mut World, state: &mut Self::State);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn span_nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.nodes(bc, state)
    }

    fn build_spans(&self, bc: &mut BuildContext) -> Self::State {
        self.build(bc)
    }

    fn update_spans(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.update(bc, state)
    }

    fn assemble_spans(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.assemble(bc, state)
    }

    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {
        self.raze(world, state)
    }
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(View)]
impl ViewTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn len(&self) -> usize {
        for_tuples!((#( 1 )+*))
    }

    #[rustfmt::skip]
    fn span_nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.nodes(bc, &state.Tuple) ),*)
        ]))
    }

    fn build_spans(&self, bc: &mut BuildContext) -> Self::State {
        for_tuples!((#( self.Tuple.build(bc) ),*))
    }

    fn update_spans(&self, bc: &mut BuildContext, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.update(bc, &mut state.Tuple); )*)
    }

    #[rustfmt::skip]
    fn assemble_spans(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.assemble(bc, &mut state.Tuple) ),*)
        ]))
    }

    fn raze_spans(&self, world: &mut World, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.raze(world, &mut state.Tuple); )*)
    }
}

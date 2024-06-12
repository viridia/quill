use crate::Cx;
use bevy::prelude::*;
use impl_trait_for_tuples::*;

#[allow(unused)]
/// A reactive effect that modifies a target entity.
pub trait EntityEffect: Sync + Send {
    type State: Send + Sync;

    /// Apply the effect to the target entity.
    ///
    /// Arguments:
    /// - `cx`: The reactive context
    /// - `target`: The display entity that will be styled.
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State;

    /// Re-apply the effect to the target entity.
    ///
    /// Arguments:
    /// - `cx`: The reactive context
    /// - `target`: The display entity that will be styled.
    /// - `state`: The state returned by the previous call to `apply`.
    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {}
}

#[doc(hidden)]
pub trait EffectTuple: Send + Sync {
    /// Aggregate EntityEffect::State for all tuple members.
    type State: Send + Sync;

    /// Return the number of effects.
    fn len(&self) -> usize;

    /// True if the tuple is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Apply the effects to the target.
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State;
}

impl<E: EntityEffect> EffectTuple for E {
    type State = E::State;

    fn len(&self) -> usize {
        1
    }

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        self.apply(cx, target)
    }
}

#[allow(unused)]
impl EffectTuple for () {
    type State = ();

    fn len(&self) -> usize {
        0
    }

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {}
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(EntityEffect)]
impl EffectTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn len(&self) -> usize {
        for_tuples!((#( 1 )+*))
    }

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        for_tuples!((#( self.Tuple.apply(cx, target) ),*))
    }
}

use crate::Cx;
use bevy::prelude::*;
use impl_trait_for_tuples::impl_for_tuples;

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
pub trait AppendEffect<T> {
    type Result: EffectTuple;

    fn append(self, tail: T) -> Self::Result;
}

#[allow(unused)]
impl<T: EntityEffect> AppendEffect<T> for () {
    type Result = (T,);

    fn append(self, tail: T) -> Self::Result {
        (tail,)
    }
}

impl<H1: EntityEffect, T: EntityEffect> AppendEffect<T> for (H1,) {
    type Result = (H1, T);

    fn append(self, tail: T) -> Self::Result {
        (self.0, tail)
    }
}

impl<H1: EntityEffect, H2: EntityEffect, T: EntityEffect> AppendEffect<T> for (H1, H2) {
    type Result = (H1, H2, T);

    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, tail)
    }
}

impl<H1: EntityEffect, H2: EntityEffect, H3: EntityEffect, T: EntityEffect> AppendEffect<T>
    for (H1, H2, H3)
{
    type Result = (H1, H2, H3, T);

    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, self.2, tail)
    }
}

#[doc(hidden)]
pub trait EffectTuple: Send + Sync {
    /// Aggregate EntityEffect::State for all tuple members.
    type State: Send + Sync;

    /// Apply the effects to the target.
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State;

    // Append a new effect to the tuple.
    fn append_effect<E: EntityEffect>(self, effect: E) -> <Self as AppendEffect<E>>::Result
    where
        Self: Sized + AppendEffect<E>,
    {
        self.append(effect)
    }
}

impl<E: EntityEffect> EffectTuple for E {
    type State = E::State;

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        self.apply(cx, target)
    }

    // fn append<E1: EntityEffect>(self, effect: E1) -> <(E, E1) as ConcatTuple<E>>::Result {
    //     (self, effect)
    // }
}

#[allow(unused)]
impl EffectTuple for () {
    type State = ();

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {}
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(EntityEffect)]
impl EffectTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        for_tuples!((#( self.Tuple.apply(cx, target) ),*))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEffect;

    impl EntityEffect for TestEffect {
        type State = ();

        fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {}
    }

    struct TestEffect2;

    impl EntityEffect for TestEffect2 {
        type State = ();

        fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {}
    }

    struct EffectList<T: EffectTuple>(T);

    impl<T: EffectTuple> EffectList<T> {
        fn append<E: EntityEffect>(self, effect: E) -> EffectList<<T as AppendEffect<E>>::Result>
        where
            Self: Sized,
            T: AppendEffect<E>,
        {
            let effects = self.0.append_effect(effect);
            EffectList(effects)
        }
    }

    #[test]
    fn test_append() {
        let effects = EffectList((TestEffect,));
        let effects2 = EffectList(effects.0.append(TestEffect2));
        let effects = (TestEffect, TestEffect2);
        let _effects = effects.append(TestEffect2);
    }
}

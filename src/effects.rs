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
    /// - `target`: The display entity that the effect will apply to.
    ///
    /// Returns:
    /// - The state of the effect.
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State;

    /// Re-apply the effect to the target entity.
    ///
    /// Arguments:
    /// - `cx`: The reactive context
    /// - `target`: The display entity that the effect will apply to.
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

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (tail,)
    }
}

impl<H1: EntityEffect, T: EntityEffect> AppendEffect<T> for (H1,) {
    type Result = (H1, T);

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (self.0, tail)
    }
}

impl<H1: EntityEffect, H2: EntityEffect, T: EntityEffect> AppendEffect<T> for (H1, H2) {
    type Result = (H1, H2, T);

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, tail)
    }
}

impl<H1: EntityEffect, H2: EntityEffect, H3: EntityEffect, T: EntityEffect> AppendEffect<T>
    for (H1, H2, H3)
{
    type Result = (H1, H2, H3, T);

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, self.2, tail)
    }
}

impl<H1: EntityEffect, H2: EntityEffect, H3: EntityEffect, H4: EntityEffect, T: EntityEffect>
    AppendEffect<T> for (H1, H2, H3, H4)
{
    type Result = (H1, H2, H3, H4, T);

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, self.2, self.3, tail)
    }
}

impl<
        H1: EntityEffect,
        H2: EntityEffect,
        H3: EntityEffect,
        H4: EntityEffect,
        H5: EntityEffect,
        T: EntityEffect,
    > AppendEffect<T> for (H1, H2, H3, H4, H5)
{
    type Result = (H1, H2, H3, H4, H5, T);

    #[inline(always)]
    fn append(self, tail: T) -> Self::Result {
        (self.0, self.1, self.2, self.3, self.4, tail)
    }
}

#[doc(hidden)]
pub trait EffectTuple: Send + Sync {
    /// Aggregate EntityEffect::State for all tuple members.
    type State: Send + Sync;

    /// Apply the effects to the target.
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State;

    /// Re-apply the effects to the target.
    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State);

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

    #[inline(always)]
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        self.apply(cx, target)
    }

    #[inline(always)]
    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        self.reapply(cx, target, state)
    }
}

#[allow(unused)]
impl EffectTuple for () {
    type State = ();

    #[inline(always)]
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {}

    #[inline(always)]
    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {}
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(EntityEffect)]
impl EffectTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        for_tuples!((#( self.Tuple.apply(cx, target) ),*))
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.reapply(cx, target, &mut state.Tuple);)*)
    }
}

/// A general-purpose effect that allows arbitrary mutations to the display entity.
pub struct CallbackEffect<F: Fn(&mut Cx, Entity, D), D: PartialEq + Clone> {
    pub(crate) effect_fn: F,
    pub(crate) deps: D,
}

impl<F: Fn(&mut Cx, Entity, D) + Send + Sync, D: PartialEq + Clone + Send + Sync> EntityEffect
    for CallbackEffect<F, D>
{
    type State = D;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        (self.effect_fn)(cx, target, self.deps.clone());
        self.deps.clone()
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.deps {
            *state = self.deps.clone();
            (self.effect_fn)(cx, target, self.deps.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEffect;

    impl EntityEffect for TestEffect {
        type State = ();

        fn apply(&self, _cx: &mut Cx, _target: Entity) -> Self::State {}
    }

    struct TestEffect2;

    impl EntityEffect for TestEffect2 {
        type State = ();

        fn apply(&self, _cx: &mut Cx, _target: Entity) -> Self::State {}
    }

    struct EffectList<T: EffectTuple>(T);

    impl<T: EffectTuple> EffectList<T> {
        #[allow(unused)]
        fn append<E: EntityEffect>(self, effect: E) -> EffectList<<T as AppendEffect<E>>::Result>
        where
            T: AppendEffect<E>,
        {
            let effects = self.0.append_effect(effect);
            EffectList(effects)
        }
    }

    #[test]
    fn test_append() {
        // Compilation test
        let effects = EffectList((TestEffect,));
        let _effects2 = EffectList(effects.0.append(TestEffect2));
    }
}

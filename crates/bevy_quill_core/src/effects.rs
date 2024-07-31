use crate::Cx;
use bevy::prelude::*;

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

macro_rules! impl_append_effect {
    ( $($view: ident, $idx: tt);+ ) => {
        impl<$(
            $view: EntityEffect,
        )+ T: EntityEffect> AppendEffect<T> for ( $( $view, )+ ) {
            type Result = ($( $view, )+ T);

            #[inline(always)]
            fn append(self, tail: T) -> Self::Result {
                ( $( self.$idx, )+ tail)
            }
        }
    };
}

impl_append_effect!(E0, 0);
impl_append_effect!(E0, 0; E1, 1);
impl_append_effect!(E0, 0; E1, 1; E2, 2);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14);
impl_append_effect!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15);

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

macro_rules! impl_effect_tuple {
    ( $($effect: ident, $idx: tt);+ ) => {
        impl<$(
            $effect: EntityEffect,
        )+> EffectTuple for ( $( $effect, )+ ) {
            type State = ( $( $effect::State ),*, );

            fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
                ($( self.$idx.apply(cx, target) ),*,)
            }

            fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
                $( self.$idx.reapply(cx, target, &mut state.$idx); )*
            }
        }
    };
}

impl_effect_tuple!(E0, 0);
impl_effect_tuple!(E0, 0; E1, 1);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15; E16, 16);

/// A general-purpose effect that allows arbitrary mutations to the display entity.
pub struct CallbackEffect<F: Fn(&mut Cx, Entity, D), D: PartialEq + Clone> {
    pub effect_fn: F,
    pub deps: D,
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

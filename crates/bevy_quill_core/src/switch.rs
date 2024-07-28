use crate::{AnyView, BoxedState, Cx, View};
use bevy::ecs::world::{DeferredWorld, World};
use bevy::prelude::Entity;

/// A tuple of cases for a switch view.
pub trait CaseTuple<Value>: Send + Sync {
    /// Return the index of the case that matches the given value.
    fn find(&self, value: &Value) -> Option<usize>;

    /// Return the view for the case at the given index. Panics if the index is out of bounds.
    fn at(&self, index: usize) -> &dyn AnyView;
}

impl<Value> CaseTuple<Value> for () {
    fn find(&self, _value: &Value) -> Option<usize> {
        None
    }

    fn at(&self, _index: usize) -> &dyn AnyView {
        unreachable!();
    }
}

macro_rules! impl_case_tuple {
    ( $($view: ident, $idx: tt);+ ) => {
        impl<Value: Send + Sync + PartialEq, $(
            $view: View,
        )+> CaseTuple<Value> for ( $( (Value, $view), )+ ) {
            fn find(&self, value: &Value) -> Option<usize> {
                match value {
                    $( v if *v == self.$idx .0 => Some($idx), )+
                    _ => None,
                }
            }

            fn at(&self, index: usize) -> &dyn AnyView {
                match index {
                    $( $idx => &self.$idx .1, )+
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_case_tuple!(V0, 0);
impl_case_tuple!(V0, 0; V1, 1);
impl_case_tuple!(V0, 0; V1, 1; V2, 2);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13; V14, 14);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13; V14, 14; V15, 15);
impl_case_tuple!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13; V14, 14; V15, 15; V16, 16);

#[doc(hidden)]
pub trait AppendCase<V, T> {
    type Result: CaseTuple<V>;

    fn append(self, value: V, tail: T) -> Self::Result;
}

#[allow(unused)]
impl<V: Send + Sync + PartialEq, T: View> AppendCase<V, T> for () {
    type Result = ((V, T),);

    #[inline(always)]
    fn append(self, value: V, tail: T) -> Self::Result {
        ((value, tail),)
    }
}

macro_rules! impl_case_append {
    ( $($view: ident, $idx: tt);+ ) => {
        impl<V: Send + Sync + PartialEq, $(
            $view: View,
        )+ T: View> AppendCase<V, T> for ( $( (V, $view), )+ ) {
            type Result = ($( (V, $view), )+ (V, T));

            #[inline(always)]
            fn append(self, value: V, tail: T) -> Self::Result {
                ( $( self.$idx, )+ (value, tail))
            }
        }
    };
}

impl_case_append!(V0, 0);
impl_case_append!(V0, 0; V1, 1);
impl_case_append!(V0, 0; V1, 1; V2, 2);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13; V14, 14);
impl_case_append!(V0, 0; V1, 1; V2, 2; V3, 3; V4, 4; V5, 5; V6, 6; V7, 7; V8, 8; V9, 9; V10, 10; V11, 11; V12, 12; V13, 13; V14, 14; V15, 15);

/// A conditional view which renders one of two children depending on the condition expression.
pub struct Switch<Value, Cases: CaseTuple<Value>, Fallback: View> {
    value: Value,
    cases: Cases,
    fallback: Fallback,
}

impl<Value> Switch<Value, (), ()> {
    /// Construct a new Switch View.
    pub fn new(value: Value) -> Self {
        Self {
            value,
            cases: (),
            fallback: (),
        }
    }
}

impl<Value: Send + Sync + PartialEq, Cases: CaseTuple<Value>, Fallback: View>
    Switch<Value, Cases, Fallback>
{
    pub fn case<CV: View>(
        self,
        value: Value,
        case: CV,
    ) -> Switch<Value, <Cases as AppendCase<Value, CV>>::Result, Fallback>
    where
        Cases: AppendCase<Value, CV>,
    {
        Switch {
            value: self.value,
            cases: self.cases.append(value, case),
            fallback: self.fallback,
        }
    }

    pub fn fallback<F: View>(self, fallback: F) -> Switch<Value, Cases, F> {
        Switch {
            value: self.value,
            cases: self.cases,
            fallback,
        }
    }
}

impl<
        Value: Send + Sync + PartialEq + Clone + 'static,
        Cases: CaseTuple<Value> + 'static,
        Fallback: View,
    > View for Switch<Value, Cases, Fallback>
{
    type State = (Option<usize>, BoxedState);

    fn nodes(&self, world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        match state {
            (Some(index), ref state) => self.cases.at(*index).nodes(world, state, out),
            (None, ref state) => {
                if let Some(state) = state.downcast_ref::<Fallback::State>() {
                    self.fallback.nodes(world, state, out)
                }
            }
        }
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let index = self.cases.find(&self.value);
        match index {
            Some(ndx) => (Some(ndx), self.cases.at(ndx).build(cx)),
            None => (None, Box::new(self.fallback.build(cx))),
        }
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let index = self.cases.find(&self.value);
        match (state.0, index) {
            (Some(index), Some(new_index)) if index == new_index => {
                self.cases.at(index).rebuild(cx, &mut state.1)
            }

            (Some(index), Some(new_index)) => {
                self.cases
                    .at(index)
                    .raze(&mut DeferredWorld::from(cx.world_mut()), &mut state.1);
                state.0 = Some(new_index);
                state.1 = self.cases.at(new_index).build(cx);
                true
            }

            (Some(index), None) => {
                self.cases
                    .at(index)
                    .raze(&mut DeferredWorld::from(cx.world_mut()), &mut state.1);
                state.0 = None;
                state.1 = Box::new(self.fallback.build(cx));
                true
            }

            (None, Some(new_index)) => {
                state.0 = Some(new_index);
                state.1 = self.cases.at(new_index).build(cx);
                true
            }

            (None, None) => {
                if let Some(st) = state.1.downcast_mut::<Fallback::State>() {
                    self.fallback.rebuild(cx, st)
                } else {
                    false
                }
            }
        }
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        match state.0 {
            Some(index) => self.cases.at(index).attach_children(world, &mut state.1),

            None => {
                if let Some(st) = state.1.downcast_mut::<Fallback::State>() {
                    self.fallback.attach_children(world, st)
                } else {
                    false
                }
            }
        }
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        match state.0 {
            Some(index) => self.cases.at(index).raze(world, &mut state.1),
            None => {
                if let Some(st) = state.1.downcast_mut::<Fallback::State>() {
                    self.fallback.raze(world, st)
                }
            }
        }
    }
}

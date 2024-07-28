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

impl<Value: Send + Sync + PartialEq, V: View> CaseTuple<Value> for ((Value, V),) {
    fn find(&self, value: &Value) -> Option<usize> {
        if self.0 .0 == *value {
            Some(0)
        } else {
            None
        }
    }

    fn at(&self, index: usize) -> &dyn AnyView {
        if index == 0 {
            &self.0 .1
        } else {
            unreachable!();
        }
    }
}

impl<Value: Send + Sync + PartialEq, V1: View, V2: View> CaseTuple<Value>
    for ((Value, V1), (Value, V2))
{
    fn find(&self, value: &Value) -> Option<usize> {
        if self.0 .0 == *value {
            Some(0)
        } else if self.1 .0 == *value {
            Some(1)
        } else {
            None
        }
    }

    fn at(&self, index: usize) -> &dyn AnyView {
        match index {
            0 => &self.0 .1,
            1 => &self.1 .1,
            _ => unreachable!(),
        }
    }
}

impl<Value: Send + Sync + PartialEq, V1: View, V2: View, V3: View> CaseTuple<Value>
    for ((Value, V1), (Value, V2), (Value, V3))
{
    fn find(&self, value: &Value) -> Option<usize> {
        match value {
            v if *v == self.0 .0 => Some(0),
            v if *v == self.1 .0 => Some(1),
            v if *v == self.2 .0 => Some(2),
            _ => None,
        }
    }

    fn at(&self, index: usize) -> &dyn AnyView {
        match index {
            0 => &self.0 .1,
            1 => &self.1 .1,
            2 => &self.2 .1,
            _ => unreachable!(),
        }
    }
}

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

impl<V: Send + Sync + PartialEq, V1: View, T: View> AppendCase<V, T> for ((V, V1),) {
    type Result = ((V, V1), (V, T));

    #[inline(always)]
    fn append(self, value: V, tail: T) -> Self::Result {
        (self.0, (value, tail))
    }
}

impl<V: Send + Sync + PartialEq, V1: View, V2: View, T: View> AppendCase<V, T>
    for ((V, V1), (V, V2))
{
    type Result = ((V, V1), (V, V2), (V, T));

    #[inline(always)]
    fn append(self, value: V, tail: T) -> Self::Result {
        (self.0, self.1, (value, tail))
    }
}

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

use bevy::ecs::world::{DeferredWorld, World};
use bevy::prelude::*;

use crate::{Cx, View};

// Cond

pub enum CondState<Pos, Neg> {
    True(Pos),
    False(Neg),
}

/// A conditional view which renders one of two children depending on the condition expression.
pub struct Cond<Pos: View, Neg: View> {
    test: bool,
    pos: Pos,
    neg: Neg,
}

impl<Pos: View, Neg: View> Cond<Pos, Neg> {
    /// Construct a new `Cond` View.
    pub fn new(test: bool, pos: Pos, neg: Neg) -> Self {
        Self { test, pos, neg }
    }
}

impl<Pos: View, Neg: View> View for Cond<Pos, Neg> {
    /// Union of true and false states.
    type State = CondState<Pos::State, Neg::State>;

    fn nodes(&self, world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        #[cfg(feature = "verbose")]
        info!("nodes()");

        match state {
            Self::State::True(ref true_state) => self.pos.nodes(world, true_state, out),
            Self::State::False(ref false_state) => self.neg.nodes(world, false_state, out),
        }
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        #[cfg(feature = "verbose")]
        info!("build()");

        if self.test {
            CondState::True(self.pos.build(cx))
        } else {
            CondState::False(self.neg.build(cx))
        }
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        #[cfg(feature = "verbose")]
        info!("rebuild()");

        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.rebuild(cx, true_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(&mut DeferredWorld::from(cx.world_mut()), state);
                    *state = Self::State::True(self.pos.build(cx));
                    true
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.rebuild(cx, false_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(&mut DeferredWorld::from(cx.world_mut()), state);
                    *state = Self::State::False(self.neg.build(cx));
                    true
                }
            }
        }
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        #[cfg(feature = "verbose")]
        info!("attach_children()",);

        match state {
            Self::State::True(ref mut true_state) => self.pos.attach_children(world, true_state),
            Self::State::False(ref mut false_state) => self.neg.attach_children(world, false_state),
        }
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("raze()");

        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(world, true_state),
            Self::State::False(ref mut false_state) => self.neg.raze(world, false_state),
        }
    }
}

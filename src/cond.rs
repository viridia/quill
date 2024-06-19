use bevy::ecs::world::World;

use crate::{Cx, View};

use crate::node_span::NodeSpan;

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
    /// Construct a new If View.
    pub fn new(test: bool, pos: Pos, neg: Neg) -> Self {
        Self { test, pos, neg }
    }
}

impl<Pos: View, Neg: View> View for Cond<Pos, Neg> {
    /// Union of true and false states.
    type State = CondState<Pos::State, Neg::State>;

    fn nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref true_state) => self.pos.nodes(world, true_state),
            Self::State::False(ref false_state) => self.neg.nodes(world, false_state),
        }
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        if self.test {
            CondState::True(self.pos.build(cx))
        } else {
            CondState::False(self.neg.build(cx))
        }
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.rebuild(cx, true_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(cx.world_mut(), state);
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
                    self.raze(cx.world_mut(), state);
                    *state = Self::State::False(self.neg.build(cx));
                    true
                }
            }
        }
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        match state {
            Self::State::True(ref mut true_state) => self.pos.attach_children(world, true_state),
            Self::State::False(ref mut false_state) => self.neg.attach_children(world, false_state),
        }
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        // match state {
        //     Self::State::True(_) => {
        //         println!("Razing Cond: True");
        //     }
        //     Self::State::False(_) => {
        //         println!("Razing Cond: False");
        //     }
        // }
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(world, true_state),
            Self::State::False(ref mut false_state) => self.neg.raze(world, false_state),
        }
    }
}

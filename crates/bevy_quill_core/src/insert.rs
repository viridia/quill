use bevy::prelude::{Bundle, Component, Entity};

use crate::{effects::EntityEffect, Cx};

/// Inserts a bundle into the target. If the deps change, then the bundle will be recomputed
/// and reinserted.
pub struct InsertBundleEffect<B: Bundle, F: Fn(D) -> B, D: PartialEq + Clone> {
    pub factory: F,
    pub deps: D,
}

impl<B: Bundle, F: Fn(D) -> B + Send + Sync, D: PartialEq + Clone + Send + Sync> EntityEffect
    for InsertBundleEffect<B, F, D>
{
    type State = D;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let mut target = cx.world_mut().entity_mut(target);
        target.insert((self.factory)(self.deps.clone()));
        self.deps.clone()
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.deps {
            *state = self.apply(cx, target);
        }
    }
}

/// Conditionally inserts a bundle into the target. If the condition is true, then the bundle
/// will be inserted. If the condition later becomes false, the component will be removed.
pub struct ConditionalInsertComponentEffect<B: Bundle, F: Fn() -> B> {
    pub factory: F,
    pub condition: bool,
}

impl<C: Component, F: Fn() -> C + Send + Sync> EntityEffect
    for ConditionalInsertComponentEffect<C, F>
{
    type State = bool;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        if self.condition {
            let mut target = cx.world_mut().entity_mut(target);
            target.insert((self.factory)());
        }
        self.condition
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if self.condition != *state {
            *state = self.condition;
            if self.condition {
                self.apply(cx, target);
            } else {
                let mut target = cx.world_mut().entity_mut(target);
                target.remove::<C>();
            }
        }
    }
}

/// Inserts a bundle into the target once and never updates it.
pub struct StaticInsertBundleEffect<B: Bundle + Clone> {
    pub bundle: B,
}

impl<B: Bundle + Clone> EntityEffect for StaticInsertBundleEffect<B> {
    type State = ();
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let mut target = cx.world_mut().entity_mut(target);
        target.insert(self.bundle.clone());
    }

    fn reapply(&self, _cx: &mut Cx, _target: Entity, _state: &mut Self::State) {}
}

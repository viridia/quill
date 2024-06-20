use bevy::prelude::{Bundle, Entity};

use crate::{effects::EntityEffect, Cx};

// /// Inserts a static, pre-constructed bundle into the target entity. No reactivity.
// pub struct ApplyInserBundleEffect<B: Bundle> {
//     pub(crate) bundle: B,
// }

// impl<B: Bundle> EntityEffect for ApplyInserBundleEffect<B> {
//     type State = ();
//     fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
//         let mut entt = cx.world_mut().entity_mut(target);
//         entt.insert(self.bundle);
//     }
// }

/// Applies dynamic styles which are computed reactively. The `deps` field is used to determine
/// whether the styles need to be recomputed; if the deps have not changed since the previous
/// update cycle, then the styles are not recomputed.
pub struct InsertBundleEffect<B: Bundle, F: Fn(D) -> B, D: PartialEq + Clone> {
    pub(crate) factory: F,
    pub(crate) deps: D,
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

/// Applies dynamic styles which are computed reactively. The `deps` field is used to determine
/// whether the styles need to be recomputed; if the deps have not changed since the previous
/// update cycle, then the styles are not recomputed.
pub struct ConditionalInsertBundleEffect<B: Bundle, F: Fn() -> B> {
    pub(crate) factory: F,
    pub(crate) condition: bool,
}

impl<B: Bundle, F: Fn() -> B + Send + Sync> EntityEffect for ConditionalInsertBundleEffect<B, F> {
    type State = ();
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        if self.condition {
            let mut target = cx.world_mut().entity_mut(target);
            target.insert((self.factory)());
        }
    }

    fn reapply(&self, _cx: &mut Cx, _target: Entity, _state: &mut Self::State) {}
}

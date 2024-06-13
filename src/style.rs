use bevy::{prelude::Entity, ui};
use bevy_mod_stylebuilder::{StyleBuilder, StyleTuple};

use crate::{effects::EntityEffect, Cx};

/// Inserts a static, pre-constructed bundle into the target entity. No reactivity.
pub struct ApplyStaticStylesEffect<S: StyleTuple> {
    pub(crate) styles: S,
}

impl<S: StyleTuple> EntityEffect for ApplyStaticStylesEffect<S> {
    type State = ();
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let mut target = cx.world_mut().entity_mut(target);
        let mut style = ui::Style::default();
        if let Some(s) = target.get::<ui::Style>() {
            style.clone_from(s);
        }
        let mut sb = StyleBuilder::new(&mut target, style);
        self.styles.apply(&mut sb);
        sb.finish();
    }
}

/// Applies dynamic styles which are computed reactively. The `deps` field is used to determine
/// whether the styles need to be recomputed; if the deps have not changed since the previous
/// update cycle, then the styles are not recomputed.
pub struct ApplyDynamicStylesEffect<F: Fn(&mut StyleBuilder), D: PartialEq + Clone> {
    pub(crate) style_fn: F,
    pub(crate) deps: D,
}

impl<F: Fn(&mut StyleBuilder) + Send + Sync, D: PartialEq + Clone + Send + Sync> EntityEffect
    for ApplyDynamicStylesEffect<F, D>
{
    type State = D;
    fn apply(&self, cx: &mut Cx, target: Entity) -> Self::State {
        let mut target = cx.world_mut().entity_mut(target);
        let mut style = ui::Style::default();
        if let Some(s) = target.get::<ui::Style>() {
            style.clone_from(s);
        }
        let mut sb = StyleBuilder::new(&mut target, style);
        (self.style_fn)(&mut sb);
        sb.finish();
        self.deps.clone()
    }

    fn reapply(&self, cx: &mut Cx, target: Entity, state: &mut Self::State) {
        if *state != self.deps {
            *state = self.apply(cx, target);
        }
    }
}

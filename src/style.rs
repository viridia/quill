use bevy::{prelude::Entity, ui};
use bevy_mod_stylebuilder::{StyleBuilder, StyleTuple};

use crate::{effects::EntityEffect, Cx};

/// Inserts a static, pre-constructed bundle into the target entity. No reactivity.
pub struct ApplyStylesEffect<S: StyleTuple> {
    pub(crate) styles: S,
}

impl<S: StyleTuple> EntityEffect for ApplyStylesEffect<S> {
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

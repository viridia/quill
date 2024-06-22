use bevy::{
    a11y::Focus,
    ecs::{entity::Entity, world::World},
    hierarchy::Parent,
};
use bevy_quill::Cx;

use crate::focus::FocusVisible;

/// True if the given entity is a descendant of the given ancestor.
fn is_descendant(world: &World, e: &Entity, ancestor: &Entity) -> bool {
    let mut ha = e;
    loop {
        if ha == ancestor {
            return true;
        }
        match world.get_entity(*ha).map(|e| e.get::<Parent>()) {
            Some(Some(parent)) => ha = parent,
            _ => return false,
        }
    }
}

/// Method to create a signal that tracks whether a target entity has focus.
pub trait UseIsFocus {
    /// Signal that returns true when the the target has focus.
    fn is_focused(&mut self, target: Entity) -> bool;

    /// Signal that returns true when the the target, or a descendant, has focus.
    fn is_focus_within(&mut self, target: Entity) -> bool;

    /// Signal that returns true when the the target has focus and the focus ring is visible.
    fn is_focus_visible(&mut self, target: Entity) -> bool;

    /// Signal that returns true when the the target, or a descendant, has focus, and the
    /// focus ring is visible.
    fn is_focus_within_visible(&mut self, target: Entity) -> bool;
}

impl<'p, 'w> UseIsFocus for Cx<'p, 'w> {
    fn is_focused(&mut self, target: Entity) -> bool {
        let focus = self.use_resource::<Focus>();
        focus.0 == Some(target)
    }

    fn is_focus_within(&mut self, target: Entity) -> bool {
        let focus = self.use_resource::<Focus>();
        match focus.0 {
            Some(focus) => is_descendant(self.world(), &focus, &target),
            None => false,
        }
    }

    fn is_focus_visible(&mut self, target: Entity) -> bool {
        let visible = self.use_resource::<FocusVisible>();
        let focus = self.use_resource::<Focus>();
        visible.0 && focus.0 == Some(target)
    }

    fn is_focus_within_visible(&mut self, target: Entity) -> bool {
        let visible = self.use_resource::<FocusVisible>();
        if !visible.0 {
            return false;
        }
        let focus = self.use_resource::<Focus>();
        match focus.0 {
            Some(focus) => is_descendant(self.world(), &focus, &target),
            None => false,
        }
    }
}

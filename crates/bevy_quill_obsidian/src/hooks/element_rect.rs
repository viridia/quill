use bevy::prelude::*;
use bevy_quill_core::Cx;

/// Trait which adds `use_element_rect` to [`Cx`].
pub trait UseElementRect {
    /// Returns the logical rect of the element with the given `id`.
    fn use_element_rect(&mut self, id: Entity) -> Rect;
}

impl<'p, 'w> UseElementRect for Cx<'p, 'w> {
    fn use_element_rect(&mut self, id: Entity) -> Rect {
        match (
            self.use_component::<Node>(id),
            self.use_component_untracked::<GlobalTransform>(id),
        ) {
            (Some(node), Some(transform)) => node.logical_rect(transform),
            _ => Rect::new(0., 0., 0., 0.),
        }
    }
}

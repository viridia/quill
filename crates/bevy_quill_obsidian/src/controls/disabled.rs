use bevy::prelude::{Component, Entity, World};
use bevy_quill::Cx;

/// A marker component to indicate that a widget is disabled.
#[derive(Component, Debug, Clone, Copy)]
pub struct Disabled;

/// Trait which defines a method to check if an entity is disabled.
pub trait IsDisabled {
    /// Returns true if the given entity is disabled.
    fn is_disabled(&self, entity: Entity) -> bool;
}

impl<'p, 'w> IsDisabled for Cx<'p, 'w> {
    fn is_disabled(&self, entity: Entity) -> bool {
        self.world().get::<Disabled>(entity).is_some()
    }
}

impl IsDisabled for World {
    fn is_disabled(&self, entity: Entity) -> bool {
        self.get::<Disabled>(entity).is_some()
    }
}

use bevy::{ecs::system::SystemParam, prelude::*};

/// An injectable helper for computing the relative position of an entity with respect to some
/// ancestor.
#[derive(SystemParam)]
pub struct RelativeWorldPositions<'w, 's> {
    query: Query<'w, 's, (&'static Node, &'static GlobalTransform, &'static Parent)>,
}

impl<'w, 's> RelativeWorldPositions<'w, 's> {
    pub fn transform_relative(&self, id: Entity, pos: Vec2, levels: usize) -> Vec2 {
        let mut current = id;
        for _ in 0..levels {
            if let Ok((_, _, parent)) = self.query.get(current) {
                current = parent.get();
            } else {
                return pos;
            }
        }

        let Ok((node, transform, _)) = self.query.get(current) else {
            return pos;
        };

        let rect = node.logical_rect(transform);
        pos - rect.min
    }
}

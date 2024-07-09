use bevy::{hierarchy::Parent, prelude::*};
use bevy_mod_picking::{focus::HoverMap, pointer::PointerId};
use bevy_quill_core::Cx;

/// Component which tracks whether the pointer is hovering over an entity.
#[derive(Default, Component)]
pub(crate) struct Hovering(pub bool);

// Note: previously this was implemented as a Reaction, however it was reacting every frame
// because HoverMap is mutated every frame regardless of whether or not it changed.
pub(crate) fn update_hover_states(
    hover_map: Option<Res<HoverMap>>,
    mut hovers: Query<(Entity, &mut Hovering)>,
    parent_query: Query<&Parent>,
) {
    let Some(hover_map) = hover_map else { return };
    let hover_set = hover_map.get(&PointerId::Mouse);
    for (entity, mut hoverable) in hovers.iter_mut() {
        let is_hovering = match hover_set {
            Some(map) => map.iter().any(|(ha, _)| {
                *ha == entity || parent_query.iter_ancestors(*ha).any(|e| e == entity)
            }),
            None => false,
        };
        if hoverable.0 != is_hovering {
            hoverable.0 = is_hovering;
        }
    }
}

/// Method which tracks whether the mouse is hovering over the given entity.
pub trait UseIsHover {
    /// Hook that returns true when the mouse is hovering over the given entity or a descendant.
    fn is_hovered(&mut self, target: Entity) -> bool;
}

impl<'p, 'w> UseIsHover for Cx<'p, 'w> {
    fn is_hovered(&mut self, target: Entity) -> bool {
        let mut entt = self.world_mut().entity_mut(target);
        if !entt.contains::<Hovering>() {
            entt.insert(Hovering(false));
        }
        self.use_component::<Hovering>(target)
            .map(|h| h.0)
            .unwrap_or(false)
    }
}

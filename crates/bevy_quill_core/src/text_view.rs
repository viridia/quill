use bevy::{
    ecs::world::DeferredWorld,
    hierarchy::BuildChildren,
    prelude::{Entity, Text, World},
};

#[cfg(feature = "verbose")]
use bevy::log::info;
use bevy_mod_stylebuilder::UseInheritedTextStyles;

use crate::{cx::Cx, View};

impl View for String {
    type State = Entity;

    fn nodes(&self, _world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        out.push(*state)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        build_text_view(cx.world_mut(), self)
    }

    fn rebuild(&self, cx: &mut crate::cx::Cx, state: &mut Self::State) -> bool {
        rebuild_text_view(cx.world_mut(), self, state)
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("Razing String View: {}", *state);

        // Delete the text node.
        world.commands().entity(*state).remove_parent().despawn();
    }
}

impl<'a: 'static> View for &'a str {
    type State = Entity;

    fn nodes(&self, _world: &World, state: &Self::State, out: &mut Vec<Entity>) {
        out.push(*state);
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        build_text_view(cx.world_mut(), self)
    }

    fn rebuild(&self, cx: &mut crate::cx::Cx, state: &mut Self::State) -> bool {
        rebuild_text_view(cx.world_mut(), self, state)
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("Razing &str View: {}", *state);

        // Delete the text node.
        world.commands().entity(*state).remove_parent().despawn();
    }
}

fn build_text_view(world: &mut World, text: &str) -> Entity {
    world
        .spawn((Text(text.to_string()), UseInheritedTextStyles))
        .id()
}

fn rebuild_text_view(world: &mut World, text: &str, state: &mut Entity) -> bool {
    // If it's a single node and has a text component
    let mut entt = world.entity_mut(*state);
    if let Some(mut old_text) = entt.get_mut::<Text>() {
        // If the text didn't change, do nothing.
        if old_text.0 == text {
            return false;
        }
        // Replace the text sections in the `Text` component.
        old_text.0 = text.to_string();
        false
    } else {
        entt.despawn();
        *state = build_text_view(world, text);
        true
    }
}

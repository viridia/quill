use bevy::{
    hierarchy::BuildWorldChildren,
    prelude::{default, Entity, World},
    text::{Text, TextSection, TextStyle},
    ui::node_bundles::TextBundle,
};

#[cfg(feature = "verbose")]
use bevy::log::info;
use bevy_mod_stylebuilder::UseInheritedTextStyles;

use crate::{cx::Cx, NodeSpan, View};

impl View for String {
    type State = Entity;

    fn nodes(&self, _world: &World, state: &Self::State) -> NodeSpan {
        NodeSpan::Node(*state)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        build_text_view(cx.world_mut(), self)
    }

    fn rebuild(&self, cx: &mut crate::cx::Cx, state: &mut Self::State) -> bool {
        rebuild_text_view(cx.world_mut(), self, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("Razing String View: {}", *state);

        // Delete the text node.
        world.entity_mut(*state).remove_parent();
        world.entity_mut(*state).despawn();
    }
}

impl<'a: 'static> View for &'a str {
    type State = Entity;

    fn nodes(&self, _world: &World, state: &Self::State) -> NodeSpan {
        NodeSpan::Node(*state)
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        build_text_view(cx.world_mut(), self)
    }

    fn rebuild(&self, cx: &mut crate::cx::Cx, state: &mut Self::State) -> bool {
        rebuild_text_view(cx.world_mut(), self, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        #[cfg(feature = "verbose")]
        info!("Razing &str View: {}", *state);

        // Delete the text node.
        world.entity_mut(*state).remove_parent();
        world.entity_mut(*state).despawn();
    }
}

fn build_text_view(world: &mut World, text: &str) -> Entity {
    world
        .spawn((
            TextBundle {
                text: Text::from_section(text, TextStyle { ..default() }),
                ..default()
            },
            UseInheritedTextStyles,
        ))
        .id()
}

fn rebuild_text_view(world: &mut World, text: &str, state: &mut Entity) -> bool {
    // If it's a single node and has a text component
    let mut entt = world.entity_mut(*state);
    if let Some(mut old_text) = entt.get_mut::<Text>() {
        // If the text didn't change, do nothing.
        if old_text.sections.len() == 1 && old_text.sections[0].value == text {
            return false;
        }
        // Replace the text sections in the `Text` component.
        old_text.sections.clear();
        old_text.sections.push(TextSection {
            value: text.to_string(),
            style: TextStyle { ..default() },
        });
        false
    } else {
        entt.despawn();
        *state = build_text_view(world, text);
        true
    }
}

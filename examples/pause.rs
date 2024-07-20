//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::prelude::*;
use bevy_mod_picking::{debug::DebugPickingMode, DefaultPickingPlugins};
use bevy_quill::{Cx, Element, QuillPlugin, View, ViewTemplate, ViewThunk};
use bevy_quill_obsidian::ObsidianUiPlugin;
use common::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Paused(pub bool);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
        ))
        .insert_state(Paused(false))
        .insert_resource(DebugPickingMode::Disabled)
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, toggle_pause, rotate))
        .add_systems(OnEnter(Paused(true)), enter_paused_state)
        .add_systems(OnExit(Paused(true)), exit_paused_state)
        .run();
}

#[derive(Component, Clone)]
struct PauseUiMarker;

fn enter_paused_state(mut commands: Commands) {
    commands.spawn((
        Element::<NodeBundle>::new()
            .children(GamePausedView)
            .to_root(),
        PauseUiMarker,
    ));
}

fn exit_paused_state(world: &mut World) {
    let mut q_ui_root = world.query_filtered::<Entity, With<PauseUiMarker>>();
    let ui_root = q_ui_root.single(world);
    let thunk = world.entity_mut(ui_root).take::<ViewThunk>().unwrap();
    thunk.raze(world, ui_root);
    world.entity_mut(ui_root).despawn();
}

#[derive(Clone, PartialEq, Default)]
struct GamePausedView;

impl ViewTemplate for GamePausedView {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        "Game Paused"
    }
}

pub fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    paused: Res<State<Paused>>,
    mut paused_next: ResMut<NextState<Paused>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if paused.0 {
            paused_next.set(Paused(false));
        } else {
            paused_next.set(Paused(true));
        }
    }
}

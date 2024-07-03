#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

mod edge_display;
mod events;
mod graph_display;
mod materials;
mod node_display;
mod terminal_display;

use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
    ui::UiMaterialPlugin,
};

use bevy_mod_picking::prelude::EventListenerPlugin;
pub use edge_display::EdgeDisplay;
pub use events::*;
pub use graph_display::GraphDisplay;
use materials::{DotGridMaterial, DrawPathMaterial};
pub use node_display::NodeDisplay;
pub use terminal_display::{InputTerminalDisplay, OutputTerminalDisplay};

/// Plugin for the Obsidian UI library.
pub struct ObsidianGraphPlugin;

impl Plugin for ObsidianGraphPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/dot_grid.wgsl");
        embedded_asset!(app, "assets/draw_path.wgsl");
        app.init_resource::<GestureState>()
            .add_plugins((
                UiMaterialPlugin::<DotGridMaterial>::default(),
                UiMaterialPlugin::<DrawPathMaterial>::default(),
                EventListenerPlugin::<GraphEvent>::default(),
            ))
            .add_event::<GraphEvent>();
    }
}

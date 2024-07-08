#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

mod edge_display;
mod edge_display_ls;
mod events;
mod graph_display;
mod materials;
mod node_display;
mod relative_pos;
mod terminal_display;

use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
    sprite::Material2dPlugin,
    ui::UiMaterialPlugin,
};

use bevy_mod_picking::prelude::EventListenerPlugin;
pub use edge_display::EdgeDisplay;
pub use events::*;
pub use graph_display::GraphDisplay;
use materials::{DotGridMaterial, DrawPathMaterial, LineMaterial};
pub use node_display::NodeDisplay;
pub use terminal_display::{InputTerminalDisplay, NoTerminalDisplay, OutputTerminalDisplay};

/// Plugin for the Obsidian UI library.
pub struct ObsidianGraphPlugin;

impl Plugin for ObsidianGraphPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/dot_grid.wgsl");
        embedded_asset!(app, "assets/draw_path.wgsl");
        embedded_asset!(app, "assets/line_material.wgsl");
        app.init_resource::<GestureState>()
            .add_plugins((
                UiMaterialPlugin::<DotGridMaterial>::default(),
                UiMaterialPlugin::<DrawPathMaterial>::default(),
                Material2dPlugin::<LineMaterial>::default(),
                EventListenerPlugin::<GraphEvent>::default(),
            ))
            .add_event::<GraphEvent>();
    }
}

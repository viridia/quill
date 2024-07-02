#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

mod graph;
mod materials;
mod node;

use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
    ui::UiMaterialPlugin,
};

pub use graph::GraphDisplay;
use materials::{DotGridMaterial, DrawPathMaterial};
pub use node::{InputTerminalDisplay, NodeDisplay, OutputTerminalDisplay};

/// Plugin for the Obsidian UI library.
pub struct ObsidianGraphPlugin;

impl Plugin for ObsidianGraphPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/dot_grid.wgsl");
        embedded_asset!(app, "assets/draw_path.wgsl");
        app.add_plugins((
            UiMaterialPlugin::<DotGridMaterial>::default(),
            UiMaterialPlugin::<DrawPathMaterial>::default(),
        ));
    }
}

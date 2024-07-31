mod mesh_builder;
mod overlay;
mod overlay_material;
mod shape_builder;

use bevy::{app::Plugin, asset::embedded_asset, pbr::MaterialPlugin};
pub use overlay::Overlay;
pub use shape_builder::{PolygonOptions, ShapeBuilder, StrokeMarker};

use crate::overlay_material::OverlayMaterial;

use self::overlay_material::UnderlayMaterial;

/// Plugin for the overlays module.
pub struct QuillOverlaysPlugin;

impl Plugin for QuillOverlaysPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        embedded_asset!(app, "overlay.wgsl");
        app.add_plugins((
            MaterialPlugin::<OverlayMaterial>::default(),
            MaterialPlugin::<UnderlayMaterial>::default(),
        ));
    }
}

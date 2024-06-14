mod cx;
mod effects;
mod element;
mod node_span;
// mod presenter;
mod style;
mod text_view;
mod tracking_scope;
mod view;
mod view_tuple;

use bevy::app::{App, Plugin, Update};
pub use cx::Cx;
pub use element::*;
pub use node_span::*;
// pub use text_view::*;
pub use view::*;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (build_views, rebuild_views));
        // Add your plugin logic here
        // For example, you can add systems, resources, or other plugins to the app

        // app.add_startup_system(setup.system());
    }
}

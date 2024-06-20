#![feature(impl_trait_in_assoc_type, associated_type_defaults)]
mod callback;
mod cond;
mod cx;
mod effects;
mod element;
mod insert;
mod mutable;
mod node_span;
mod style;
mod text_view;
mod tracking_scope;
mod view;
mod view_template;
mod view_tuple;

use bevy::{
    app::{App, Plugin, Update},
    prelude::IntoSystemConfigs,
};
pub use callback::*;
pub use cond::Cond;
pub use cx::Cx;
pub use element::*;
pub use mutable::*;
pub use node_span::*;
pub use tracking_scope::TrackingScope;
pub use view::*;
pub use view_template::ViewTemplate;
pub use view_tuple::ViewTuple;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (build_views, rebuild_views, reattach_children).chain(),
        );
    }
}

#![feature(impl_trait_in_assoc_type, associated_type_defaults)]
mod callback;
mod cond;
mod cx;
mod effects;
mod element;
mod r#for;
mod for_each;
mod for_index;
mod fragment;
mod insert;
mod lcs;
mod mutable;
mod node_span;
mod portal;
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
use bevy_mod_stylebuilder::{StyleBuilderPlugin, StyleBuilderSystemSet};

pub use callback::*;
pub use cond::Cond;
pub use cx::Cx;
pub use element::*;
pub use for_each::ForEach;
pub use for_index::ForIndex;
pub use fragment::Fragment;
pub use mutable::*;
pub use node_span::*;
pub use portal::Portal;
pub use r#for::For;
pub use tracking_scope::TrackingScope;
pub use tracking_scope::TrackingScopeTracing;
pub use view::*;
pub use view_template::ViewTemplate;
pub use view_tuple::AnyViewTuple;
pub use view_tuple::ChildViews;
pub use view_tuple::IntoChildViews;
pub use view_tuple::ViewTuple;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StyleBuilderPlugin).add_systems(
            Update,
            (build_views, rebuild_views, reattach_children)
                .chain()
                .before(StyleBuilderSystemSet),
        );
    }
}

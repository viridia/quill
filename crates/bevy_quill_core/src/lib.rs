#![feature(impl_trait_in_assoc_type, associated_type_defaults)]
mod callback;
mod cond;
mod cx;
mod dynamic;
mod effects;
mod element;
mod r#for;
mod for_each;
mod for_index;
mod insert;
mod lcs;
mod mutable;
mod portal;
mod style;
mod switch;
mod text_view;
mod tracking_scope;
mod view;
mod view_child;
mod view_template;

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::IntoSystemConfigs,
};
use bevy_mod_stylebuilder::{StyleBuilderPlugin, StyleBuilderSystemSet};

pub mod prelude {
    pub use crate::callback::*;
    pub use crate::cond::Cond;
    pub use crate::cx::Cx;
    pub use crate::cx::EffectOptions;
    pub use crate::element::*;
    pub use crate::for_each::ForEach;
    pub use crate::for_index::ForIndex;
    pub use crate::mutable::*;
    pub use crate::r#for::For;
    pub use crate::switch::Switch;
    pub use crate::view::*;
    pub use crate::view_template::ViewTemplate;
}

pub use callback::*;
pub use cond::Cond;
pub use cx::Cx;
pub use cx::EffectOptions;
pub use dynamic::Dynamic;
pub use element::*;
pub use for_each::ForEach;
pub use for_index::ForIndex;
pub use mutable::*;
pub use portal::Portal;
pub use r#for::For;
pub use switch::Switch;
use tracking_scope::cleanup_tracking_scopes;
pub use tracking_scope::TrackingScope;
pub use tracking_scope::TrackingScopeTracing;
pub use view::*;
pub use view_child::IntoViewChild;
pub use view_child::ViewChild;
pub use view_template::ViewTemplate;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StyleBuilderPlugin)
            .add_systems(Startup, (cleanup_tracking_scopes, cleanup_view_roots))
            .add_systems(
                Update,
                (build_views, reaction_control_system, reattach_children)
                    .chain()
                    .before(StyleBuilderSystemSet),
            );
    }
}

// mod atlas_loader;
mod builder;
mod builder_background;
mod builder_border_color;
mod builder_border_radius;
mod builder_font;
mod builder_layout;
mod builder_outline;
mod builder_visibility;
mod builder_z_index;
mod text_styles;
// mod builder_texture_atlas;

#[cfg(feature = "mod_picking")]
mod builder_pointer_events;

use std::sync::Arc;

use bevy::{
    app::{Plugin, Update},
    prelude::{IntoSystemConfigs, SystemSet},
};
// pub use atlas_loader::TextureAtlasLoader;
pub use builder::*;
pub use builder_background::StyleBuilderBackground;
pub use builder_border_color::StyleBuilderBorderColor;
pub use builder_border_radius::StyleBuilderBorderRadius;
pub use builder_font::StyleBuilderFont;
pub use builder_layout::StyleBuilderLayout;
pub use builder_outline::StyleBuilderOutline;
pub use builder_visibility::StyleBuilderVisibility;
pub use builder_z_index::StyleBuilderZIndex;
use text_styles::update_text_styles;
pub use text_styles::{InheritableFontStyles, UseInheritedTextStyles};
// pub use builder_texture_atlas::StyleBuilderTextureAtlas;

#[cfg(feature = "mod_picking")]
pub use builder_pointer_events::StyleBuilderPointerEvents;

/// `StyleTuple` - a variable-length tuple of [`StyleHandle`]s.
pub trait StyleTuple: Sync + Send {
    /// Method to apply the style to a target entity.
    fn apply(&self, ctx: &mut StyleBuilder);

    /// Wrap the tuple in a [`StyleHandle`].
    fn into_handle(self) -> StyleHandle;
}

/// Empty tuple.
impl StyleTuple for () {
    fn apply(&self, _ctx: &mut StyleBuilder) {}

    fn into_handle(self) -> StyleHandle {
        StyleHandle::none()
    }
}

impl<F: Fn(&mut StyleBuilder) + Send + Sync + 'static> StyleTuple for F {
    fn apply(&self, ctx: &mut StyleBuilder) {
        (self)(ctx);
    }

    fn into_handle(self) -> StyleHandle {
        StyleHandle::new(self)
    }
}

impl StyleTuple for StyleHandle {
    fn apply(&self, ctx: &mut StyleBuilder) {
        if let Some(s) = self.style.as_ref() {
            s.apply(ctx);
        }
    }

    fn into_handle(self) -> StyleHandle {
        StyleHandle::new(self)
    }
}

macro_rules! impl_style_tuple {
    ( $($style: ident, $idx: tt);+ ) => {
        impl<$(
            $style: StyleTuple + 'static,
        )+> StyleTuple for ( $( $style, )* ) {
            fn apply(&self, builder: &mut StyleBuilder) {
                $( self.$idx.apply(builder); )*
            }

            fn into_handle(self) -> StyleHandle {
                StyleHandle::new(self)
            }
        }
    };
}

impl_style_tuple!(E0, 0);
impl_style_tuple!(E0, 0; E1, 1);
impl_style_tuple!(E0, 0; E1, 1; E2, 2);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15);

/// Wrapper type that allows [`StyleTuple`]s to be passed from parent to child views.
#[derive(Default, Clone)]
pub struct StyleHandle {
    /// Reference to the collection of styles.
    pub style: Option<Arc<dyn StyleTuple>>,
}

impl PartialEq for StyleHandle {
    fn eq(&self, other: &Self) -> bool {
        match (&self.style, &other.style) {
            (Some(s1), Some(s2)) => Arc::ptr_eq(s1, s2),
            (None, None) => true,
            _ => false,
        }
    }
}

impl StyleHandle {
    /// Construct a new style handle.
    pub fn new<S: StyleTuple + 'static>(style: S) -> Self {
        Self {
            style: Some(Arc::new(style)),
        }
    }

    /// Construct a placeholder style handle.
    pub fn none() -> Self {
        Self { style: None }
    }
}

/// A system set that includes any systems that run dynamic style computations. These will
/// generally run after the UI nodes have been updated.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleBuilderSystemSet;

pub struct StyleBuilderPlugin;

impl Plugin for StyleBuilderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, update_text_styles.in_set(StyleBuilderSystemSet));
    }
}

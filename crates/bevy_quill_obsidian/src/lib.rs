#![feature(impl_trait_in_assoc_type, associated_type_defaults)]
use bevy::{asset::embedded_asset, prelude::*};

mod rounded_corners;

/// Utilities for animating component properties.
pub mod animation;

/// Module containing standard color definitions.
#[allow(missing_docs)]
pub mod colors;

/// Module containing interactive and layout control widgets.
pub mod controls;

/// Module containing utilities for creating custom window cursors.
pub mod cursor;

/// Utilities for tabbing between widgets.
pub mod focus;

/// Utilities for floating popups.
pub mod floating;

/// Module containing extensions to `Cx`.
pub mod hooks;

/// Module containing custom materials.
mod materials;

/// Utilities for managing scrolling views.
pub mod scrolling;

/// Module containing standard sizes.
pub mod size;

/// Module of utilities for embedding a 3D viewport in the 2D UI.
pub mod viewport;

/// Standard styles for fonts.
pub mod typography;

pub mod prelude {
    pub use crate::controls::*;
    pub use crate::hooks::*;
    pub use crate::size::*;
}

use bevy_mod_picking::prelude::EventListenerPlugin;
use controls::{MenuCloseEvent, RecentColors};
use materials::{GradientRectMaterial, SliderRectMaterial, SwatchRectMaterial};
pub use rounded_corners::RoundedCorners;

pub use hooks::is_hover::UseIsHover as _;

/// Plugin for the Obsidian UI library.
pub struct ObsidianUiPlugin;

impl Plugin for ObsidianUiPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/fonts/Open_Sans/static/OpenSans-Bold.ttf");
        embedded_asset!(app, "assets/fonts/Open_Sans/static/OpenSans-BoldItalic.ttf");
        embedded_asset!(app, "assets/fonts/Open_Sans/static/OpenSans-Medium.ttf");
        embedded_asset!(
            app,
            "assets/fonts/Open_Sans/static/OpenSans-MediumItalic.ttf"
        );
        embedded_asset!(app, "assets/fonts/Open_Sans/static/OpenSans-Regular.ttf");
        embedded_asset!(app, "assets/fonts/Open_Sans/static/OpenSans-Italic.ttf");
        embedded_asset!(app, "assets/icons/add_box.png");
        embedded_asset!(app, "assets/icons/add.png");
        embedded_asset!(app, "assets/icons/checkmark.png");
        embedded_asset!(app, "assets/icons/chevron_down.png");
        embedded_asset!(app, "assets/icons/chevron_up.png");
        embedded_asset!(app, "assets/icons/chevron_left.png");
        embedded_asset!(app, "assets/icons/chevron_right.png");
        embedded_asset!(app, "assets/icons/close.png");
        embedded_asset!(app, "assets/icons/disc.png");
        embedded_asset!(app, "assets/icons/gradient_thumb.png");
        embedded_asset!(app, "assets/icons/lock.png");
        embedded_asset!(app, "assets/icons/redo.png");
        embedded_asset!(app, "assets/icons/remove.png");
        embedded_asset!(app, "assets/icons/tune.png");
        embedded_asset!(app, "assets/icons/undo.png");
        embedded_asset!(app, "assets/shaders/gradient_rect.wgsl");
        embedded_asset!(app, "assets/shaders/swatch_rect.wgsl");
        embedded_asset!(app, "assets/shaders/slider_rect.wgsl");
        app.add_plugins((
            UiMaterialPlugin::<GradientRectMaterial>::default(),
            UiMaterialPlugin::<SliderRectMaterial>::default(),
            UiMaterialPlugin::<SwatchRectMaterial>::default(),
            hooks::BistableTransitionPlugin,
            animation::AnimatedTransitionPlugin,
            focus::KeyboardInputPlugin,
        ))
        .add_plugins((
            EventListenerPlugin::<scrolling::ScrollWheel>::default(),
            EventListenerPlugin::<MenuCloseEvent>::default(),
        ))
        .add_event::<scrolling::ScrollWheel>()
        .add_systems(
            Update,
            (
                scrolling::handle_scroll_events,
                scrolling::update_scroll_positions,
                hooks::is_hover::update_hover_states,
                cursor::update_cursor,
            ),
        )
        .init_resource::<RecentColors>()
        .add_systems(PostUpdate, floating::position_floating);
    }
}

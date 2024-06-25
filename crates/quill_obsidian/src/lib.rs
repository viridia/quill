#![feature(impl_trait_in_assoc_type, associated_type_defaults)]
use bevy::prelude::*;

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
// pub mod floating;

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

use bevy_mod_picking::prelude::EventListenerPlugin;
use materials::{GradientRectMaterial, SliderRectMaterial, SwatchRectMaterial};
pub use rounded_corners::RoundedCorners;

/// Plugin for the Obsidian UI library.
pub struct ObsidianUiPlugin;

pub use hooks::is_hover::UseIsHover as _;

impl Plugin for ObsidianUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiMaterialPlugin::<GradientRectMaterial>::default(),
            UiMaterialPlugin::<SliderRectMaterial>::default(),
            UiMaterialPlugin::<SwatchRectMaterial>::default(),
            // UiMaterialPlugin::<DotGridMaterial>::default(),
            // UiMaterialPlugin::<DrawPathMaterial>::default(),
            hooks::BistableTransitionPlugin,
            animation::AnimatedTransitionPlugin,
            focus::KeyboardInputPlugin,
        ))
        .add_plugins((
            EventListenerPlugin::<scrolling::ScrollWheel>::default(),
            // EventListenerPlugin::<MenuCloseEvent>::default(),
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
        );
        // .add_systems(PostUpdate, floating::position_floating);
    }
}

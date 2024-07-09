use bevy::{asset::AssetPath, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill_core::*;

use crate::colors;

/// Control that displays an icon.
#[derive(Clone, PartialEq)]
pub struct Icon {
    /// Asset path for the icon
    pub icon: String,

    /// Size of the icon in pixels.
    pub size: Vec2,

    /// Color of the icon.
    pub color: Srgba,

    /// Additional styles to apply to the icon
    pub style: StyleHandle,
}

impl Icon {
    /// Create a new icon.
    pub fn new(icon: &str) -> Self {
        Self {
            icon: icon.to_string(),
            ..default()
        }
    }

    /// Set the size of the icon.
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the icon.
    pub fn color(mut self, color: impl Into<Srgba>) -> Self {
        self.color = color.into();
        self
    }

    /// Set the style of the icon.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            icon: "".to_string(),
            size: Vec2::splat(12.0),
            color: colors::FOREGROUND,
            style: StyleHandle::default(),
        }
    }
}

impl ViewTemplate for Icon {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        let icon = self.icon.clone();
        let size = self.size;

        Element::<NodeBundle>::new()
            .style((
                move |sb: &mut StyleBuilder| {
                    sb.width(size.x)
                        .height(size.y)
                        .background_image(AssetPath::parse(&icon));
                },
                self.style.clone(),
            ))
            .style_dyn(
                |color, sb| {
                    sb.background_image_color(color);
                },
                self.color,
            )
    }
}

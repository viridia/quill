#![allow(missing_docs)]

use bevy::prelude::*;

/// Struct that holds the properties for text rendering, which can be inherited. This allows
/// setting for font face, size and color to be established at a parent level and inherited by
/// child text elements.
///
/// This will be applied to any text nodes that are children of the target entity, unless
/// those nodes explicitly override the properties.
#[derive(Component, Default, Clone, Debug)]
pub struct InheritableFontStyles {
    /// Path to the font asset.
    pub font: Option<Handle<Font>>,

    /// Inherited size of the font.
    pub font_size: Option<f32>,

    /// Inherited text color.
    pub color: Option<Color>,
}

impl InheritableFontStyles {
    /// True if all text style properties are set.
    pub fn is_final(&self) -> bool {
        self.font.is_some() && self.font_size.is_some() && self.color.is_some()
    }

    /// Merge the properties from another `InheritableTextStyles` into this one.
    pub fn merge(&mut self, other: &InheritableFontStyles) {
        if other.font.is_some() && self.font.is_none() {
            self.font.clone_from(&other.font);
        }
        if other.font_size.is_some() && self.font_size.is_none() {
            self.font_size = other.font_size;
        }
        if other.color.is_some() && self.color.is_none() {
            self.color = other.color;
        }
    }
}

/// A marker component that is used to indicate that the text element needs to recompute the
/// inherited text styles.
#[derive(Component)]
pub struct TextStyleChanged;

pub(crate) fn update_text_styles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Text), With<TextStyleChanged>>,
    inherited: Query<&InheritableFontStyles>,
    parents: Query<&Parent>,
    server: Res<AssetServer>,
) {
    for (entity, mut text) in query.iter_mut() {
        let mut styles = InheritableFontStyles::default();

        // Search parents for inherited styles.
        let mut ancestor = entity;
        loop {
            if styles.is_final() {
                break;
            }
            if let Ok(inherited_styles) = inherited.get(ancestor) {
                styles.merge(inherited_styles);
                if styles.is_final() {
                    break;
                }
            }
            if let Ok(parent) = parents.get(ancestor) {
                ancestor = parent.get();
            } else {
                break;
            }
        }

        // If we have a font handle, but it's not ready, then skip this update.
        if let Some(ref handle) = styles.font {
            match server.load_state(handle) {
                bevy::asset::LoadState::Loaded => {}
                _ => {
                    continue;
                }
            }
        }

        let style = TextStyle {
            font: styles.font.unwrap_or_default(),
            font_size: styles.font_size.unwrap_or(12.),
            color: styles.color.unwrap_or(Color::WHITE),
        };

        for section in text.sections.iter_mut() {
            section.style = style.clone();
        }
        commands.entity(entity).remove::<TextStyleChanged>();
    }
}

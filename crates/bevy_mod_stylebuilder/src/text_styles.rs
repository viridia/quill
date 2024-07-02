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

/// A marker component that is used to indicate that the text entity wants to opt-in to using
/// inherited text styles.
#[derive(Component)]
pub struct UseInheritedTextStyles;

pub(crate) fn update_text_styles(
    mut query: Query<(Entity, &mut Text), With<UseInheritedTextStyles>>,
    inherited: Query<Ref<InheritableFontStyles>>,
    parents: Query<&Parent>,
) {
    let inherited_changed = inherited.iter().any(|cmp| cmp.is_changed());
    for (entity, mut text) in query.iter_mut() {
        if text.is_changed() || inherited_changed {
            let style = match compute_inherited_style(entity, &inherited, &parents) {
                Some(value) => value,
                None => continue,
            };

            let styles_changed = text.sections.iter().any(|section| {
                section.style.font != style.font
                    || section.style.font_size != style.font_size
                    || section.style.color != style.color
            });
            if styles_changed {
                for section in text.sections.iter_mut() {
                    section.style = style.clone();
                }
            }
        }
    }
}

fn compute_inherited_style(
    entity: Entity,
    inherited: &Query<Ref<InheritableFontStyles>, ()>,
    parents: &Query<&Parent, ()>,
) -> Option<TextStyle> {
    let mut styles = InheritableFontStyles::default();
    let mut ancestor = entity;
    loop {
        if styles.is_final() {
            break;
        }
        if let Ok(inherited_styles) = inherited.get(ancestor) {
            styles.merge(inherited_styles.as_ref());
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
    let style = TextStyle {
        font: styles.font.unwrap_or_default(),
        font_size: styles.font_size.unwrap_or(12.),
        color: styles.color.unwrap_or(Color::WHITE),
    };
    Some(style)
}

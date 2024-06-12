#![allow(missing_docs)]

use super::builder::{AssetPathParam, ColorParam, OptFloatParam, StyleBuilder};
use bevy::prelude::*;

pub trait StyleBuilderFont {
    fn color(&mut self, color: impl ColorParam) -> &mut Self;
    fn font<'p>(&mut self, path: impl AssetPathParam<'p>) -> &mut Self;
    fn font_size(&mut self, val: impl OptFloatParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderFont for StyleBuilder<'a, 'w> {
    fn color(&mut self, color: impl ColorParam) -> &mut Self {
        match self.target.get_mut::<InheritableFontStyles>() {
            Some(mut text_style) => text_style.color = color.to_val(),
            None => {
                self.target.insert(InheritableFontStyles {
                    color: color.to_val(),
                    ..Default::default()
                });
            }
        };
        self
    }

    fn font<'p>(&mut self, path: impl AssetPathParam<'p>) -> &mut Self {
        let font = path.to_path().map(|p| self.load_asset::<Font>(p));
        match self.target.get_mut::<InheritableFontStyles>() {
            Some(mut text_style) => {
                text_style.font = font;
            }
            None => {
                self.target.insert(InheritableFontStyles {
                    font,
                    ..Default::default()
                });
            }
        };
        self
    }

    fn font_size(&mut self, val: impl OptFloatParam) -> &mut Self {
        match self.target.get_mut::<InheritableFontStyles>() {
            Some(mut text_style) => {
                text_style.font_size = val.to_val();
            }
            None => {
                self.target.insert(InheritableFontStyles {
                    font_size: val.to_val(),
                    ..Default::default()
                });
            }
        };
        self
    }
}

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

#![allow(missing_docs)]

use crate::{text_styles::InheritableFontStyles, MaybeHandleOrPath};

use super::builder::{ColorParam, OptFloatParam, StyleBuilder};
use bevy::prelude::*;

pub trait StyleBuilderFont {
    fn color(&mut self, color: impl ColorParam) -> &mut Self;
    fn font<'p>(&mut self, path: impl Into<MaybeHandleOrPath<'p, Font>>) -> &mut Self;
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

    fn font<'p>(&mut self, path: impl Into<MaybeHandleOrPath<'p, Font>>) -> &mut Self {
        let font = match path.into() {
            MaybeHandleOrPath::Handle(h) => Some(h),
            MaybeHandleOrPath::Path(p) => Some(self.load_asset::<Font>(p)),
            MaybeHandleOrPath::None => None,
        };
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

use bevy::{
    prelude::*,
    render::texture::Image,
    ui::{self, UiImage},
};

use super::builder::{ColorParam, MaybeHandleOrPath, StyleBuilder};

#[allow(missing_docs)]
pub trait StyleBuilderBackground {
    /// Set the background image of the target entity.
    fn background_image<'p>(&mut self, path: impl Into<MaybeHandleOrPath<'p, Image>>) -> &mut Self;

    /// Set the background image of the target entity, and also explicitly configure the
    /// horizontal and vertical flip.
    fn background_image_flipped<'p>(
        &mut self,
        path: impl Into<MaybeHandleOrPath<'p, Image>>,
        flip_x: bool,
        flip_y: bool,
    ) -> &mut Self;

    /// Set the background color, or `None` for transparent.
    fn background_color(&mut self, color: impl ColorParam) -> &mut Self;

    /// Set the background color, or `None` for transparent.
    fn background_image_color(&mut self, color: impl ColorParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderBackground for StyleBuilder<'a, 'w> {
    fn background_image<'p>(&mut self, path: impl Into<MaybeHandleOrPath<'p, Image>>) -> &mut Self {
        self.background_image_flipped(path, false, false)
    }

    fn background_image_flipped<'p>(
        &mut self,
        path: impl Into<MaybeHandleOrPath<'p, Image>>,
        flip_x: bool,
        flip_y: bool,
    ) -> &mut Self {
        let texture = match path.into() {
            MaybeHandleOrPath::Handle(h) => Some(h),
            MaybeHandleOrPath::Path(p) => Some(self.load_asset::<Image>(p)),
            MaybeHandleOrPath::None => None,
        };
        match (texture, self.target.get_mut::<UiImage>()) {
            (Some(texture), Some(mut uii)) => {
                uii.texture = texture;
                uii.flip_x = flip_x;
                uii.flip_y = flip_y;
            }
            (Some(texture), None) => {
                self.target.insert(UiImage {
                    texture,
                    flip_x,
                    flip_y,
                    ..default()
                });
            }
            (None, Some(_)) => {
                self.target.remove::<UiImage>();
            }
            _ => (),
        };
        self
    }

    fn background_color(&mut self, color: impl ColorParam) -> &mut Self {
        if let Some(color) = color.to_val() {
            self.target.insert(ui::BackgroundColor(color));
        } else {
            self.target.remove::<ui::BackgroundColor>();
        }
        self
    }

    fn background_image_color(&mut self, color: impl ColorParam) -> &mut Self {
        match (color.to_val(), self.target.get_mut::<UiImage>()) {
            (Some(color), Some(mut uii)) => {
                uii.color = color;
            }
            (Some(color), None) => {
                self.target.insert(UiImage { color, ..default() });
            }
            (None, Some(_)) => {
                self.target.remove::<UiImage>();
            }
            _ => (),
        };
        self
    }
}

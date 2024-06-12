use super::builder::{ColorParam, LengthParam, StyleBuilder};
use bevy::ui;

#[allow(missing_docs)]
pub trait StyleBuilderOutline {
    fn outline_color(&mut self, color: impl ColorParam) -> &mut Self;
    fn outline_width(&mut self, length: impl LengthParam) -> &mut Self;
    fn outline_offset(&mut self, length: impl LengthParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderOutline for StyleBuilder<'a, 'w> {
    fn outline_color(&mut self, color: impl ColorParam) -> &mut Self {
        match (color.to_val(), self.target.get_mut::<ui::Outline>()) {
            (Some(color), Some(mut outline)) => {
                outline.color = color;
            }
            (None, Some(_)) => {
                self.target.remove::<ui::Outline>();
            }
            (Some(color), None) => {
                self.target.insert(ui::Outline {
                    color,
                    ..Default::default()
                });
            }
            (None, None) => (),
        };
        self
    }

    fn outline_width(&mut self, length: impl LengthParam) -> &mut Self {
        match self.target.get_mut::<ui::Outline>() {
            Some(mut outline) => {
                outline.width = length.to_val();
            }
            None => {
                self.target.insert(ui::Outline {
                    width: length.to_val(),
                    ..Default::default()
                });
            }
        }
        self
    }

    fn outline_offset(&mut self, length: impl LengthParam) -> &mut Self {
        match self.target.get_mut::<ui::Outline>() {
            Some(mut outline) => {
                outline.offset = length.to_val();
            }
            None => {
                self.target.insert(ui::Outline {
                    offset: length.to_val(),
                    ..Default::default()
                });
            }
        }
        self
    }
}

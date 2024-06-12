use bevy::ui;

use super::builder::{ColorParam, StyleBuilder};

#[allow(missing_docs)]
pub trait StyleBuilderBorderColor {
    fn border_color(&mut self, color: impl ColorParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderBorderColor for StyleBuilder<'a, 'w> {
    fn border_color(&mut self, color: impl ColorParam) -> &mut Self {
        if let Some(color) = color.to_val() {
            self.target.insert(ui::BorderColor(color));
        } else {
            self.target.remove::<ui::BorderColor>();
        }
        self
    }
}

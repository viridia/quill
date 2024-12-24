use super::builder::{StyleBuilder, ZIndexParam};
use bevy::ui::ZIndex;

#[allow(missing_docs)]
pub trait StyleBuilderZIndex {
    fn z_index(&mut self, index: impl ZIndexParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderZIndex for StyleBuilder<'a, 'w> {
    fn z_index(&mut self, index: impl ZIndexParam) -> &mut Self {
        match index.to_val() {
            ZIndex(0) => self.target.remove::<ZIndex>(),
            val => self.target.insert(val),
        };
        self
    }
}

use super::builder::{BorderRadiusParam, StyleBuilder};

#[allow(missing_docs)]
pub trait StyleBuilderBorderRadius {
    fn border_radius(&mut self, radius: impl BorderRadiusParam) -> &mut Self;
}

impl<'a, 'w> StyleBuilderBorderRadius for StyleBuilder<'a, 'w> {
    fn border_radius(&mut self, radius: impl BorderRadiusParam) -> &mut Self {
        self.target.insert(radius.to_border_radius());
        self
    }
}

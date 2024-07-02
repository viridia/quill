use super::builder::StyleBuilder;
use bevy::render::view::Visibility;

#[allow(missing_docs)]
pub trait StyleBuilderVisibility {
    fn visible(&mut self, visible: bool) -> &mut Self;
}

impl<'a, 'w> StyleBuilderVisibility for StyleBuilder<'a, 'w> {
    fn visible(&mut self, visible: bool) -> &mut Self {
        let visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        match self.target.get_mut::<Visibility>() {
            Some(mut vis) => {
                if *vis != visibility {
                    *vis = visibility
                }
            }
            None => {
                self.target.insert(visibility);
            }
        };
        self
    }
}

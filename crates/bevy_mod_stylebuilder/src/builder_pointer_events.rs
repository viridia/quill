use bevy_mod_picking::picking_core::Pickable;

use super::builder::StyleBuilder;

#[allow(missing_docs)]
pub trait StyleBuilderPointerEvents {
    fn pointer_events(&mut self, enabled: bool) -> &mut Self;
}

impl<'a, 'w> StyleBuilderPointerEvents for StyleBuilder<'a, 'w> {
    fn pointer_events(&mut self, enabled: bool) -> &mut Self {
        match enabled {
            true => self.target.remove::<Pickable>(),
            false => self.target.insert(Pickable {
                should_block_lower: false,
                is_hoverable: false,
            }),
        };
        self
    }
}

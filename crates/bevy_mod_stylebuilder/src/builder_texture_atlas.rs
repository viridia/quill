use bevy::{sprite::TextureAtlas, ui::UiTextureAtlasImage};

use super::builder::{AssetPathParam, StyleBuilder};

#[allow(missing_docs)]
pub trait StyleBuilderTextureAtlas {
    /// Set the background image of the target entity.
    fn texture_atlas<'p>(&mut self, path: impl AssetPathParam<'p>) -> &mut Self;

    /// Set the index of which tile is being used in the texture atlas
    fn texture_atlas_tile(&mut self, index: usize) -> &mut Self;

    /// Set the index of which tile is being used in the texture atlas, and also explicitly
    /// configure the horizontal and vertical flip.
    fn texture_atlas_tile_flipped(
        &mut self,
        tile_index: usize,
        flip_x: bool,
        flip_y: bool,
    ) -> &mut Self;
}

impl<'a, 'w> StyleBuilderTextureAtlas for StyleBuilder<'a, 'w> {
    fn texture_atlas<'p>(&mut self, path: impl AssetPathParam<'p>) -> &mut Self {
        todo!("Implement texture atlas loading");
        // let texture = path
        //     .to_path()
        //     .map(|p| self.load_asset::<TextureAtlas>(p))
        //     .unwrap();
        // self.target.insert(texture);
        // self
    }

    fn texture_atlas_tile(&mut self, index: usize) -> &mut Self {
        match self.target.get_mut::<UiTextureAtlasImage>() {
            Some(mut uii) => {
                uii.index = index;
            }
            None => {
                self.target.insert(UiTextureAtlasImage {
                    index,
                    flip_x: false,
                    flip_y: false,
                });
            }
        };
        self
    }

    fn texture_atlas_tile_flipped(
        &mut self,
        index: usize,
        flip_x: bool,
        flip_y: bool,
    ) -> &mut Self {
        match self.target.get_mut::<UiTextureAtlasImage>() {
            Some(mut uii) => {
                uii.index = index;
                uii.flip_x = flip_x;
                uii.flip_y = flip_y;
            }
            None => {
                self.target.insert(UiTextureAtlasImage {
                    index,
                    flip_x,
                    flip_y,
                });
            }
        };
        self
    }
}

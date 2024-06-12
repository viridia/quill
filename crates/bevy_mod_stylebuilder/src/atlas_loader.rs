use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    math::{Rect, Vec2},
    sprite::{TextureAtlas, TextureAtlasLayout},
};
use serde::{Deserialize, Serialize};

pub struct TextureAtlasLoader;

#[derive(Debug, Deserialize, Serialize)]
struct TextureAtlasSer {
    texture: String,
    size: Vec2,
    textures: Vec<Rect>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TextureAtlasGridSer {
    texture: String,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
}

impl AssetLoader for TextureAtlasLoader {
    type Asset = TextureAtlasLayout;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        if let Some(ext) = load_context.asset_path().get_full_extension() {
            if ext == "atlas.grid.ron" {
                return Box::pin(async move {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                    let atlas_ser: TextureAtlasGridSer =
                        ron::de::from_str(&String::from_utf8(bytes)?)?;
                    let texture_path = load_context
                        .asset_path()
                        .resolve_embed(&atlas_ser.texture)?;
                    let texture = load_context.load(&texture_path);
                    let result = TextureAtlas::from_grid(
                        texture,
                        atlas_ser.tile_size,
                        atlas_ser.columns,
                        atlas_ser.rows,
                        atlas_ser.padding,
                        atlas_ser.offset,
                    );
                    Ok(result)
                });
            }
        }

        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let atlas_ser: TextureAtlasSer = ron::de::from_str(&String::from_utf8(bytes)?)?;
            let texture_path = load_context
                .asset_path()
                .resolve_embed(&atlas_ser.texture)?;
            let texture = load_context.load(&texture_path);
            let mut result = TextureAtlas::new_empty(texture, atlas_ser.size);
            for texture in atlas_ser.textures.iter() {
                result.add_texture(*texture);
            }
            Ok(result)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["atlas.ron", "atlas.grid.ron"]
    }
}

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub(crate) struct SwatchRectMaterial {
    #[uniform(0)]
    pub(crate) color: Vec4,
    #[uniform(1)]
    pub(crate) border_radius: Vec4,
}

impl UiMaterial for SwatchRectMaterial {
    fn fragment_shader() -> ShaderRef {
        "obsidian_ui://shaders/swatch_rect.wgsl".into()
    }
}

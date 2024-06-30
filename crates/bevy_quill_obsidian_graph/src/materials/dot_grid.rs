use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct DotGridMaterial {
    #[uniform(0)]
    pub(crate) color_bg: Vec4,
    #[uniform(1)]
    pub(crate) color_fg: Vec4,
}

impl UiMaterial for DotGridMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_quill_obsidian_graph/assets/dot_grid.wgsl".into()
    }
}

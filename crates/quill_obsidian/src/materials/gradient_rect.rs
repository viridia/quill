use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub(crate) struct GradientRectMaterial {
    #[uniform(0)]
    pub(crate) num_color_stops: i32,
    #[uniform(1)]
    pub(crate) color_stops: [Vec4; 8],
    #[uniform(3)]
    pub(crate) cap_size: f32,
}

impl UiMaterial for GradientRectMaterial {
    fn fragment_shader() -> ShaderRef {
        "obsidian_ui://shaders/gradient_rect.wgsl".into()
    }
}

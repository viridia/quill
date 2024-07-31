#import bevy_core_pipeline::tonemapping::tone_mapping
#import bevy_pbr::{
    mesh_view_bindings::view,
    mesh_functions as mfns,
    mesh_bindings::mesh,
}

@group(2) @binding(100)
var<uniform> color: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return tone_mapping(color, view.color_grading);
}

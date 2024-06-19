#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> color_bg: vec4<f32>;

@group(1) @binding(1)
var<uniform> color_fg: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let size = vec2<f32>(in.size.x, in.size.y);
    let cell = fract(in.uv * size /  16.) * 16.;
    return select(color_bg, color_fg, cell.x <= 1.5 && cell.y <= 1.5);
}

// This shader draws a two-toned rounded rect for a slider widget.
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> color_lo: vec4<f32>;

@group(1) @binding(1)
var<uniform> color_hi: vec4<f32>;

@group(1) @binding(2)
var<uniform> value: vec4<f32>;

@group(1) @binding(3)
var<uniform> radius: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv - 0.5;
    let size = vec2<f32>(in.size.x, in.size.y);
    let color = select(color_lo, color_hi, in.uv.x <= value.x);
    let external_distance = sd_rounded_box((in.uv - 0.5) * size, size, vec4<f32>(radius));
    let alpha = smoothstep(0.5, -0.5, external_distance);

    return vec4<f32>(color.rgb, alpha);
}

// From: https://github.com/bevyengine/bevy/pull/8973
// The returned value is the shortest distance from the given point to the boundary of the rounded box.
// Negative values indicate that the point is inside the rounded box, positive values that the point is outside, and zero is exactly on the boundary.
// arguments
// point -> The function will return the distance from this point to the closest point on the boundary.
// size -> The maximum width and height of the box.
// corner_radii -> The radius of each rounded corner. Ordered counter clockwise starting top left:
//                      x = top left, y = top right, z = bottom right, w = bottom left.
fn sd_rounded_box(point: vec2<f32>, size: vec2<f32>, corner_radii: vec4<f32>) -> f32 {
    // if 0.0 < y then select bottom left (w) and bottom right corner radius (z)
    // else select top left (x) and top right corner radius (y)
    let rs = select(corner_radii.xy, corner_radii.wz, 0.0 < point.y);
    // w and z are swapped so that both pairs are in left to right order, otherwise this second select statement would return the incorrect value for the bottom pair.
    let radius = select(rs.x, rs.y, 0.0 < point.x);
    // Vector from the corner closest to the point, to the point
    let corner_to_point = abs(point) - 0.5 * size;
    // Vector from the center of the radius circle to the point
    let q = corner_to_point + radius;
    // length from center of the radius circle to the point, 0s a component if the point is not within the quadrant of the radius circle that is part of the curved corner.
    let l = length(max(q, vec2(0.0)));
    let m = min(max(q.x, q.y), 0.0);
    return l + m - radius;
}

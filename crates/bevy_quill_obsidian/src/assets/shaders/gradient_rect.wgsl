// This shader draws a rounded rect with a given input color
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> num_color_stops: vec4<i32>;

@group(1) @binding(1)
var<uniform> color_stops: array<vec4<f32>, 8>;

@group(1) @binding(3)
var<uniform> cap_size: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let t = (in.uv.x - 0.1) * 1.0 / 0.8 * f32(num_color_stops.x - 1);
    let color_index_lo = clamp(i32(floor(t)), 0, num_color_stops.x - 1);
    let color_index_hi = clamp(i32(ceil(t)), 0, num_color_stops.x - 1);
    let color_lo = color_stops[color_index_lo];
    let color_hi = color_stops[color_index_hi];
    let color = mix(color_lo, color_hi, t - f32(color_index_lo));

    let uv = (in.uv - vec2<f32>(0.5, 0.5)) * in.size / 8.;
    let check = select(0.0, 1.0, (fract(uv.x) < 0.5) != (fract(uv.y) < 0.5));
    let bg = mix(vec3<f32>(0.4, 0.4, 0.4), vec3<f32>(0.6, 0.6, 0.6), check);
    let c = srgb_to_linear(mix(bg, color.rgb, color.w));

    let size = vec2<f32>(in.size.x, in.size.y);
    let external_distance = sd_rounded_box((in.uv - 0.5) * size, size, vec4<f32>(size.y * 0.5));
    let alpha = smoothstep(0.5, -0.5, external_distance);

    return vec4<f32>(c, alpha);
}

// Convert sRGB to linear color space because we interpolate in sRGB space.
fn srgb_to_linear(srgb: vec3<f32>) -> vec3<f32> {
    let a = 0.055;
    let srgbLow = srgb / 12.92;
    let srgbHigh = pow((srgb + a) / (1.0 + a), vec3<f32>(2.4, 2.4, 2.4));
    let linear = mix(srgbLow, srgbHigh, step(vec3<f32>(0.04045, 0.04045, 0.04045), srgb));
    return linear;
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

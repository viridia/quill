fn bricks(
    uv: vec2<f32>,
    x_count: i32,
    y_count: i32,
    x_spacing: f32,
    y_spacing: f32,
    x_blur: f32,
    y_blur: f32,
    stagger: f32,
    corner: i32) -> f32 {
  let y = uv.y * f32(y_count);
  let yr = floor(y);
  let yi = floor(y + 0.5);
  let yf = smootherstep(y_spacing, y_spacing + y_blur, abs(y - yi));
  let x = uv.x * f32(x_count) + select(0.0, stagger, floor(yr * 0.5) * 2.0 == yr);
  let xi = floor(x + 0.5);
  let xf = smootherstep(x_spacing, x_spacing + x_blur, abs(x - xi));
  var value: f32;
  if corner == 1 { // Mitered
    value = max(0., (xf + yf) - 1.0);
  } else if corner == 2 { // Rounded
    value = max(0., 1. - sqrt((1.-xf) * (1.-xf) + (1.-yf) * (1.-yf)));
  } else { // Square
    value = min(xf, yf);
  }
  return value;
}

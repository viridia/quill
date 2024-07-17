// Like smoothstep, but with a smoother transition between the low and high values.
fn smootherstep(low: f32, high: f32, t: f32) -> f32 {
  if (t <= low) { return 0.0; }
  if (t >= high) { return 1.0; }
  let e = (t - low) / (high - low);
  return e * e * e * (e * (e * 6.0 - 15.0) + 10.0);
}

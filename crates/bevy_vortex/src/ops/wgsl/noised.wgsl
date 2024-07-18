// https://iquilezles.org/articles/gradientnoise/
// returns 3D value noise (in .x)  and its derivatives (in .yzw)
// MIT License. © Inigo Quilez - https://iquilezles.org/articles/morenoise/
fn noised(x: vec3<f32>) -> vec4<f32> {
  let p = vec3<i32>(floor(x));
  let w = fract(x);

  let u = w * w * w * (w * (w * 6.0 - 15.0) + 10.0);
  let du = 30.0 * w * w * (w * (w - 2.0) + 1.0);

  let a = hash(p + vec3<i32>(0, 0, 0));
  let b = hash(p + vec3<i32>(1, 0, 0));
  let c = hash(p + vec3<i32>(0, 1, 0));
  let d = hash(p + vec3<i32>(1, 1, 0));
  let e = hash(p + vec3<i32>(0, 0, 1));
  let f = hash(p + vec3<i32>(1, 0, 1));
  let g = hash(p + vec3<i32>(0, 1, 1));
  let h = hash(p + vec3<i32>(1, 1, 1));

  let k0 =   a;
  let k1 =   b - a;
  let k2 =   c - a;
  let k3 =   e - a;
  let k4 =   a - b - c + d;
  let k5 =   a - c - e + g;
  let k6 =   a - b - e + f;
  let k7 = - a + b + c - d + e - f - g + h;

  return
    vec4<f32>( -1.0 + 2.0 * (k0 + k1*u.x + k2*u.y + k3*u.z + k4*u.x*u.y + k5*u.y*u.z + k6*u.z*u.x + k7*u.x*u.y*u.z),
                2.0 * du * vec3<f32>(k1 + k4*u.y + k6*u.z + k7*u.y*u.z,
                                     k2 + k5*u.z + k4*u.x + k7*u.z*u.x,
                                     k3 + k6*u.x + k5*u.y + k7*u.x*u.y ) );
}

fn hash(n: vec3<i32>) -> f32
{
  return fract(10000.0 * sin(f32(n.x + n.y * 13 + n.z * 17)));
}

fn noised_octaves(
    v: vec3<f32>,
    scale: f32,
    octaves: i32,
    roughness: f32,
    distortion: f32) -> vec4<f32> {

    var pos = v * scale;
    var result = vec4<f32>(0.0);
    var coeff = 1.0;
    var total = 0.0;
    for (var i = 0u; i < u32(octaves); i++) {
        result += noised(pos) * coeff;
        total += coeff;
        coeff *= roughness;
        // pos *= distortion;
        pos *= 2.0;
    }
    return result / total * vec4f(0.5, 1.0, 1.0, 1.0) + vec4f(0.5, 0.0, 0.0, 0.0);
}

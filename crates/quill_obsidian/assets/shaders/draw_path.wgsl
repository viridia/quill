#import bevy_ui::ui_vertex_output::UiVertexOutput

const OP_MOVE_TO: u32 = 0u;
const OP_LINE_TO: u32 = 1u;
const OP_QUAD1: u32 = 2u;
const OP_QUAD2: u32 = 3u;

struct PathCommand {
    op: u32,
    pos: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@group(1) @binding(1)
var<uniform> width: f32;

@group(1) @binding(2)
var<storage> commands: array<PathCommand>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let pt = vec2<f32>(in.size.x, in.size.y) * in.uv;
    let d = distance_to_path(pt);
    let a = 1.0 - smoothstep(width * 0.5 - 0.3, width * 0.5 + 0.3, d);
    return vec4<f32>(color.rgb, color.a * a);
}

fn distance_to_path(pt: vec2<f32>) -> f32 {
    var prev = vec2<f32>(0., 0.);
    var dist: f32 = 10000000.0;
    let n = arrayLength(&commands);
    for (var i = 0u; i < n; i = i + 1u) {
        let cmd = commands[i];
        if (cmd.op == OP_MOVE_TO) {
            prev = cmd.pos;
        } else if (cmd.op == OP_LINE_TO) {
            let next = cmd.pos;
            dist = min(dist, distance_sq_to_line(pt, prev, next));
            prev = next;
        } else if (cmd.op == OP_QUAD1) {
            let ctrl = cmd.pos;
            let next = commands[i + 1].pos;
            dist = min(dist, distance_sq_to_quadratic(pt, prev, ctrl, next));
            i = i + 1u;
            prev = next;
        } else if (cmd.op == OP_QUAD2) {
            prev = cmd.pos;
        }
    }
    return sqrt(dist);
}

fn distance_sq_to_line(pt: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = pt - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return dot2(pa - ba * h);
}

// From https://iquilezles.org/articles/distfunctions2d/
fn distance_sq_to_quadratic(pos: vec2<f32>, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>) -> f32 {
    let a = B - A;
    let b = A - 2.0 * B + C;
    let c = a * 2.0;
    let d = A - pos;
    let kk = 1.0 / dot(b, b);
    let kx = kk * dot(a, b);
    let ky = kk * (2.0 * dot(a, a)+dot(d, b)) / 3.0;
    let kz = kk * dot(d, a);
    var res = 0.0;
    let p = ky - kx * kx;
    let p3 = p * p * p;
    let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
    var h = q * q + 4.0 * p3;
    if (h >= 0.0) {
        h = sqrt(h);
        let x = (vec2<f32>(h, -h) - q) / 2.0;
        let uv = sign(x) * pow(abs(x), vec2(1.0 / 3.0));
        let t = clamp(uv.x + uv.y - kx, 0.0, 1.0);
        res = dot2(d + (c + b * t) * t);
    } else {
        let z = sqrt(-p);
        let v = acos( q/(p * z * 2.0) ) / 3.0;
        let m = cos(v);
        let n = sin(v) * 1.732050808;
        let t = clamp(vec3<f32>(m + m,-n - m,n - m) * z - kx, vec3<f32>(0.0), vec3<f32>(1.0));
        res = min(dot2(d + (c + b * t.x) * t.x),
                  dot2(d + (c + b * t.y) * t.y));
        // the third root cannot be the closest
        // res = min(res,dot2(d+(c+b*t.z)*t.z));
    }
    return res;
}

fn dot2(v: vec2<f32>) -> f32 {
    return dot(v, v);
}

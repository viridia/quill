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
            if are_colinear(prev, ctrl, next) {
                // If points are co-linear, then just draw a line.
                dist = min(dist, distance_sq_to_line(pt, prev, next));
            } else {
                dist = min(dist, distance_sq_to_quadratic(pt, prev, ctrl, next));
            }
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
// fn distance_sq_to_quadratic2(pos: vec2<f32>, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>) -> f32 {
//     let a = B - A;
//     let b = A - 2.0 * B + C;
//     let c = a * 2.0;
//     let d = A - pos;
//     let kk = 1.0 / dot(b, b);
//     let kx = kk * dot(a, b);
//     let ky = kk * (2.0 * dot(a, a)+dot(d, b)) / 3.0;
//     let kz = kk * dot(d, a);
//     var res = 0.0;
//     let p = ky - kx * kx;
//     let p3 = p * p * p;
//     let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
//     var h = q * q + 4.0 * p3;
//     if (h >= 0.0) {
//         h = sqrt(h);
//         let x = (vec2<f32>(h, -h) - q) / 2.0;
//         let uv = sign(x) * pow(abs(x), vec2(1.0 / 3.0));
//         let t = clamp(uv.x + uv.y - kx, 0.0, 1.0);
//         res = dot2(d + (c + b * t) * t);
//     } else {
//         let z = sqrt(-p);
//         let v = acos( q/(p * z * 2.0) ) / 3.0;
//         let m = cos(v);
//         let n = sin(v) * 1.732050808;
//         let t = clamp(vec3<f32>(m + m,-n - m,n - m) * z - kx, vec3<f32>(0.0), vec3<f32>(1.0));
//         res = min(dot2(d + (c + b * t.x) * t.x),
//                   dot2(d + (c + b * t.y) * t.y));
//         // the third root cannot be the closest
//         // res = min(res,dot2(d+(c+b*t.z)*t.z));
//     }
//     return res;
// }

// This method provides just an approximation, and is only usable in
// the very close neighborhood of the curve. Taken and adapted from
// http://research.microsoft.com/en-us/um/people/hoppe/ravg.pdf
fn distance_sq_to_quadratic(p: vec2<f32>, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>) -> f32
{
    var v0: vec2<f32> = A;
    var v1: vec2<f32> = B;
    var v2: vec2<f32> = C;
	let i: vec2<f32> = v0 - v2;
    let j: vec2<f32> = v2 - v1;
    let k: vec2<f32> = v1 - v0;
    let w: vec2<f32> = j-k;

	v0 -= p; v1 -= p; v2 -= p;

	let x: f32 = cro(v0, v2);
    let y: f32 = cro(v1, v0);
    let z: f32 = cro(v2, v1);

	let s: vec2<f32> = 2.0*(y*j+z*k)-x*i;

    let r: f32 =  (y*z-x*x*0.25)/dot2(s);
    let t: f32 = clamp( (0.5*x+y+r*dot(s,w))/(x+y+z),0.0,1.0);

    let d: vec2<f32> = v0+t*(k+k+t*w);
	return d.x * d.x + d.y * d.y;
}

fn dot2(v: vec2<f32>) -> f32 {
    return dot(v, v);
}

fn cro( a: vec2<f32>, b: vec2<f32> ) -> f32 { return a.x*b.y-a.y*b.x; }

fn are_colinear(p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> bool {
    let epsilon: f32 = 1e-6;
    let area = abs((p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)) / 2.0);
    return area < epsilon;
}

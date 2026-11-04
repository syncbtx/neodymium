// Noise glow + drifting rotating rectangles backdrop, for amane's Rectangle::shader.
// WGSL port of the Shadertoy shader; iTime is replaced by the injected phase.
//
// amane provides: uv (0..1, y down), `size` (px), `time`, and `values` (16 x vec4).
//
// values layout (set with .shader_values(vec![...])):
//   [0] background color, straight RGBA (alpha ignored, output is opaque)
//   [1] rectangle color,  straight RGBA (alpha ignored)
//   [2] (phase, count, min_size, size_range)
//       phase:      injected by the caller, plays the role of iTime
//       count:      number of rectangles (original: 60, capped at 256)
//       min_size:   smallest rectangle edge, in half-height units (original: 0.03)
//       size_range: max size - min size (original: 0.05)
//   [3] (time_scale, 0, 0, 0)
//       time_scale: multiplies phase before use (1.0 = original pacing if
//                   phase advances ~1 per second)

// noise background
const NOISE_INTENSITY: f32 = 2.8;
const NOISE_DEFINITION: f32 = 0.6;
const GLOW_POS: vec2<f32> = vec2<f32>(-2.0, 0.0);

// rectangles
const Y_DISTRIBUTION: f32 = 0.5;
const MAX_COUNT: f32 = 256.0;

fn random(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn noise(p_in: vec2<f32>) -> f32 {
    let p = p_in * NOISE_INTENSITY;
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(random(i + vec2<f32>(0.0, 0.0)), random(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(random(i + vec2<f32>(0.0, 1.0)), random(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y,
    );
}

fn fbm(uv_in: vec2<f32>) -> f32 {
    var uv = uv_in * 5.0;
    // column-major, same layout as the GLSL mat2
    let m = mat2x2<f32>(1.6, 1.2, -1.2, 1.6);
    var f = 0.5000 * noise(uv);
    uv = m * uv;
    f += 0.2500 * noise(uv);
    uv = m * uv;
    f += 0.1250 * noise(uv);
    uv = m * uv;
    f += 0.0625 * noise(uv);
    return 0.5 + 0.5 * f;
}

fn bg(uv_in: vec2<f32>, t: f32, bg_color: vec3<f32>) -> vec3<f32> {
    var uv = uv_in;
    let velocity = t / 1.6;
    let intensity = sin(uv.x * 3.0 + velocity * 2.0) * 1.1 + 1.5;
    uv.y -= 2.0;
    let bp = uv + GLOW_POS;
    uv *= NOISE_DEFINITION;

    // ripple
    let rb = fbm(vec2<f32>(uv.x * 0.5 - velocity * 0.03, uv.y)) * 0.1;
    uv += vec2<f32>(rb, rb);

    // coloring
    var rz = fbm(uv * 0.9 + vec2<f32>(velocity * 0.35, 0.0));
    rz *= dot(bp * intensity, bp) + 1.2;

    let col = bg_color / (0.1 - rz);
    return sqrt(abs(col));
}

fn rectangle(uv: vec2<f32>, pos_in: vec2<f32>, width: f32, height: f32, blur_in: f32) -> f32 {
    let blur = max(blur_in, 1.0e-4); // smoothstep needs edge0 < edge1
    let pos = (vec2<f32>(width, height) + 0.01) / 2.0 - abs(uv - pos_in);
    let s = smoothstep(vec2<f32>(0.0), vec2<f32>(blur), pos);
    return s.x * s.y;
}

fn rotate2d(a: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(a), -sin(a), sin(a), cos(a));
}

@fragment
fn main(@location(0) uv_in: vec2<f32>) -> @location(0) vec4<f32> {
    let bg_color = values[0].rgb;
    let rect_color = values[1].rgb;

    let t = values[2].x * values[3].x;
    let total = clamp(floor(values[2].y), 0.0, MAX_COUNT);
    let min_size = values[2].z;
    let max_size = values[2].w;

    // shadertoy space: y up, -1..1 vertically, x scaled by aspect
    let aspect = size.x / size.y;
    var uv = vec2<f32>(uv_in.x, 1.0 - uv_in.y) * 2.0 - 1.0;
    uv.x *= aspect;

    // bg
    var color = bg(uv, t, bg_color) * (2.0 - abs(uv.y * 2.0));

    // rectangles
    let vel_x = -t / 8.0;
    let vel_y = t / 10.0;

    for (var i = 0.0; i < total; i += 1.0) {
        let index = i / total;
        let rnd = random(vec2<f32>(index, index));

        // x travels across the full width, whatever the aspect ratio
        let px = fract(vel_x * rnd + index) * (2.0 * aspect) - aspect;
        let py = sin(index * rnd * 1000.0 + vel_y) * Y_DISTRIBUTION;
        let ps = max_size * rnd + min_size;
        let pos = vec2<f32>(px, py);

        var uv_rot = uv - pos + vec2<f32>(ps * 0.5);
        uv_rot = rotate2d(i + t / 2.0) * uv_rot;
        uv_rot += pos + vec2<f32>(ps * 0.5);

        let blur = (max_size + min_size - ps) / 2.0;
        let rect = rectangle(uv_rot, pos, ps, ps, blur);
        color += rect_color * rect * (ps / max(max_size, 1.0e-4));
    }

    return vec4<f32>(color, 1.0);
}

// Wavy progress ring / bar as an SDF fragment shader, for amane's Rectangle::shader.
//
// amane provides: uv (0..1), `size` (px), `time`, and `values` (16 x vec4).
//
// values layout (set with .shader_values(vec![...])):
//   [0] normal color,  straight RGBA
//   [1] track color,   straight RGBA
//   [2] (thickness, amplitude, freq, phase)
//       freq: ring = waves per turn, bar = wavelength in px; phase in radians
//   [3] (progress 0..1, mode 0 = ring / 1 = bar, warn threshold, error threshold)
//   [4] warning color, straight RGBA
//   [5] error color,   straight RGBA
//
// progress below `warn` uses the normal color, from `warn` up the warning
// color, from `error` up the error color. Thresholds are on the same 0..1 scale.

struct P {
    color: vec4<f32>,
    track: vec4<f32>,
    thickness: f32,
    amplitude: f32,
    freq: f32,
    phase: f32,
    progress: f32,
    bar: bool,
};

const TAU: f32 = 6.28318530718;
const FAR: f32 = 1.0e5;

fn cover(d: f32) -> f32 {
    return clamp(0.5 - d, 0.0, 1.0);
}

// returns (track distance, fill distance)
fn ring(px: vec2<f32>, u: P) -> vec2<f32> {
    let mid = size.x * 0.5;
    let hw = u.thickness * 0.5;

    let radius = mid - u.amplitude - hw;

    let p = px - vec2<f32>(mid, mid);
    let r = length(p);

    let track_d = r - mid;

    let sweep = TAU * clamp(u.progress, 0.0, 1.0);
    var fill_d = FAR;

    if (sweep > 0.01) {
        let th = (atan2(p.x, -p.y) + TAU) % TAU;

        let waves = select(0.0, TAU * radius / u.freq, u.freq > 0.0);

        let sc = vec2<f32>(sin(waves * th + u.phase), cos(waves * th + u.phase));
        let rho = radius + u.amplitude * sc.x;
        let drho = u.amplitude * waves * sc.y;
        let rr = max(r, 1.0e-3);
        let k = sqrt(1.0 + (drho * drho) / (rr * rr));

        if (th <= sweep) {
            fill_d = abs(r - rho) / k - hw;
        }

        let r0 = radius + u.amplitude * sin(u.phase);
        let c0 = vec2<f32>(0.0, -r0);

        let r1 = radius + u.amplitude * sin(waves * sweep + u.phase);
        let c1 = vec2<f32>(r1 * sin(sweep), -r1 * cos(sweep));

        fill_d = min(fill_d, min(length(p - c0), length(p - c1)) - hw);
    }

    return vec2<f32>(track_d, fill_d);
}

fn bar(px: vec2<f32>, u: P) -> vec2<f32> {
    let hw = u.thickness * 0.5;
    let mid = size.y * 0.5;
    let x0 = hw;
    let x1 = size.x - hw;

    let q = vec2<f32>(clamp(px.x, x0, x1), mid);
    let track_d = length(px - q) - hw;

    let xe = x0 + (x1 - x0) * clamp(u.progress, 0.0, 1.0);
    var fill_d = FAR;

    if (xe - x0 > 0.01) {
        let k = select(0.0, TAU / u.freq, u.freq > 0.0);
        let arg = k * (px.x - x0) + u.phase;
        let y = mid + u.amplitude * sin(arg);
        let dy = u.amplitude * k * cos(arg);

        if (px.x >= x0 && px.x <= xe) {
            fill_d = abs(px.y - y) / sqrt(1.0 + dy * dy) - hw;
        }

        let e0 = vec2<f32>(x0, mid + u.amplitude * sin(u.phase));
        let e1 = vec2<f32>(xe, mid + u.amplitude * sin(k * (xe - x0) + u.phase));
        fill_d = min(fill_d, min(length(px - e0), length(px - e1)) - hw);
    }

    return vec2<f32>(track_d, fill_d);
}

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // normal / warning / error by progress
    var base = values[0];
    if (values[3].x >= values[3].w) {
        base = values[5];
    } else if (values[3].x >= values[3].z) {
        base = values[4];
    }

    let u = P(
        base,
        values[1],
        values[2].x,
        values[2].y,
        values[2].z,
        values[2].w,
        values[3].x,
        values[3].y > 0.5,
    );

    let px = uv * size;

    var d: vec2<f32>;
    if (u.bar) {
        d = bar(px, u);
    } else {
        d = ring(px, u);
    }

    let ta = cover(d.x) * u.track.a;
    let fa = cover(d.y) * u.color.a;

    let t = vec4<f32>(u.track.rgb * ta, ta);
    let f = vec4<f32>(u.color.rgb * fa, fa);

    // premultiplied composite, then back to straight alpha like amane's examples return
    let out = f + t * (1.0 - f.a);
    return vec4<f32>(out.rgb / max(out.a, 1.0e-4), out.a);
}

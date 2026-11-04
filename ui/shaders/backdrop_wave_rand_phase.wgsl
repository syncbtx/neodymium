// Drifting wave backdrop as an SDF fragment shader, for amane's Rectangle::shader.
//
// Amplitude and wavelength evolve randomly over time, entirely on the GPU.
// No per-frame CPU state: the only moving input is amane's `time`.
//
// values layout (set with .shader_values(vec![...])):
//   [0] wave color,  straight RGBA
//   [1] track color, straight RGBA (faint baseline; alpha 0 to disable)
//   [2] (thickness, max_amplitude, min_wavelength, max_wavelength)   all px
//   [3] (seed, drift_speed, phase, 0)
//       seed:         any float, different per instance
//       drift_speed:  how fast amp/wavelength mix evolves (~0.1..0.5 is calm)
//       phase:        radians, injected by the caller (share it with the other
//                     components so every wave travels in lockstep)
//
// How it avoids the "sliding" problem of changing freq directly:
// the wave is a fixed set of three sines (long / mid / short wavelength).
// Randomness only changes their *weights* and the overall amplitude, so the
// shape morphs smoothly and the dominant wavelength drifts without any
// phase discontinuity.

const TAU: f32 = 6.28318530718;

fn cover(d: f32) -> f32 {
    return clamp(0.5 - d, 0.0, 1.0);
}

fn hash11(n: f32) -> f32 {
    return fract(sin(n * 127.1 + 311.7) * 43758.5453123);
}

// smooth 1D value noise, range 0..1, C2-continuous (quintic fade)
fn noise(x: f32) -> f32 {
    let i = floor(x);
    let f = x - i;
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    return mix(hash11(i), hash11(i + 1.0), u);
}

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color = values[0];
    let track = values[1];
    let hw = values[2].x * 0.5;
    let mid = size.y * 0.5;

    // keep the wave inside the rect
    let amp_max = min(values[2].y, max(mid - hw - 1.0, 0.0));
    let wl_min = max(values[2].z, 2.0);
    let wl_max = max(values[2].w, wl_min + 1.0);

    let seed = values[3].x;
    let drift = values[3].y;
    let phase = values[3].z;

    let t = time * drift;

    // overall amplitude breathes between 15% and 100% of max
    let amp = amp_max * mix(0.15, 1.0, noise(t * 0.8 + seed * 3.7));

    // three fixed wavelengths, geometrically spaced
    let wl0 = wl_max;
    let wl1 = sqrt(wl_min * wl_max);
    let wl2 = wl_min;

    let k0 = TAU / wl0;
    let k1 = TAU / wl1;
    let k2 = TAU / wl2;

    // random weights, squared so one component tends to dominate,
    // then normalised so |y| <= amp
    let n0 = noise(t * 1.00 + seed + 0.0);
    let n1 = noise(t * 1.27 + seed + 17.0);
    let n2 = noise(t * 0.83 + seed + 41.0);
    var w = vec3<f32>(n0 * n0, n1 * n1, n2 * n2) + vec3<f32>(0.02);
    w = w / (w.x + w.y + w.z);

    let px = uv * size;

    // all component phases derive from the injected phase; fixed ratios keep
    // the pattern from simply translating, and seed offsets decorrelate instances
    let ph0 = phase * 1.0 + seed;
    let ph1 = phase * 1.3 + seed * 2.0;
    let ph2 = phase * 0.7 + seed * 3.0;

    let a0 = k0 * px.x + ph0;
    let a1 = k1 * px.x + ph1;
    let a2 = k2 * px.x + ph2;

    let y = mid + amp * (w.x * sin(a0) + w.y * sin(a1) + w.z * sin(a2));
    let dy = amp * (w.x * k0 * cos(a0) + w.y * k1 * cos(a1) + w.z * k2 * cos(a2));

    // first-order distance to the curve
    let wave_d = abs(px.y - y) / sqrt(1.0 + dy * dy) - hw;
    let track_d = abs(px.y - mid) - hw * 0.5;

    let wa = cover(wave_d) * color.a;
    let ta = cover(track_d) * track.a;

    let f = vec4<f32>(color.rgb * wa, wa);
    let tr = vec4<f32>(track.rgb * ta, ta);

    let out = f + tr * (1.0 - f.a);
    return vec4<f32>(out.rgb / max(out.a, 1.0e-4), out.a);
}

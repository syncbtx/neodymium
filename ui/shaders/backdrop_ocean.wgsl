// Side-view ocean surface as an SDF fragment shader, for amane's Rectangle::shader.
//
// A filled body of water under a drifting wave surface, seen from the side.
// Same drift idea as backdrop_wave.wgsl: three fixed wavelengths whose weights
// and overall amplitude are modulated by smooth noise of `time`, so the swell
// morphs without any phase discontinuity. Wave travel comes from the injected
// phase. Crests are sharpened (narrow peaks, broad troughs) to read as ocean
// rather than a plain sine.
//
// amane provides: uv (0..1, y down), `size` (px), `time`, and `values` (16 x vec4).
//
// values layout (set with .shader_values(vec![...])):
//   [0] water color at the surface, straight RGBA
//   [1] water color at depth,       straight RGBA
//   [2] (level, max_amplitude, min_wavelength, max_wavelength)
//       level: resting surface height as a fraction of the rect height,
//              measured from the top (0.5 = middle)
//       amplitude and wavelengths in px
//   [3] (seed, drift_speed, phase, sharpness)
//       seed:        any float, different per instance
//       drift_speed: how fast amplitude/wavelength mix evolves (~0.1..0.5 is calm)
//       phase:       radians, injected by the caller
//       sharpness:   crest sharpening, 1 = plain sine, ~2..3 = ocean-like
//   [4] crest/foam line color, straight RGBA
//   [5] (line_thickness, depth_fade, 0, 0)
//       line_thickness: px, thickens toward the crests
//       depth_fade:     px below the surface over which surface color blends
//                       into the depth color

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

// sharpened sine: returns (height in -1..1, d height / d a)
// u = (1 + sin a) / 2 in 0..1, height = 2 u^p - 1; p > 1 narrows crests
fn crest(a: f32, p: f32) -> vec2<f32> {
    let u = max(0.5 + 0.5 * sin(a), 1.0e-4);
    let up = pow(u, p - 1.0);
    return vec2<f32>(2.0 * up * u - 1.0, p * up * cos(a));
}

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let water_top = values[0];
    let water_deep = values[1];
    let foam = values[4];

    let hw = values[5].x * 0.5;
    let depth_fade = max(values[5].y, 1.0);

    let level = clamp(values[2].x, 0.0, 1.0) * size.y;

    // keep the surface (and its thickened crest line) inside the rect
    let room = min(level, size.y - level);
    let amp_max = min(values[2].y, max(room - hw * 2.0 - 1.0, 0.0));
    let wl_min = max(values[2].z, 2.0);
    let wl_max = max(values[2].w, wl_min + 1.0);

    let seed = values[3].x;
    let drift = values[3].y;
    let phase = values[3].z;
    let sharp = max(values[3].w, 1.0);

    let t = time * drift;

    // overall swell breathes between 15% and 100% of max
    let amp = amp_max * mix(0.15, 1.0, noise(t * 0.8 + seed * 3.7));

    // three fixed wavelengths, geometrically spaced
    let k0 = TAU / wl_max;
    let k1 = TAU / sqrt(wl_min * wl_max);
    let k2 = TAU / wl_min;

    // random weights, squared so one component tends to dominate,
    // then normalised so |h| <= 1
    let n0 = noise(t * 1.00 + seed + 0.0);
    let n1 = noise(t * 1.27 + seed + 17.0);
    let n2 = noise(t * 0.83 + seed + 41.0);
    var w = vec3<f32>(n0 * n0, n1 * n1, n2 * n2) + vec3<f32>(0.02);
    w = w / (w.x + w.y + w.z);

    let px = uv * size;

    // component phases derive from the injected phase; fixed ratios keep the
    // pattern from simply translating, seed offsets decorrelate instances
    let a0 = k0 * px.x + phase * 1.0 + seed;
    let a1 = k1 * px.x + phase * 1.3 + seed * 2.0;
    let a2 = k2 * px.x + phase * 0.7 + seed * 3.0;

    let c0 = crest(a0, sharp);
    let c1 = crest(a1, sharp);
    let c2 = crest(a2, sharp);

    let h = w.x * c0.x + w.y * c1.x + w.z * c2.x;
    let dh = w.x * k0 * c0.y + w.y * k1 * c1.y + w.z * k2 * c2.y;

    // y is down, so crests (positive h) move the surface up
    let y = level - amp * h;
    let dy = -amp * dh;
    let norm = sqrt(1.0 + dy * dy);

    // water body: everything below the surface curve
    let fill = cover((y - px.y) / norm);

    let depth = max(px.y - y, 0.0);
    let td = smoothstep(0.0, 1.0, clamp(depth / depth_fade, 0.0, 1.0));
    let body = mix(water_top, water_deep, td);

    // crest line, thicker where the surface is high
    let crest01 = clamp(0.5 + 0.5 * h, 0.0, 1.0);
    let hw_f = hw * mix(0.6, 2.0, crest01);
    let foam_d = abs(px.y - y) / norm - hw_f;

    let ba = fill * body.a;
    let fa = cover(foam_d) * foam.a;

    let b = vec4<f32>(body.rgb * ba, ba);
    let f = vec4<f32>(foam.rgb * fa, fa);

    // premultiplied composite, then back to straight alpha
    let out = f + b * (1.0 - f.a);
    return vec4<f32>(out.rgb / max(out.a, 1.0e-4), out.a);
}

// Layered neon sine waves with a cosine palette, for amane's Rectangle::shader.
// WGSL port of a Shadertoy shader, reworked to work in pixels so thin bars
// don't alias; iTime is replaced by the injected phase.
//
// amane provides: uv (0..1, y down), `size` (px), `time`, and `values` (16 x vec4).
//
// values layout (set with .shader_values(vec![...])):
//   [0] palette a (rgb)        original: (0.5, 0.5, 0.5)
//   [1] palette b (rgb)        original: (0.5, 0.5, 0.5)
//   [2] palette c (rgb)        original: (1.0, 1.0, 1.0)
//   [3] palette d (rgb)        original: (0.1, 0.4, 0.5)
//   [4] background color, straight RGBA. Alpha 0 gives glowing lines on a
//       transparent rect, alpha 1 an opaque backdrop
//   [5] (phase, layers, amplitude, wavelength)
//       phase:      radians, injected by the caller; layers travel at
//                   phase * (1 - layer), so the front layers move slowest
//       layers:     number of wave layers (capped at 64)
//       amplitude:  wave height as a fraction of half the rect height (0.25)
//       wavelength: px
//   [6] (thickness, edge_thickness, palette_x, palette_phase)
//       thickness:      line glow half-width in px. Keep >= ~2 or the lines
//                       fall between pixels and read as static
//       edge_thickness: extra px of width reached at the left/right ends
//                       (scales with x^8, so it only flares at the ends)
//       palette_x:      palette shift across the width
//       palette_phase:  palette shift per radian of phase
//
// Palette is the usual cosine palette: a + b * cos(2 pi (c t + d)).

const TAU: f32 = 6.28318530718;

fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b * cos(TAU * (c * t + d));
}

@fragment
fn main(@location(0) uv_in: vec2<f32>) -> @location(0) vec4<f32> {
    let pa = values[0].rgb;
    let pb = values[1].rgb;
    let pc = values[2].rgb;
    let pd = values[3].rgb;
    let bg = values[4];

    let t = values[5].x;
    let n = clamp(floor(values[5].y), 1.0, 64.0);
    let amp0 = values[5].z;
    let k = TAU / max(values[5].w, 4.0);

    let thick0 = max(values[6].x, 1.0);
    let edge = values[6].y;
    let pal_x = values[6].z;
    let pal_t = values[6].w;

    let px = uv_in * size;
    let mid = size.y * 0.5;
    let half = mid;

    // -1..1 across the width, for the edge flare and palette
    let xn = uv_in.x * 2.0 - 1.0;
    let thick = thick0 + edge * pow(abs(xn), 8.0);

    // keep the wave inside the rect
    let amp_cap = max(half - thick0 - 1.0, 0.0);

    var glow = vec3<f32>(0.0);

    for (var i = 0.0; i < n; i += 1.0) {
        let layer = i / n;

        let amp = min(amp0 * half * (1.0 + sin(t + layer) * (1.0 - layer)), amp_cap);
        let arg = k * px.x - t * (1.0 - layer);

        let y = mid + amp * sin(arg);
        let dy = amp * k * cos(arg);

        // first-order distance to the curve, in px
        let dist = abs(px.y - y) / sqrt(1.0 + dy * dy);
        let bright = smoothstep(0.0, 1.0, 1.0 - dist / thick);

        let hue = palette(pal_x * xn + layer + pal_t * t, pa, pb, pc, pd);
        glow += vec3<f32>(bright) * hue;
    }

    // additive glow over the background; alpha follows the glow where the
    // background is transparent
    let lum = clamp(max(max(glow.r, glow.g), glow.b), 0.0, 1.0);
    let a = max(bg.a, lum);
    let premul = bg.rgb * bg.a + glow;
    return vec4<f32>(clamp(premul / max(a, 1.0e-4), vec3<f32>(0.0), vec3<f32>(1.0)), a);
}

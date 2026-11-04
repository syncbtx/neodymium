use amane::{Canvas, Cap, Circle, Color, Path, Rectangle, Shape, shapes};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProgressMode {
    Ring,
    WaveyRing,
    Bar,
    WaveyBar,
}

impl From<model::bar::module::SystemModuleStyle> for ProgressMode {
    fn from(style: model::bar::module::SystemModuleStyle) -> Self {
        match style {
            model::bar::module::SystemModuleStyle::Ring => ProgressMode::Ring,
            model::bar::module::SystemModuleStyle::WaveyRing => ProgressMode::WaveyRing,
            model::bar::module::SystemModuleStyle::Bar => ProgressMode::Bar,
            model::bar::module::SystemModuleStyle::WaveyBar => ProgressMode::WaveyBar,
            model::bar::module::SystemModuleStyle::Graph => ProgressMode::WaveyBar,
        }
    }
}

/// A wavy arc as raw cubics, in canvas coordinates.
pub struct Curve {
    pub start: (f32, f32),
    /// `[x1, y1, x2, y2, x, y]` per segment, same order as `cubic_to`.
    pub cubics: Vec<[f32; 6]>,
}

/// Centre line of a wavy arc.
///
/// - `radius`: radius of the mean circle
/// - `amplitude`: how far the line swings in and out of it
/// - `waves`: full waves per turn (constant wavelength; whole numbers close
///   up neatly at 100%)
/// - `phase`: radians; animate it to make the ring flow
/// - `start`, `sweep`: degrees; a negative sweep goes counterclockwise
pub fn wavy_curve(
    cx: f32,
    cy: f32,
    radius: f32,
    amplitude: f32,
    waves: f32,
    phase: f32,
    start: f32,
    sweep: f32,
) -> Curve {
    let a0 = start.to_radians();
    let span = sweep.to_radians();

    // radius and its slope at angle `a` (radians), wave phase measured from `a0`
    let rho = |a: f32| radius + amplitude * (waves * (a - a0) + phase).sin();
    let drho = |a: f32| amplitude * waves * (waves * (a - a0) + phase).cos();

    // point and velocity (per radian of angle) at `a`, y grows downward
    let at = |a: f32| {
        let (s, c) = a.sin_cos();
        let r = rho(a);
        let dr = drho(a);
        let p = (cx + r * s, cy - r * c);
        let v = (dr * s + r * c, -dr * c + r * s);
        (p, v)
    };

    // 8 segments per wave keeps the Hermite fit well under a pixel
    let per_turn = (waves.abs() * 8.0).max(16.0);
    let steps = ((span.abs() / std::f32::consts::TAU * per_turn).ceil() as usize).max(4);
    let step = span / steps as f32;

    let (first, _) = at(a0);
    let mut cubics = Vec::with_capacity(steps);

    for i in 0..steps {
        let from = a0 + step * i as f32;
        let to = from + step;
        let (p0, v0) = at(from);
        let (p1, v1) = at(to);
        let k = step / 3.0;

        cubics.push([
            p0.0 + v0.0 * k,
            p0.1 + v0.1 * k,
            p1.0 - v1.0 * k,
            p1.1 - v1.1 * k,
            p1.0,
            p1.1,
        ]);
    }

    Curve {
        start: first,
        cubics,
    }
}

/// Centre line of a horizontal wavy line from `x0` to `x0 + length`.
///
/// The wave is tied to x, so growing the line never squeezes the waves.
/// `wavelength` is in pixels, `phase` in radians (animate it to make it flow).
pub fn wavy_line_curve(
    x0: f32,
    cy: f32,
    length: f32,
    amplitude: f32,
    wavelength: f32,
    phase: f32,
) -> Curve {
    let k = std::f32::consts::TAU / wavelength;

    let at = |x: f32| {
        let arg = k * (x - x0) + phase;
        ((x, cy + amplitude * arg.sin()), amplitude * k * arg.cos())
    };

    // 8 segments per wavelength
    let steps = ((length / wavelength * 8.0).ceil() as usize).max(2);
    let step = length / steps as f32;

    let (first, _) = at(x0);
    let mut cubics = Vec::with_capacity(steps);

    for i in 0..steps {
        let from = x0 + step * i as f32;
        let to = from + step;
        let (p0, d0) = at(from);
        let (p1, d1) = at(to);
        let h = step / 3.0;

        cubics.push([p0.0 + h, p0.1 + d0 * h, p1.0 - h, p1.1 - d1 * h, p1.0, p1.1]);
    }

    Curve {
        start: first,
        cubics,
    }
}

// ---- amane glue (everything above is plain math) ----

/// The wavy arc as an amane `Path`. Add `.stroke(..)` and `.cap(Cap::Round)`.
pub fn wavy_arc(
    cx: f32,
    cy: f32,
    radius: f32,
    amplitude: f32,
    waves: f32,
    phase: f32,
    start: f32,
    sweep: f32,
) -> Path {
    let curve = wavy_curve(cx, cy, radius, amplitude, waves, phase, start, sweep);

    let mut path = Path::new().move_to(curve.start.0, curve.start.1);

    for c in curve.cubics {
        path = path.cubic_to(c[0], c[1], c[2], c[3], c[4], c[5]);
    }

    path
}

#[derive(Clone, Copy)]
pub struct Style {
    pub size: f32,
    pub thickness: f32,
    pub amplitude: f32,
    pub freq: f32,
    pub color: Color,
    pub track: Color,
    pub warning: Color,
    pub error: Color,
    pub thresholds: [f32; 2],
    pub mode: ProgressMode,
}

/// A wavy progress ring, starting at 12 o'clock. `progress` is 0.0..=1.0,
/// `phase` is the wave offset in radians.
pub fn ring_(style: Style, progress: f32, phase: f32) -> Canvas {
    let size = style.size;
    let middle = size / 2.0;

    // keep the whole stroke inside the canvas
    let radius = if style.amplitude == 0.0 {
        middle - style.thickness / 2.0
    } else {
        middle - style.amplitude + style.thickness / 2.0
    };

    let sweep = 360.0 * progress.clamp(0.0, 1.0);

    let mut layers = shapes![Circle::new().fill(style.track)];

    if sweep > 0.5 {
        layers.push(Box::new(
            wavy_arc(
                middle,
                middle,
                radius,
                style.amplitude,
                style.freq,
                phase,
                0.0,
                sweep,
            )
            .stroke(style.thickness, style.color)
            .cap(Cap::Round),
        ));
    }

    Canvas::new().width(size).height(size).shapes(layers)
}

pub fn color_to_vec4(c: Color) -> [f32; 4] {
    let r = c.red() as f32 / 255.0;
    let g = c.green() as f32 / 255.0;
    let b = c.blue() as f32 / 255.0;
    let a = c.alpha() as f32 / 255.0;
    [r, g, b, a]
}

pub fn progress(style: &Style, current: f32, phase: f32) -> Rectangle {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/progress.wgsl");

    let amplitude = (current * 1.35).max(1.0);

    let freq = 8.0 * 1.0 / current.max(0.75);

    let mode = match style.mode {
        ProgressMode::Ring | ProgressMode::WaveyRing => 0.0,
        ProgressMode::Bar | ProgressMode::WaveyBar => 1.0,
    };

    let warning = style.thresholds[0];
    let error = style.thresholds[1];

    Rectangle::new()
        .width(style.size)
        .height(style.size)
        .shader(path)
        .shader_values(vec![
            color_to_vec4(style.color),                // [0] normal color
            color_to_vec4(style.track),                // [1] track color
            [style.thickness, amplitude, freq, phase], // [2] thickness, amplitude, freq, phase
            [current, mode, warning, error],           // [3] progress, mode, warn, error
            color_to_vec4(style.warning),              // [4] warning color
            color_to_vec4(style.error),                // [5] error color
        ])
}

pub fn flat_bar(
    size: f32,
    thickness: f32,
    current: f32,
    phase: f32,
    colors: &[Color; 4],
    thresholds: [f32; 2],
) -> Rectangle {
    let style = &Style {
        size,
        thickness,
        amplitude: 0.0,
        freq: 0.0,
        color: colors[0],
        track: colors[1],
        warning: colors[2],
        error: colors[3],
        thresholds,
        mode: ProgressMode::Bar,
    };
    progress(style, current, phase)
}

pub fn flat_ring(
    size: f32,
    thickness: f32,
    current: f32,
    phase: f32,
    colors: &[Color; 4],
    thresholds: [f32; 2],
) -> Rectangle {
    let style = &Style {
        size,
        thickness,
        amplitude: 0.0,
        freq: 0.0,
        color: colors[0],
        track: colors[1],
        warning: colors[2],
        error: colors[3],
        thresholds,
        mode: ProgressMode::Ring,
    };
    progress(style, current, phase)
}

pub fn wavy_bar(
    size: f32,
    thickness: f32,
    current: f32,
    phase: f32,
    colors: &[Color; 4],
    thresholds: [f32; 2],
) -> Rectangle {
    let amplitude = (current * 1.35).max(1.0);

    let freq = 8.0 * 1.0 / current.max(0.75);

    let style = &Style {
        size,
        thickness,
        amplitude,
        freq,
        color: colors[0],
        track: colors[1],
        warning: colors[2],
        error: colors[3],
        thresholds,
        mode: ProgressMode::WaveyBar,
    };
    progress(style, current, phase)
}

pub fn wavy_ring(
    size: f32,
    thickness: f32,
    current: f32,
    phase: f32,
    colors: &[Color; 4],
    thresholds: [f32; 2],
) -> Rectangle {
    let amplitude = (current * 1.35).max(1.0);

    let freq = 8.0 * 1.0 / current.max(0.75);

    let style = &Style {
        size,
        thickness,
        amplitude,
        freq,
        color: colors[0],
        track: colors[1],
        warning: colors[2],
        error: colors[3],
        thresholds,
        mode: ProgressMode::WaveyRing,
    };
    progress(style, current, phase)
}

use amane::{Canvas, Cap, Circle, Color, Line, Path, Rectangle, Shape, Widget, shapes};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProgressStyle {
    Ring,
    WaveyRing,
    Bar,
    WaveyBar,
}

impl From<model::bar::module::SystemModuleStyle> for ProgressStyle {
    fn from(style: model::bar::module::SystemModuleStyle) -> Self {
        match style {
            model::bar::module::SystemModuleStyle::Ring => ProgressStyle::Ring,
            model::bar::module::SystemModuleStyle::WaveyRing => ProgressStyle::WaveyRing,
            model::bar::module::SystemModuleStyle::Bar => ProgressStyle::Bar,
            model::bar::module::SystemModuleStyle::WaveyBar => ProgressStyle::WaveyBar,
            model::bar::module::SystemModuleStyle::Graph => ProgressStyle::WaveyBar,
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
    pub waves: f32,
    pub track: Color,
    pub color: Color,
}

/// A wavy progress ring, starting at 12 o'clock. `progress` is 0.0..=1.0,
/// `phase` is the wave offset in radians.
pub fn ring(style: Style, progress: f32, phase: f32) -> Canvas {
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
                style.waves,
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

fn to_path(curve: Curve) -> Path {
    let mut path = Path::new().move_to(curve.start.0, curve.start.1);

    for c in curve.cubics {
        path = path.cubic_to(c[0], c[1], c[2], c[3], c[4], c[5]);
    }

    path
}

/// Look of a linear bar.
#[derive(Clone, Copy)]
pub struct BarStyle {
    pub width: f32,
    pub thickness: f32,
    pub amplitude: f32,
    /// Pixels per full wave.
    pub wavelength: f32,
    pub track: Color,
    pub color: Color,
}

/// A wavy linear progress bar: flat track, wavy active part with round caps.
/// `progress` is 0.0..=1.0. The canvas is `amplitude * 2 + thickness` tall.
pub fn bar(style: BarStyle, progress: f32, phase: f32) -> Canvas {
    let height = style.amplitude * 2.0 + style.thickness;
    let middle = height / 2.0;

    // the round caps stick out half a thickness, so inset by that
    let inset = style.thickness / 2.0;
    let start = inset;
    let end = style.width - inset;

    let length = (end - start) * progress.clamp(0.0, 1.0);

    let mut layers = shapes![
        Line::new()
            .from(start, middle)
            .to(end, middle)
            .stroke(style.thickness, style.track)
            .cap(Cap::Round)
    ];

    if length > 0.5 {
        let curve = wavy_line_curve(
            start,
            middle,
            length,
            style.amplitude,
            style.wavelength,
            phase,
        );

        layers.push(Box::new(
            to_path(curve)
                .stroke(style.thickness, style.color)
                .cap(Cap::Round),
        ));
    }

    Canvas::new()
        .width(style.width)
        .height(height)
        .shapes(layers)
}

pub fn view(
    _is_horizontal: bool,
    ratio: f32, // expected 0.0..=1.0
    style: ProgressStyle,
    length: f32,
    thickness: f32,
    phase: f32,
    track_color: Color,
    fill_color: Color,
) -> Box<dyn Widget> {
    let ratio = ratio.clamp(0.0, 1.0);

    let widget: Box<dyn Widget> = match style {
        ProgressStyle::Bar => {
            let bar_style = BarStyle {
                width: length,
                thickness,
                amplitude: 0.0,
                wavelength: 1.0,
                track: track_color,
                color: fill_color,
            };
            Box::new(bar(bar_style, ratio, phase))
        }
        ProgressStyle::WaveyBar => {
            let bar_style = BarStyle {
                width: length,
                thickness,
                amplitude: 1.5,
                wavelength: 14.0,
                track: track_color,
                color: fill_color,
            };
            Box::new(bar(bar_style, ratio, phase))
        }
        ProgressStyle::Ring => {
            let ring_style = Style {
                size: length,
                thickness,
                amplitude: 0.0,
                waves: 0.0,
                track: track_color,
                color: fill_color,
            };
            Box::new(ring(ring_style, ratio, phase))
        }
        ProgressStyle::WaveyRing => {
            let ring_style = Style {
                size: length,
                thickness,
                amplitude: 1.0,
                waves: 8.0,
                track: track_color,
                color: fill_color,
            };
            Box::new(ring(ring_style, ratio, phase))
        }
    };

    widget
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::bar::module::SystemModuleStyle;

    #[test]
    fn test_progress_style_from_system_module_style() {
        assert_eq!(
            ProgressStyle::from(SystemModuleStyle::Ring),
            ProgressStyle::Ring
        );
        assert_eq!(
            ProgressStyle::from(SystemModuleStyle::WaveyRing),
            ProgressStyle::WaveyRing
        );
        assert_eq!(
            ProgressStyle::from(SystemModuleStyle::Bar),
            ProgressStyle::Bar
        );
        assert_eq!(
            ProgressStyle::from(SystemModuleStyle::WaveyBar),
            ProgressStyle::WaveyBar
        );
        assert_eq!(
            ProgressStyle::from(SystemModuleStyle::Graph),
            ProgressStyle::WaveyBar
        );
    }

    #[test]
    fn test_progress_view_construction() {
        let _w_bar = view(
            true,
            0.5,
            ProgressStyle::Bar,
            64.0,
            2.0,
            0.0,
            Color::BLACK,
            Color::WHITE,
        );
        let _w_ring = view(
            true,
            0.75,
            ProgressStyle::WaveyRing,
            32.0,
            2.0,
            1.5,
            Color::BLACK,
            Color::WHITE,
        );
    }
}

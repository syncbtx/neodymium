use amane::{Color, Rectangle};

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
            model::bar::module::SystemModuleStyle::Text => ProgressMode::Bar,
        }
    }
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

pub fn color_to_vec4(c: Color) -> [f32; 4] {
    let r = c.red() as f32 / 255.0;
    let g = c.green() as f32 / 255.0;
    let b = c.blue() as f32 / 255.0;
    let a = c.alpha() as f32 / 255.0;
    [r, g, b, a]
}

pub fn progress(style: &Style, current: f32, phase: f32) -> Rectangle {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/progress.wgsl");

    let amplitude = if style.amplitude == 0.0 {
        0.0
    } else {
        (current * 0.9).clamp(0.8, 0.9)
    };

    let freq = if style.freq == 0.0 {
        0.0
    } else {
        10.0 - 4.0 * current
    };

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
    // let amplitude = (current * 1.35).max(0.8);

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

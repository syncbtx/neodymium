use amane::{Rectangle, Size, Stack, Widget};
use hex_color::HexColor;
use model::{Backdrop, Palette};

use crate::{Axis, color::resolve_color_with_alpha, component::progress::color_to_vec4};

/// `cross` is the bar's thickness. The wave's length along the bar is taken from
/// its parent (Size::Parent), so it always fits the real content box.
pub fn view(axis: Axis, backdrop: Backdrop, cross: f32, palette: &Palette, phase: f32) -> Stack {
    match backdrop {
        Backdrop::None => Stack::new(vec![]),
        Backdrop::Blur { .. } => Stack::new(vec![]),
        Backdrop::Waveform {
            opacity,
            drift_speed,
        } => {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/backdrop_wave.wgsl");
            let primary = resolve_color_with_alpha(palette.primary, opacity);
            let transparent = resolve_color_with_alpha(HexColor::rgb(0, 0, 0), 0);

            let (width, height) = axis.size(Size::Parent, Size::Fixed(cross));

            let vertical = if axis.is_vertical() { 1.0 } else { 0.0 };

            let wave01 = Rectangle::new()
                .width(width)
                .height(height)
                .shader(path)
                .shader_values(vec![
                    color_to_vec4(primary),               // fill color
                    color_to_vec4(transparent),           // track color
                    [1.25, cross / 4.0, 128.0, 256.0],    // thickness, max amp, min/max wavelength
                    [42.0, drift_speed, phase, vertical], // seed, drift speed, phase, vertical
                ]);

            Stack::new(vec![Box::new(wave01) as Box<dyn Widget>])
        }
        Backdrop::Shader { .. } => Stack::new(vec![]),
    }
}

use amane::{Rectangle, Stack, Widget};
use hex_color::HexColor;
use model::{Backdrop, Palette};

use crate::{color::resolve_color_with_alpha, component::progress::color_to_vec4};

pub fn view(backdrop: Backdrop, main: f32, cross: f32, palette: &Palette, phase: f32) -> Stack {
    match backdrop {
        Backdrop::None => Stack::new(vec![]),
        Backdrop::Blur { .. } => Stack::new(vec![]),
        Backdrop::Waveform { opacity } => {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/backdrop_wave.wgsl");
            let primary = resolve_color_with_alpha(palette.primary, opacity);
            let transparent = resolve_color_with_alpha(HexColor::rgb(0, 0, 0), 0);

            let wave01 = Rectangle::new()
                .width(main)
                .height(cross)
                .shader(path)
                .shader_values(vec![
                    color_to_vec4(primary),
                    color_to_vec4(transparent),
                    [1.5, cross / 4.0, 128.0, 256.0], // thickness, max amp, min/max wavelength
                    [42.0, 0.5, phase, 0.0],          // seed, drift speed, phase
                ]);

            Stack::new(vec![Box::new(wave01) as Box<dyn Widget>])
        }
        Backdrop::Shader { .. } => Stack::new(vec![]),
    }
}

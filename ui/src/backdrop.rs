use amane::{Rectangle, Stack, Widget};
use hex_color::HexColor;
use model::{Backdrop, palette::Palette};

use crate::{
    color::{resolve_color, resolve_color_with_alpha},
    component::progress::color_to_vec4,
};

pub fn view(backdrop: Backdrop, main: f32, cross: f32, palette: &Palette, phase: f32) -> Stack {
    match backdrop {
        Backdrop::None => {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/backdrop_wave.wgsl");

            // let path02 = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/backdrop_rects.wgsl");

            let primary = resolve_color(palette.primary);
            // let secondary = resolve_color(palette.secondary);
            // let tertiary = resolve_color(palette.tertiary);
            let transparent = resolve_color_with_alpha(HexColor::rgb(0, 0, 0), 0);

            let wave01 = Rectangle::new()
                .width(main)
                .height(cross)
                .shader(path)
                .shader_values(vec![
                    color_to_vec4(primary),
                    color_to_vec4(transparent),
                    [2.0, cross / 2.0, 48.0, 256.0], // thickness, max amp, min/max wavelength
                    [42.0, 0.5, phase, 0.0],         // seed, drift speed, phase
                ]);

            // let wave02 = Rectangle::new()
            //     .width(main)
            //     .height(cross)
            //     .shader(path)
            //     .shader_values(vec![
            //         color_to_vec4(secondary),
            //         color_to_vec4(transparent),
            //         [2.0, cross / 2.0, 64.0, 256.0], // thickness, max amp, min/max wavelength
            //         [64.0, 0.45, phase * 1.5, 0.0],  // seed, drift speed, phase
            //     ]);

            // let wave03 = Rectangle::new()
            //     .width(main)
            //     .height(cross)
            //     .shader(path)
            //     .shader_values(vec![
            //         color_to_vec4(tertiary),
            //         color_to_vec4(transparent),
            //         [2.0, cross / 2.0, 72.0, 256.0], // thickness, max amp, min/max wavelength
            //         [128.0, 0.45, -phase * 0.75, 0.0], // seed, drift speed, phase
            //     ]);

            Stack::new(vec![
                Box::new(wave01) as Box<dyn Widget>,
                // Box::new(wave02) as Box<dyn Widget>,
                // Box::new(wave03) as Box<dyn Widget>,
            ])
        }
        Backdrop::Blur { .. } => todo!(),
        Backdrop::Waveform { .. } => todo!(),
        Backdrop::Shader { .. } => todo!(),
    }
}

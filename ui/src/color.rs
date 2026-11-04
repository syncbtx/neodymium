use amane::Color;
use hex_color::HexColor;

pub fn resolve_color(c: HexColor) -> Color {
    Color::rgb(c.r, c.g, c.b)
}

pub fn resolve_color_with_alpha(c: HexColor, a: u8) -> Color {
    let eff_a = ((c.a as u16 * a as u16) / 255) as u8;
    Color::rgba(c.r, c.g, c.b, eff_a)
}

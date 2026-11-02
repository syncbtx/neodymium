use amane::Color;
use hex_color::HexColor;

pub fn resolve_color(c: HexColor) -> Color {
    Color::rgb(c.r, c.g, c.b)
}

pub fn resolve_color_with_alpha(c: HexColor, a: u8) -> Color {
    Color::rgba(c.r, c.g, c.b, a)
}

use amane::Color;
use hex_color::HexColor;

pub fn resolve_color(c: HexColor) -> Color {
    Color::rgb(c.r, c.g, c.b)
}

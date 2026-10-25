use amane::Color;
use hex_color::HexColor;

pub fn hex_to_color(c: HexColor) -> Color {
    Color::rgb(c.r, c.g, c.b)
}

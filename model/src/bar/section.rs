use hex_color::HexColor;

use crate::bar::module::BarModule;

pub struct BarSection {
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>,
    pub opacity: u32,
    pub spacing: u32,
    pub major_axis_padding: u32,
    pub minor_axis_padding: u32,
    pub border: Border,
    pub border_radius: Option<u32>,
    pub modules: Vec<BarModule>,
}

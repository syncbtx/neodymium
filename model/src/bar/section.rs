use hex_color::HexColor;

use crate::{Border, Insets, bar::module::Module};

#[derive(Debug)]
pub struct BarSection {
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>,
    pub contrast: bool,
    pub opacity: u32,
    pub spacing: u32,
    pub padding: Insets<u32>,
    pub border: Border,
    pub border_radius: Option<u32>,
    pub modules: Vec<Module>,
}

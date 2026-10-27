use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Border, Insets, bar::module::Module};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarSectionColors {
    pub background: HexColor,
    pub foreground: HexColor,
    pub module_background: HexColor,
    pub module_foreground: HexColor,
}

impl Default for BarSectionColors {
    fn default() -> Self {
        Self {
            background: HexColor::rgba(255, 131, 131, 1),
            foreground: HexColor::rgba(25, 36, 31, 1),
            module_background: HexColor::rgba(25, 36, 31, 1),
            module_foreground: HexColor::rgba(192, 193, 194, 1),
        }
    }
}

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
    pub colors: BarSectionColors,
    pub modules: Vec<Module>,
}

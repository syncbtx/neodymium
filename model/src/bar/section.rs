use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Alpha, Border, Font, Insets, bar::module::Module};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum SectionTint {
    #[default]
    Surface,
    Primary,
    Secondary,
    Tertiary,
    Error,
}

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
    pub tint: SectionTint,
    pub opacity: Alpha,
    pub spacing: u32,
    pub padding: Option<Insets<u32>>,
    pub border: Border,
    pub border_radius: Option<u32>,
    pub font: Option<Font>,
    pub modules: Vec<Module>,
}

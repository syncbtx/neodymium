use configuration::{domain::bars::SectionConfig, shared::Border};
use hex_color::HexColor;

use crate::bar::module::BarModule;

#[derive(Debug)]
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

impl BarSection {
    pub fn from_config(config: &SectionConfig) -> Self {
        let mut modules = Vec::new();
        for cfg in config.modules.iter() {
            modules.push(BarModule::from_config(&cfg));
        }
        Self {
            background: config.background,
            foreground: config.foreground,
            opacity: config.opacity,
            spacing: config.spacing,
            major_axis_padding: config.major_axis_padding,
            minor_axis_padding: config.minor_axis_padding,
            border: config.border,
            border_radius: config.border_radius,
            modules,
        }
    }
}

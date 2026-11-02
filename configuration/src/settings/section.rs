use hex_color::HexColor;
use model::{
    Border, Font, Insets,
    bar::{
        module::Module,
        section::{BarSection, BarSectionColors, SectionTint},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::resolve::Resolve;

#[derive(Debug, Serialize, Clone, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SectionSettings {
    pub tint: SectionTint,
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>, // hex | "contrast"
    pub contrast: bool,               // hex | "contrast"
    pub opacity: u32,                 // must be explicitly overrided, defaults to 100
    pub spacing: u32,
    pub padding: Insets<u32>,
    pub border: Border,             // defaults to none
    pub border_radius: Option<u32>, // only freaks touch this
    pub colors: BarSectionColors,
    pub font: Option<Font>,
    pub modules: Vec<Module>,
}

impl Resolve for SectionSettings {
    type Output = BarSection;

    fn resolve(&self) -> Self::Output {
        BarSection {
            tint: self.tint,
            background: self.background,
            foreground: self.foreground,
            contrast: self.contrast,
            opacity: self.opacity,
            spacing: self.spacing,
            padding: self.padding,
            border: self.border,
            border_radius: self.border_radius,
            colors: self.colors,
            font: self.font.clone(),
            modules: self.modules.clone(),
        }
    }
}

impl SectionSettings {
    pub fn sanitize(&mut self, bar_cross_axis: u32) {
        if self.opacity == 0 {
            self.opacity = 255;
        } else {
            self.opacity = self.opacity.min(255);
        }

        let max_radius = bar_cross_axis / 2;
        if let Some(r) = self.border_radius {
            self.border_radius = Some(r.min(max_radius));
        }

        self.spacing = self.spacing.min(64);

        self.padding.top = self.padding.top.min(bar_cross_axis / 4);
        self.padding.bottom = self.padding.bottom.min(bar_cross_axis / 4);
        self.padding.left = self.padding.left.min(bar_cross_axis / 4);
        self.padding.right = self.padding.right.min(bar_cross_axis / 4);
    }
}

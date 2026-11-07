use model::{
    Alpha, Border, Font, Insets,
    bar::{
        module::Module,
        section::{BarSection, SectionTint},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SectionSettings {
    pub tint: SectionTint,
    pub opacity: Alpha,
    pub spacing: u32,
    pub padding: Option<Insets<u32>>,
    pub border: Border,
    pub border_radius: Option<u32>,
    pub module_border_radius: Option<u32>,
    pub font: Option<Font>,
    pub modules: Vec<Module>,
}

impl SectionSettings {
    pub fn resolve_with_fallback(&self, fallback_padding: Insets<u32>) -> BarSection {
        BarSection {
            tint: self.tint,
            opacity: self.opacity,
            spacing: self.spacing,
            padding: self.padding.unwrap_or(fallback_padding),
            border: self.border,
            border_radius: self.border_radius,
            module_border_radius: self.module_border_radius,
            font: self.font.clone(),
            modules: self.modules.clone(),
        }
    }
}

impl SectionSettings {
    pub fn sanitize(&mut self, bar_cross_axis: u32) {
        let max_radius = bar_cross_axis / 2;
        if let Some(r) = self.border_radius {
            self.border_radius = Some(r.min(max_radius));
        }

        self.spacing = self.spacing.min(64);
        match self.padding {
            Some(padding) => {
                self.padding = Some(Insets {
                    top: padding.top.min(bar_cross_axis / 4),
                    bottom: padding.bottom.min(bar_cross_axis / 4),
                    left: padding.left.min(bar_cross_axis / 4),
                    right: padding.right.min(bar_cross_axis / 4),
                });
            }
            None => {}
        }
    }
}

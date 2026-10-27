use hex_color::HexColor;
use model::{
    Alpha, AttachmentMode, Backdrop, BarBlend, Border, Edge, Font, Insets, Justify, Layer,
    MainAxisSize, ScreenCornerClaimer,
    bar::{
        Bar, Bars,
        module::Module,
        section::{BarSection, BarSectionColors},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{resolve::Resolve, validate::Sanitize};

#[derive(Debug, Serialize, Clone, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SectionSettings {
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>, // hex | "contrast"
    pub contrast: bool,               // hex | "contrast"
    pub opacity: u32,                 // must be explicitly overrided, defaults to 100
    pub spacing: u32,
    pub padding: Insets<u32>,
    pub border: Border,             // defaults to none
    pub border_radius: Option<u32>, // only freaks touch this
    pub colors: BarSectionColors,
    pub modules: Vec<Module>,
}

impl Resolve for SectionSettings {
    type Output = BarSection;

    fn resolve(&self) -> Self::Output {
        BarSection {
            background: self.background,
            foreground: self.foreground,
            contrast: self.contrast,
            opacity: self.opacity,
            spacing: self.spacing,
            padding: self.padding,
            border: self.border,
            border_radius: self.border_radius,
            colors: self.colors,
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

#[derive(Debug, Serialize, Clone, Default, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarSettings {
    pub enabled: bool,
    pub edge: Edge,
    pub layer: Layer,
    pub attachment_mode: AttachmentMode,
    pub islands: bool,
    pub autohide: bool,
    pub border: Border,
    pub border_radius: u32,
    pub section_border_radius: u32, // global fallback for sections
    pub backdrop: Backdrop,
    pub background: HexColor, // applied when backdrop is None and also serves as fallback for sections
    pub foreground: HexColor,
    pub contrast: bool,
    pub opacity: Alpha,
    pub main_axis_size: MainAxisSize, // length
    pub section_cross_axis_size: u32, // section thickness
    pub margin: Insets<u32>,
    pub padding: Insets<u32>,
    pub section_padding: Insets<u32>,
    pub sections: Vec<SectionSettings>,
    pub sections_justify: Justify,
    pub font: Option<Font>,
}

impl Resolve for BarSettings {
    type Output = Bar;

    fn resolve(&self) -> Self::Output {
        Bar {
            enabled: self.enabled,
            edge: self.edge,
            layer: self.layer,
            attachment_mode: self.attachment_mode,
            exclusive_zone: true,
            islands: self.islands,
            autohide: self.autohide,
            border: self.border,
            border_radius: self.border_radius,
            section_border_radius: self.section_border_radius,
            backdrop: self.backdrop.clone(),
            background: self.background,
            foreground: self.foreground,
            contrast: self.contrast,
            opacity: self.opacity,
            main_axis_size: self.main_axis_size,
            section_cross_axis_size: self.section_cross_axis_size,
            margin: self.margin,
            padding: self.padding,
            section_padding: self.section_padding,
            sections: self.sections.iter().map(|m| m.resolve()).collect(),
            sections_justify: self.sections_justify,
            font: self.font.clone(),
        }
    }
}

impl Sanitize for BarSettings {
    fn sanitize(&mut self) {
        if self.section_cross_axis_size < 16 {
            self.section_cross_axis_size = 24;
        }

        match &mut self.main_axis_size {
            MainAxisSize::Percent(pct) => *pct = (*pct).clamp(1, 100),
            MainAxisSize::Pixels(px) => *px = (*px).max(16),
            MainAxisSize::Auto => {}
        }

        let half_thickness = self.section_cross_axis_size / 2;
        match self.edge {
            Edge::Top | Edge::Bottom => {
                if self.padding.top + self.padding.bottom >= half_thickness {
                    self.padding.top = 4;
                    self.padding.bottom = 4;
                }
            }
            Edge::Left | Edge::Right => {
                if self.padding.left + self.padding.right >= half_thickness {
                    self.padding.left = 4;
                    self.padding.right = 4;
                }
            }
        }

        self.border_radius = self.border_radius.min(self.section_cross_axis_size / 2);
        self.section_border_radius = self
            .section_border_radius
            .min(self.section_cross_axis_size / 2);

        self.sections.retain(|sec| !sec.modules.is_empty());
        for sec in &mut self.sections {
            sec.sanitize(self.section_cross_axis_size);
        }

        if self.sections.is_empty() {
            self.enabled = false;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarsSettings {
    pub top: BarSettings,
    pub right: BarSettings,
    pub bottom: BarSettings,
    pub left: BarSettings,
    pub screen_corner_claimer: ScreenCornerClaimer,
    pub blend: BarBlend,
}

impl<'a> IntoIterator for &'a BarsSettings {
    type Item = &'a BarSettings;
    type IntoIter = std::array::IntoIter<&'a BarSettings, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.top, &self.right, &self.bottom, &self.left].into_iter()
    }
}

impl<'a> IntoIterator for &'a mut BarsSettings {
    type Item = &'a mut BarSettings;
    type IntoIter = std::array::IntoIter<&'a mut BarSettings, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &mut self.top,
            &mut self.right,
            &mut self.bottom,
            &mut self.left,
        ]
        .into_iter()
    }
}

impl BarsSettings {
    pub fn iter(&self) -> std::array::IntoIter<&BarSettings, 4> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut BarSettings, 4> {
        self.into_iter()
    }
}

impl Resolve for BarsSettings {
    type Output = Bars;

    fn resolve(&self) -> Self::Output {
        Bars {
            top: self.top.resolve(),
            right: self.right.resolve(),
            bottom: self.bottom.resolve(),
            left: self.left.resolve(),
            screen_corner_claimer: self.screen_corner_claimer,
            blend: self.blend,
        }
    }
}

impl Sanitize for BarsSettings {
    fn sanitize(&mut self) {
        self.top.edge = Edge::Top;
        self.right.edge = Edge::Right;
        self.bottom.edge = Edge::Bottom;
        self.left.edge = Edge::Left;
    }
}

use hex_color::HexColor;
use model::{
    Alpha, AttachmentMode, Backdrop, BarBlend, Border, Edge, Font, Insets, Layer, MainAxisSize,
    ScreenCornerClaimer,
    bar::{Bar, Bars, module::Module, section::BarSection},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{resolve::Resolve, settings::ConfigDomain, validate::Sanitize};

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
            modules: self.modules.clone(),
        }
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
    pub cross_axis_size: u32,         // thickness
    pub margin: Insets<u32>,
    pub padding: Insets<u32>,
    pub section_padding: Insets<u32>,
    pub sections: Vec<SectionSettings>,
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
            cross_axis_size: self.cross_axis_size,
            margin: self.margin,
            padding: self.padding,
            section_padding: self.section_padding,
            sections: self.sections.iter().map(|m| m.resolve()).collect(),
            font: self.font.clone(),
        }
    }
}

impl Sanitize for BarSettings {
    fn sanitize(&mut self) {}
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

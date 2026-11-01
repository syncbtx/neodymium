use std::str::FromStr;

use hex_color::HexColor;
use model::{
    Alpha, Anchor, AttachmentMode, Backdrop, Border, Edge, Font, Insets, Justify, Layer,
    MainAxisSize,
    bar::{Bar, module::Module, section::BarSectionColors},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{resolve::Resolve, settings::section::SectionSettings, validate::Sanitize};

#[derive(Debug, Serialize, Clone, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarSettings {
    pub enabled: bool,
    pub edge: Edge,
    pub anchor: Anchor,
    pub layer: Layer,
    pub attachment_mode: AttachmentMode,
    pub islands: bool,
    pub spacing: u32,
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
    pub padding: Insets<u32>,
    pub section_padding: Insets<u32>,
    pub sections: Vec<SectionSettings>,
    pub sections_justify: Justify,
    pub font: Option<Font>,
}

impl Default for BarSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            edge: Edge::default(),
            anchor: Anchor::default(),
            layer: Layer::default(),
            attachment_mode: AttachmentMode::Floating { margin: 8 },
            islands: false,
            autohide: false,
            spacing: 8,
            border: Border::default(),
            border_radius: 8,
            section_border_radius: 8,
            backdrop: Backdrop::default(),
            background: HexColor::from_str("#ecdecdff").unwrap(),
            foreground: HexColor::from_str("#1cec1cff").unwrap(),
            contrast: true,
            opacity: Alpha::default(),
            main_axis_size: MainAxisSize::Percent(80),
            section_cross_axis_size: 24,
            padding: Insets::all(8),
            section_padding: Insets::all(8),
            sections: vec![
                SectionSettings {
                    background: None,
                    foreground: None,
                    contrast: true,
                    opacity: 255,
                    spacing: 8,
                    padding: Insets::all(8),
                    border: Border::None,
                    border_radius: Some(8),
                    colors: BarSectionColors::default(),
                    font: None,
                    modules: vec![Module::Workspaces {
                        collapse_unused: true,
                    }],
                },
                SectionSettings {
                    background: None,
                    foreground: None,
                    contrast: true,
                    opacity: 255,
                    spacing: 8,
                    padding: Insets::all(8),
                    border: Border::None,
                    border_radius: Some(8),
                    colors: BarSectionColors::default(),
                    font: None,
                    modules: vec![Module::Calendar {
                        time: true,
                        format_24_hrs: true,
                        date: true,
                        compact: false,
                    }],
                },
                SectionSettings {
                    background: None,
                    foreground: None,
                    contrast: true,
                    opacity: 255,
                    spacing: 8,
                    padding: Insets::all(8),
                    border: Border::None,
                    border_radius: Some(8),
                    colors: BarSectionColors::default(),
                    font: None,
                    modules: vec![Module::FocusedWindowIcon],
                },
            ],
            sections_justify: Justify::default(),
            font: None,
        }
    }
}

impl Resolve for BarSettings {
    type Output = Bar;

    fn resolve(&self) -> Self::Output {
        Bar {
            enabled: self.enabled,
            edge: self.edge,
            anchor: self.anchor,
            layer: self.layer,
            attachment_mode: self.attachment_mode,
            exclusive_zone: true,
            islands: self.islands,
            autohide: self.autohide,
            spacing: self.spacing,
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
            margin: Insets::default(),
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

use hex_color::HexColor;
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{borrow::Cow, fmt::Display, num::NonZeroU32, str::FromStr};

use crate::{
    settings::ConfigDomain,
    shared::{Alpha, Border, Edge},
    validate::{ConfigValidate, ValidationError},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BarLayer {
    #[default]
    Top,
    Bottom,
    Overlay,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum AttachmentMode {
    #[default]
    Floating,
    Attached,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum MajorAxisSize {
    #[default]
    Auto,
    Percent(u32),
    Pixels(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BarBackdrop {
    #[default]
    None,
    Blur {
        radius: u32,
    },
    Waveform {
        color: HexColor,
        opacity: u32,
    },
    // Gradient,
    // Glow
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ScreenCornerClaimer {
    #[default]
    HorizontalBars,
    VerticalBars,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ModuleConfig {
    #[default]
    WindowTitle,
    Workspaces {
        collapse_unused: bool,
    },
    Spacer(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarBlend {
    pub enabled: bool, // disable borders and apply border radius
    pub radius: u32,   // no exclusive zone reserved
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SectionConfig {
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>, // hex | "contrast"
    pub opacity: u32,                 // must be explicitly overrided, defaults to 100
    pub spacing: u32,
    pub major_axis_padding: u32,
    pub minor_axis_padding: u32,
    pub border: Border,             // defaults to none
    pub border_radius: Option<u32>, // only freaks touch this
    pub modules: Vec<ModuleConfig>,
}

impl Default for SectionConfig {
    fn default() -> Self {
        Self {
            background: None,
            foreground: None,
            opacity: 100,
            spacing: 8,
            major_axis_padding: 12,
            minor_axis_padding: 12,
            border: Border::default(),
            border_radius: None,
            modules: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarConfig {
    pub enabled: bool,
    pub edge: Edge,
    pub layer: BarLayer,
    pub islands: bool,
    pub autohide: bool,
    pub border: Border,
    pub border_radius: u32,
    pub section_border_radius: u32, // global fallback for sections
    pub backdrop: BarBackdrop,
    pub background: HexColor, // applied when backdrop is None and also serves as fallback for sections
    pub foreground: HexColor,
    pub contrast: bool,
    pub opacity: Alpha, // applied strictly to the background
    pub attachment_mode: AttachmentMode,
    pub major_axis_size: MajorAxisSize, // length
    pub minor_axis_size: NonZeroU32,    // thickness
    pub major_axis_margin: u32,
    pub minor_axis_margin: u32,
    pub major_axis_padding: u32,
    pub minor_axis_padding: u32,
    pub sections: Vec<SectionConfig>,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            edge: Edge::default(),
            layer: BarLayer::Top,
            islands: false,
            autohide: false,
            border: Border::default(),
            border_radius: 12,
            section_border_radius: 12,
            backdrop: BarBackdrop::None,
            background: HexColor::BLACK,
            foreground: HexColor::WHITE,
            contrast: false,
            opacity: Alpha::default(),
            attachment_mode: AttachmentMode::Floating,
            major_axis_size: MajorAxisSize::Percent(75),
            minor_axis_size: NonZeroU32::new(32).unwrap(),
            major_axis_margin: 0,
            minor_axis_margin: 12,
            major_axis_padding: 8,
            minor_axis_padding: 8,
            sections: vec![SectionConfig {
                background: Some(hex_color::HexColor::GREEN),
                foreground: Some(hex_color::HexColor::RED),
                opacity: 255,
                spacing: 24,
                major_axis_padding: 12,
                minor_axis_padding: 12,
                border: Border {
                    width: 0,
                    color: HexColor::WHITE,
                },
                border_radius: None,
                modules: vec![
                    ModuleConfig::Spacer(64),
                    ModuleConfig::Spacer(32),
                    ModuleConfig::Spacer(12),
                ],
            }],
        }
    }
}

impl BarConfig {
    pub fn validate_for_edge(&self, edge: Edge, errors: &mut Vec<ValidationError>) {
        let thickness = self.minor_axis_size.get();
        let total_padding = self.minor_axis_padding * 2;
        if total_padding >= thickness {
            errors.push(ValidationError::PaddingExceedsThickness {
                edge,
                total_padding,
                thickness,
            });
        }

        for (section_idx, section) in self.sections.iter().enumerate() {
            if section.modules.is_empty() {
                errors.push(ValidationError::SectionEmptyModules { edge, section_idx });
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct BarsConfig {
    pub top: BarConfig,
    pub right: BarConfig,
    pub bottom: BarConfig,
    pub left: BarConfig,
    pub screen_corner_claimer: ScreenCornerClaimer,
    pub blend: BarBlend,
}

impl Default for BarsConfig {
    fn default() -> Self {
        let mut top = BarConfig::default();
        top.enabled = true;

        let mut right = BarConfig::default();
        right.edge = Edge::Right;
        let mut bottom = BarConfig::default();
        bottom.edge = Edge::Bottom;
        let mut left = BarConfig::default();
        left.edge = Edge::Left;
        Self {
            top,
            right,
            bottom,
            left,
            screen_corner_claimer: ScreenCornerClaimer::default(),
            blend: BarBlend::default(),
        }
    }
}

impl ConfigDomain for BarsConfig {
    const SCHEMA_NAME: &'static str = "bars";
}

impl ConfigValidate for BarsConfig {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        self.top.validate_for_edge(Edge::Top, &mut errors);
        self.right.validate_for_edge(Edge::Right, &mut errors);
        self.bottom.validate_for_edge(Edge::Bottom, &mut errors);
        self.left.validate_for_edge(Edge::Left, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

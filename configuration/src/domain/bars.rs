use hex_color::HexColor;
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{borrow::Cow, fmt::Display, num::NonZeroU32, str::FromStr};

use crate::{
    domain::ConfigDomain,
    validate::{ConfigValidate, ValidationError},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Edge {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

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

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum MajorAxisSize {
    #[default]
    Auto,
    Percent(u32),
    Pixels(u32),
}

impl Display for MajorAxisSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MajorAxisSize::Auto => write!(f, "auto"),
            MajorAxisSize::Percent(pct) => write!(f, "{pct}%"),
            MajorAxisSize::Pixels(px) => write!(f, "{px}px"),
        }
    }
}

impl FromStr for MajorAxisSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Self::Auto);
        };

        if let Some(pct) = s.strip_suffix("%") {
            return pct
                .parse::<u32>()
                .map(Self::Percent)
                .map_err(|_| format!("invalid percentage: {s:?}"));
        };

        if let Some(px) = s.strip_suffix("px") {
            return px
                .parse::<u32>()
                .map(Self::Pixels)
                .map_err(|_| format!("invalid pixels: {s:?}"));
        };

        Err(format!(
            "expected \"auto\" | percentage like \"75%\" | pixels like \"1024px\" but found {s:?}"
        ))
    }
}

impl Serialize for MajorAxisSize {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for MajorAxisSize {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = Cow::<str>::deserialize(d)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl JsonSchema for MajorAxisSize {
    fn schema_name() -> Cow<'static, str> {
        "majorAxisSize".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "pattern": "^(auto|[0-9]+(\\.[0-9]+)?%|[0-9]+px)$",
            "description": "Length of the bar along its edge: \"auto\" | \"50%\" | \"1920px\"."
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Border {
    pub width: u32,
    pub color: HexColor,
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
    pub opacity: u32, // applied strictly to the background
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
            opacity: 100,
            attachment_mode: AttachmentMode::Floating,
            major_axis_size: MajorAxisSize::Percent(75),
            minor_axis_size: NonZeroU32::new(32).unwrap(),
            major_axis_margin: 0,
            minor_axis_margin: 12,
            major_axis_padding: 8,
            minor_axis_padding: 8,
            sections: Vec::new(),
        }
    }
}

impl BarConfig {
    pub fn validate_for_edge(&self, edge: Edge, errors: &mut Vec<ValidationError>) {
        if self.opacity > 100 {
            errors.push(ValidationError::OpacityOutOfRange {
                edge,
                section_idx: None,
                actual: self.opacity,
            });
        }

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
            if section.opacity > 100 {
                errors.push(ValidationError::OpacityOutOfRange {
                    edge,
                    section_idx: Some(section_idx),
                    actual: section.opacity,
                });
            }

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
        Self {
            top,
            right: BarConfig::default(),
            bottom: BarConfig::default(),
            left: BarConfig::default(),
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

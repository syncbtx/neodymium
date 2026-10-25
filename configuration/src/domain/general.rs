use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{domain::ConfigDomain, validate::ConfigValidate};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub const fn value(self) -> u16 {
        match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Regular => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    #[default]
    Regular,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Font {
    /// A family name ("JetBrains Mono") or a path to a font file ("~/fonts/Inter.ttf").
    pub path_or_name: String,

    pub weight: FontWeight,
    pub style: FontStyle,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            path_or_name: "sans-serif".to_owned(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AutohideConfig {
    pub delay_ms: u32,
    pub duration_ms: u32,
    pub trigger_zone_size: u32,
    pub handle_size: u32,
}

impl Default for AutohideConfig {
    fn default() -> Self {
        Self {
            delay_ms: 150,
            duration_ms: 300,
            trigger_zone_size: 5,
            handle_size: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct GeneralConfig {
    pub autohide: AutohideConfig,
    pub scale: f32,
    pub font: Font,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            autohide: AutohideConfig::default(),
            scale: 1.0,
            font: Font::default(),
        }
    }
}

impl ConfigDomain for GeneralConfig {
    const SCHEMA_NAME: &'static str = "general";
}

impl ConfigValidate for GeneralConfig {
    fn validate(&self) -> Result<(), Vec<crate::validate::ValidationError>> {
        Ok(())
    }
}

use hex_color::HexColor;
use model::Alpha;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::validate::Sanitize;

mod preconfigured;
mod seed_color;
mod wallpaper;

pub use preconfigured::*;
pub use seed_color::*;
pub use wallpaper::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ColorScheme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SchemeType {
    #[default]
    SchemeTonalSpot,
    SchemeVibrant,
    SchemeExpressive,
    SchemeFruitSalad,
    SchemeRainbow,
    SchemeMonochrome,
    SchemeNeutral,
    SchemeFidelity,
    SchemeContent,
    SchemeSmart,
}

impl SchemeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemeTonalSpot => "scheme-tonal-spot",
            Self::SchemeVibrant => "scheme-vibrant",
            Self::SchemeExpressive => "scheme-expressive",
            Self::SchemeFruitSalad => "scheme-fruit-salad",
            Self::SchemeRainbow => "scheme-rainbow",
            Self::SchemeMonochrome => "scheme-monochrome",
            Self::SchemeNeutral => "scheme-neutral",
            Self::SchemeFidelity => "scheme-fidelity",
            Self::SchemeContent => "scheme-content",
            Self::SchemeSmart => "scheme-smart",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ColorPreference {
    Saturation,
    Darkness,
    Lightness,
    LessSaturation,
    Value,
    ClosestToFallback,
}

impl ColorPreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Saturation => "saturation",
            Self::Darkness => "darkness",
            Self::Lightness => "lightness",
            Self::LessSaturation => "less-saturation",
            Self::Value => "value",
            Self::ClosestToFallback => "closest-to-fallback",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ResizeFilter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl ResizeFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Triangle => "triangle",
            Self::CatmullRom => "catmull-rom",
            Self::Gaussian => "gaussian",
            Self::Lanczos3 => "lanczos3",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ThemeSource {
    #[default]
    Wallpaper,
    SeedColor,
    PreConfigured,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct ThemeSettings {
    pub mode: ColorScheme,
    pub active: ThemeSource,
    pub wallpaper: WallpaperTheme,
    pub seed_color: SeedColorTheme,
    pub pre_configured: PreConfiguredTheme,
}

impl ThemeSettings {
    pub fn active_opacity(&self) -> Alpha {
        match self.active {
            ThemeSource::Wallpaper => self.wallpaper.opacity,
            ThemeSource::SeedColor => self.seed_color.opacity,
            ThemeSource::PreConfigured => Alpha::default(),
        }
    }
}

impl Sanitize for ThemeSettings {
    fn sanitize(&mut self) {
        self.wallpaper.sanitize();
        self.seed_color.sanitize();
    }
}

#[derive(Debug, Copy, Clone, Default, Deserialize)]
pub struct ColorEntry {
    color: HexColor,
}

#[derive(Debug, Copy, Clone, Default, Deserialize)]
pub struct ModeEntry {
    dark: ColorEntry,
    light: ColorEntry,
}

#[derive(Deserialize)]
pub struct MatugenOutput {
    colors: model::Palette<ModeEntry>,
}

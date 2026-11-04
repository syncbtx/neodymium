use hex_color::HexColor;
use model::Alpha;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::validate::Sanitize;

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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct WallpaperTheme {
    pub index: Option<usize>,
    pub contrast: f32,
    pub opacity: Alpha,
    pub scheme_type: SchemeType,
    pub prefer: Option<ColorPreference>,
    pub resize_filter: Option<ResizeFilter>,
    pub lightness_dark: Option<f32>,
    pub lightness_light: Option<f32>,
    pub fallback_color: Option<HexColor>,
}

pub type WallpaperSettings = WallpaperTheme;

impl Default for WallpaperTheme {
    fn default() -> Self {
        Self {
            index: None,
            contrast: 0.0,
            opacity: Alpha::default(),
            scheme_type: SchemeType::SchemeTonalSpot,
            prefer: None,
            resize_filter: None,
            lightness_dark: None,
            lightness_light: None,
            fallback_color: None,
        }
    }
}

impl Sanitize for WallpaperTheme {
    fn sanitize(&mut self) {
        if let Some(idx) = &mut self.index {
            *idx = (*idx).min(4);
        }
        self.contrast = self.contrast.clamp(-1.0, 1.0);
        if let Some(ld) = &mut self.lightness_dark {
            *ld = ld.min(1.0);
        }
        if let Some(ll) = &mut self.lightness_light {
            *ll = ll.max(-1.0);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct SeedColorTheme {
    pub seed_color: HexColor,
    pub contrast: f32,
    pub opacity: Alpha,
    pub scheme_type: SchemeType,
    pub lightness_dark: Option<f32>,
    pub lightness_light: Option<f32>,
}

pub type SeedColorSettings = SeedColorTheme;

impl Default for SeedColorTheme {
    fn default() -> Self {
        Self {
            seed_color: HexColor::rgb(0x41, 0x5f, 0x91),
            contrast: 0.0,
            opacity: Alpha::default(),
            scheme_type: SchemeType::SchemeTonalSpot,
            lightness_dark: None,
            lightness_light: None,
        }
    }
}

impl Sanitize for SeedColorTheme {
    fn sanitize(&mut self) {
        self.contrast = self.contrast.clamp(-1.0, 1.0);
        if let Some(ld) = &mut self.lightness_dark {
            *ld = ld.min(1.0);
        }
        if let Some(ll) = &mut self.lightness_light {
            *ll = ll.max(-1.0);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PreConfiguredTheme {
    #[default]
    Catppuccin,
    Gruvbox,
    Nord,
    TokyoNight,
    Solarised,
    Monokai,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ActiveThemeSource {
    #[default]
    Wallpaper,
    SeedColor,
    PreConfigured,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct ThemeSettings {
    pub mode: ColorScheme,
    pub active: ActiveThemeSource,
    pub wallpaper: WallpaperTheme,
    pub seed_color: SeedColorTheme,
    pub pre_configured: PreConfiguredTheme,
}

pub type ThemeSetting = ThemeSettings;

impl ThemeSettings {
    pub fn active_opacity(&self) -> Alpha {
        match self.active {
            ActiveThemeSource::Wallpaper => self.wallpaper.opacity,
            ActiveThemeSource::SeedColor => self.seed_color.opacity,
            ActiveThemeSource::PreConfigured => Alpha::default(),
        }
    }
}

impl Sanitize for ThemeSettings {
    fn sanitize(&mut self) {
        self.wallpaper.sanitize();
        self.seed_color.sanitize();
    }
}

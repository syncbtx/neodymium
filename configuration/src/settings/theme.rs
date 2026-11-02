use hex_color::HexColor;
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
    #[serde(alias = "tonal-spot", alias = "tonalSpot")]
    SchemeTonalSpot,
    #[serde(alias = "vibrant")]
    SchemeVibrant,
    #[serde(alias = "expressive")]
    SchemeExpressive,
    #[serde(alias = "fruit-salad", alias = "fruitSalad")]
    SchemeFruitSalad,
    #[serde(alias = "rainbow")]
    SchemeRainbow,
    #[serde(alias = "monochrome")]
    SchemeMonochrome,
    #[serde(alias = "neutral")]
    SchemeNeutral,
    #[serde(alias = "fidelity")]
    SchemeFidelity,
    #[serde(alias = "content")]
    SchemeContent,
    #[serde(alias = "smart")]
    SchemeSmart,
}

impl SchemeType {
    pub const fn as_str(&self) -> &'static str {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ColorPreference {
    #[default]
    Saturation,
    Darkness,
    Lightness,
    #[serde(alias = "lessSaturation")]
    LessSaturation,
    Value,
    #[serde(alias = "closestToFallback")]
    ClosestToFallback,
}

impl ColorPreference {
    pub const fn as_str(&self) -> &'static str {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ResizeFilter {
    #[default]
    Nearest,
    Triangle,
    #[serde(alias = "catmullRom")]
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl ResizeFilter {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Nearest => "nearest",
            Self::Triangle => "triangle",
            Self::CatmullRom => "catmull-rom",
            Self::Gaussian => "gaussian",
            Self::Lanczos3 => "lanczos3",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, JsonSchema)]
pub struct ThemeOpacity(pub u8);

impl Default for ThemeOpacity {
    fn default() -> Self {
        Self(255)
    }
}

impl From<u8> for ThemeOpacity {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<f32> for ThemeOpacity {
    fn from(val: f32) -> Self {
        if val <= 1.0 {
            Self((val.clamp(0.0, 1.0) * 255.0).round() as u8)
        } else {
            Self(val.clamp(0.0, 255.0).round() as u8)
        }
    }
}

impl ThemeOpacity {
    pub const fn value(&self) -> u8 {
        self.0
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32 / 255.0
    }
}

impl<'de> Deserialize<'de> for ThemeOpacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ThemeOpacityVisitor;

        impl<'de> serde::de::Visitor<'de> for ThemeOpacityVisitor {
            type Value = ThemeOpacity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an opacity integer (0..=255) or float (0.0..=1.0)")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v <= 255 {
                    Ok(ThemeOpacity(v as u8))
                } else {
                    Err(E::custom(format!("opacity {v} is out of range 0..255")))
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if (0..=255).contains(&v) {
                    Ok(ThemeOpacity(v as u8))
                } else {
                    Err(E::custom(format!("opacity {v} is out of range 0..255")))
                }
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if (0.0..=1.0).contains(&v) {
                    Ok(ThemeOpacity((v * 255.0).round() as u8))
                } else if (1.0..=255.0).contains(&v) {
                    Ok(ThemeOpacity(v.round() as u8))
                } else {
                    Err(E::custom(format!(
                        "opacity {v} is out of range (expected 0.0..=1.0 or 0..=255)"
                    )))
                }
            }
        }

        deserializer.deserialize_any(ThemeOpacityVisitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct WallpaperTheme {
    #[serde(alias = "source_color_index", alias = "sourceColorIndex")]
    pub index: Option<usize>,
    pub contrast: f32,
    #[serde(alias = "base_opacity", alias = "baseOpacity")]
    pub opacity: ThemeOpacity,
    #[serde(alias = "type", alias = "scheme", alias = "scheme_type")]
    pub scheme_type: SchemeType,
    pub prefer: Option<ColorPreference>,
    #[serde(alias = "resize_filter")]
    pub resize_filter: Option<ResizeFilter>,
    #[serde(alias = "lightness_dark")]
    pub lightness_dark: Option<f32>,
    #[serde(alias = "lightness_light")]
    pub lightness_light: Option<f32>,
    #[serde(alias = "fallback_color")]
    pub fallback_color: Option<HexColor>,
}

pub type WallpaperSettings = WallpaperTheme;

impl Default for WallpaperTheme {
    fn default() -> Self {
        Self {
            index: None,
            contrast: 0.0,
            opacity: ThemeOpacity::default(),
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
    #[serde(alias = "seed_color", alias = "color")]
    pub seed_color: HexColor,
    pub contrast: f32,
    #[serde(alias = "base_opacity", alias = "baseOpacity")]
    pub opacity: ThemeOpacity,
    #[serde(alias = "type", alias = "scheme", alias = "scheme_type")]
    pub scheme_type: SchemeType,
    #[serde(alias = "lightness_dark")]
    pub lightness_dark: Option<f32>,
    #[serde(alias = "lightness_light")]
    pub lightness_light: Option<f32>,
}

pub type SeedColorSettings = SeedColorTheme;

impl Default for SeedColorTheme {
    fn default() -> Self {
        Self {
            seed_color: HexColor::rgb(0x41, 0x5f, 0x91),
            contrast: 0.0,
            opacity: ThemeOpacity::default(),
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
    #[serde(alias = "seedColor")]
    pub seed_color: SeedColorTheme,
    #[serde(alias = "preConfigured")]
    pub pre_configured: PreConfiguredTheme,
}

pub type ThemeSetting = ThemeSettings;

impl ThemeSettings {
    pub fn active_opacity(&self) -> u8 {
        match self.active {
            ActiveThemeSource::Wallpaper => self.wallpaper.opacity.0,
            ActiveThemeSource::SeedColor => self.seed_color.opacity.0,
            ActiveThemeSource::PreConfigured => 255,
        }
    }
}

impl Sanitize for ThemeSettings {
    fn sanitize(&mut self) {
        self.wallpaper.sanitize();
        self.seed_color.sanitize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_faithful_section_10() {
        let json = r##"{
            "mode": "dark",
            "active": "wallpaper",
            "wallpaper": {
                "index": 2,
                "contrast": 0.5,
                "opacity": 200,
                "type": "scheme-vibrant",
                "prefer": "saturation",
                "resizeFilter": "gaussian",
                "lightnessDark": -0.2,
                "lightnessLight": 0.3,
                "fallbackColor": "#1e1e2e"
            },
            "seedColor": {
                "color": "#ff0055",
                "contrast": -0.2,
                "baseOpacity": 0.8,
                "scheme": "expressive",
                "lightnessDark": 0.0,
                "lightnessLight": 0.1
            },
            "preConfigured": "nord"
        }"##;

        let settings: ThemeSettings = serde_json::from_str(json).expect("failed to deserialize theme settings");
        assert_eq!(settings.wallpaper.index, Some(2));
        assert_eq!(settings.wallpaper.contrast, 0.5);
        assert_eq!(settings.wallpaper.opacity.value(), 200);
        assert_eq!(settings.wallpaper.scheme_type, SchemeType::SchemeVibrant);
        assert_eq!(settings.wallpaper.prefer, Some(ColorPreference::Saturation));
        assert_eq!(settings.wallpaper.resize_filter, Some(ResizeFilter::Gaussian));
        assert_eq!(settings.wallpaper.lightness_dark, Some(-0.2));
        assert_eq!(settings.wallpaper.lightness_light, Some(0.3));
        assert_eq!(settings.wallpaper.fallback_color, Some(HexColor::rgb(0x1e, 0x1e, 0x2e)));

        assert_eq!(settings.seed_color.seed_color, HexColor::rgb(0xff, 0x00, 0x55));
        assert_eq!(settings.seed_color.contrast, -0.2);
        assert_eq!(settings.seed_color.opacity.value(), (0.8f32 * 255.0).round() as u8);
        assert_eq!(settings.seed_color.scheme_type, SchemeType::SchemeExpressive);
    }
}

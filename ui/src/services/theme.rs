use std::{path::Path, str::FromStr};

use amane::Service;
use configuration::settings::theme::{
    ActiveThemeSource, ColorScheme, PreConfiguredTheme, SeedColorTheme, ThemeSetting,
    WallpaperTheme,
};
use hex_color::HexColor;
use model::palette::{Palette, Theme as ModelTheme};

#[derive(Debug, Clone)]
pub struct ThemeStore {
    pub wallpaper: ModelTheme,
    pub seed_color: ModelTheme,
    pub pre_configured: ModelTheme,
}

impl Default for ThemeStore {
    fn default() -> Self {
        Self {
            wallpaper: ModelTheme::default(),
            seed_color: ModelTheme::default(),
            pre_configured: pre_configured_theme(PreConfiguredTheme::Catppuccin),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub store: ThemeStore,
    pub active_source: ActiveThemeSource,
    pub mode: ColorScheme,
    pub opacity: u8,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            store: ThemeStore::default(),
            active_source: ActiveThemeSource::Wallpaper,
            mode: ColorScheme::Dark,
            opacity: 255,
        }
    }
}

impl Service for Theme {
    fn new() -> Self {
        Self::default()
    }
}

impl Theme {
    pub fn active(&self) -> &Palette {
        let theme = match self.active_source {
            ActiveThemeSource::Wallpaper => &self.store.wallpaper,
            ActiveThemeSource::SeedColor => &self.store.seed_color,
            ActiveThemeSource::PreConfigured => &self.store.pre_configured,
        };

        match self.mode {
            ColorScheme::Dark => &theme.dark,
            ColorScheme::Light => &theme.light,
        }
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            ColorScheme::Dark => ColorScheme::Light,
            ColorScheme::Light => ColorScheme::Dark,
        };
    }

    /// Apply cheap/fast metadata fields only. Never call this while holding
    /// the write lock alongside a slow computation.
    pub fn set_meta(&mut self, settings: &ThemeSetting) {
        self.mode = settings.mode;
        self.active_source = settings.active;
        self.opacity = settings.active_opacity().0;
    }

    pub fn set_pre_configured(&mut self, palette: ModelTheme) {
        self.store.pre_configured = palette;
    }

    pub fn set_seed_color(&mut self, palette: ModelTheme) {
        self.store.seed_color = palette;
    }

    pub fn set_wallpaper(&mut self, palette: ModelTheme) {
        self.store.wallpaper = palette;
    }
}

pub fn generate_from_seed(settings: &SeedColorTheme) -> Option<ModelTheme> {
    let hex_str = format!(
        "#{:02x}{:02x}{:02x}",
        settings.seed_color.r, settings.seed_color.g, settings.seed_color.b
    );
    let mut cmd = format!(
        "matugen color hex '{hex_str}' -j hex --dry-run -q --type {} --contrast {}",
        settings.scheme_type.as_str(),
        settings.contrast
    );

    if let Some(ld) = settings.lightness_dark {
        cmd.push_str(&format!(" --lightness-dark {ld}"));
    }
    if let Some(ll) = settings.lightness_light {
        cmd.push_str(&format!(" --lightness-light {ll}"));
    }

    let stdout = amane::output(&cmd);
    parse_matugen_output(&stdout)
}

pub fn generate_from_wallpaper(path: &Path, settings: &WallpaperTheme) -> Option<ModelTheme> {
    let run_cmd = |index_override: Option<usize>| -> Option<ModelTheme> {
        let path_str = path.to_string_lossy();
        let mut cmd = format!(
            "matugen image '{path_str}' -j hex --dry-run -q --type {} --contrast {}",
            settings.scheme_type.as_str(),
            settings.contrast
        );

        let eff_idx = index_override.or(settings.index);
        if let Some(idx) = eff_idx {
            cmd.push_str(&format!(" --source-color-index {idx}"));
            if let Some(prefer) = settings.prefer {
                cmd.push_str(&format!(" --prefer {}", prefer.as_str()));
            }
        } else if let Some(prefer) = settings.prefer {
            cmd.push_str(&format!(" --prefer {}", prefer.as_str()));
        } else {
            cmd.push_str(" --source-color-index 0");
        }

        if let Some(filter) = settings.resize_filter {
            cmd.push_str(&format!(" --resize-filter {}", filter.as_str()));
        }
        if let Some(ld) = settings.lightness_dark {
            cmd.push_str(&format!(" --lightness-dark {ld}"));
        }
        if let Some(ll) = settings.lightness_light {
            cmd.push_str(&format!(" --lightness-light {ll}"));
        }
        if let Some(fc) = settings.fallback_color {
            let hex_str = format!("#{:02x}{:02x}{:02x}", fc.r, fc.g, fc.b);
            cmd.push_str(&format!(" --fallback-color '{hex_str}'"));
        }

        let stdout = amane::output(&cmd);
        parse_matugen_output(&stdout)
    };

    if let Some(palette) = run_cmd(None) {
        Some(palette)
    } else if settings.index.is_some() && settings.index != Some(0) {
        run_cmd(Some(0))
    } else {
        None
    }
}

fn parse_matugen_output(json_str: &str) -> Option<ModelTheme> {
    let root: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let colors = root.get("colors")?.as_object()?;

    let get_color = |key: &str, mode: &str| -> Option<HexColor> {
        let hex_str = colors.get(key)?.get(mode)?.get("color")?.as_str()?;
        HexColor::from_str(hex_str).ok()
    };

    let build_palette = |mode: &str, fallback: Palette| -> Palette {
        Palette {
            primary: get_color("primary", mode).unwrap_or(fallback.primary),
            on_primary: get_color("on_primary", mode).unwrap_or(fallback.on_primary),
            secondary: get_color("secondary", mode).unwrap_or(fallback.secondary),
            on_secondary: get_color("on_secondary", mode).unwrap_or(fallback.on_secondary),
            tertiary: get_color("tertiary", mode).unwrap_or(fallback.tertiary),
            on_tertiary: get_color("on_tertiary", mode).unwrap_or(fallback.on_tertiary),
            error: get_color("error", mode).unwrap_or(fallback.error),
            on_error: get_color("on_error", mode).unwrap_or(fallback.on_error),
            surface: get_color("surface", mode).unwrap_or(fallback.surface),
            surface_container: get_color("surface_container", mode)
                .unwrap_or(fallback.surface_container),
            surface_container_low: get_color("surface_container_low", mode)
                .unwrap_or(fallback.surface_container_low),
            surface_container_high: get_color("surface_container_high", mode)
                .unwrap_or(fallback.surface_container_high),
            surface_container_highest: get_color("surface_container_highest", mode)
                .unwrap_or(fallback.surface_container_highest),
            on_surface: get_color("on_surface", mode).unwrap_or(fallback.on_surface),
            on_surface_variant: get_color("on_surface_variant", mode)
                .unwrap_or(fallback.on_surface_variant),
            outline: get_color("outline", mode).unwrap_or(fallback.outline),
            outline_variant: get_color("outline_variant", mode).unwrap_or(fallback.outline_variant),
        }
    };

    Some(ModelTheme {
        dark: build_palette("dark", Palette::fallback_dark()),
        light: build_palette("light", Palette::fallback_light()),
    })
}

pub fn pre_configured_theme(theme: PreConfiguredTheme) -> ModelTheme {
    match theme {
        PreConfiguredTheme::Catppuccin => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0x89, 0xb4, 0xfa),
                on_primary: HexColor::rgb(0x11, 0x11, 0x1b),
                secondary: HexColor::rgb(0xb4, 0xbe, 0xfe),
                on_secondary: HexColor::rgb(0x18, 0x18, 0x25),
                tertiary: HexColor::rgb(0xf5, 0xc2, 0xe7),
                on_tertiary: HexColor::rgb(0x31, 0x32, 0x44),
                error: HexColor::rgb(0xf3, 0x8b, 0xa8),
                on_error: HexColor::rgb(0x11, 0x11, 0x1b),
                surface: HexColor::rgb(0x11, 0x11, 0x1b),
                surface_container_low: HexColor::rgb(0x18, 0x18, 0x25),
                surface_container: HexColor::rgb(0x1e, 0x1e, 0x2e),
                surface_container_high: HexColor::rgb(0x31, 0x32, 0x44),
                surface_container_highest: HexColor::rgb(0x45, 0x47, 0x5a),
                on_surface: HexColor::rgb(0xcd, 0xd6, 0xf4),
                on_surface_variant: HexColor::rgb(0xa6, 0xad, 0xc8),
                outline: HexColor::rgb(0x58, 0x5b, 0x70),
                outline_variant: HexColor::rgb(0x45, 0x47, 0x5a),
            },
            light: Palette {
                primary: HexColor::rgb(0x1e, 0x66, 0xf5),
                on_primary: HexColor::rgb(0xff, 0xff, 0xff),
                secondary: HexColor::rgb(0x72, 0x87, 0xfd),
                on_secondary: HexColor::rgb(0xff, 0xff, 0xff),
                tertiary: HexColor::rgb(0xea, 0x76, 0xcb),
                on_tertiary: HexColor::rgb(0xff, 0xff, 0xff),
                error: HexColor::rgb(0xd2, 0x0f, 0x39),
                on_error: HexColor::rgb(0xff, 0xff, 0xff),
                surface: HexColor::rgb(0xef, 0xf1, 0xf5),
                surface_container_low: HexColor::rgb(0xe6, 0xe9, 0xef),
                surface_container: HexColor::rgb(0xdc, 0xce, 0xe0),
                surface_container_high: HexColor::rgb(0xcc, 0xd0, 0xda),
                surface_container_highest: HexColor::rgb(0xbc, 0xc0, 0xcc),
                on_surface: HexColor::rgb(0x4c, 0x4f, 0x69),
                on_surface_variant: HexColor::rgb(0x6c, 0x6f, 0x85),
                outline: HexColor::rgb(0x9c, 0xa0, 0xb0),
                outline_variant: HexColor::rgb(0xac, 0xb0, 0xbe),
            },
        },
        PreConfiguredTheme::Gruvbox => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0xfe, 0x80, 0x19),
                on_primary: HexColor::rgb(0x28, 0x28, 0x28),
                secondary: HexColor::rgb(0x8e, 0xc0, 0x7c),
                on_secondary: HexColor::rgb(0x28, 0x28, 0x28),
                tertiary: HexColor::rgb(0xfa, 0xbd, 0x2f),
                on_tertiary: HexColor::rgb(0x28, 0x28, 0x28),
                error: HexColor::rgb(0xfb, 0x49, 0x34),
                on_error: HexColor::rgb(0x28, 0x28, 0x28),
                surface: HexColor::rgb(0x1d, 0x20, 0x21),
                surface_container_low: HexColor::rgb(0x28, 0x28, 0x28),
                surface_container: HexColor::rgb(0x32, 0x30, 0x2f),
                surface_container_high: HexColor::rgb(0x3c, 0x38, 0x36),
                surface_container_highest: HexColor::rgb(0x50, 0x49, 0x45),
                on_surface: HexColor::rgb(0xeb, 0xdb, 0xb2),
                on_surface_variant: HexColor::rgb(0xd5, 0xc4, 0xa1),
                outline: HexColor::rgb(0x66, 0x5c, 0x54),
                outline_variant: HexColor::rgb(0x50, 0x49, 0x45),
            },
            light: Palette {
                primary: HexColor::rgb(0xaf, 0x3a, 0x03),
                on_primary: HexColor::rgb(0xff, 0xff, 0xff),
                secondary: HexColor::rgb(0x42, 0x7b, 0x58),
                on_secondary: HexColor::rgb(0xff, 0xff, 0xff),
                tertiary: HexColor::rgb(0xb5, 0x76, 0x14),
                on_tertiary: HexColor::rgb(0xff, 0xff, 0xff),
                error: HexColor::rgb(0x9d, 0x00, 0x06),
                on_error: HexColor::rgb(0xff, 0xff, 0xff),
                surface: HexColor::rgb(0xfb, 0xf1, 0xc7),
                surface_container_low: HexColor::rgb(0xf2, 0xe5, 0xbc),
                surface_container: HexColor::rgb(0xeb, 0xdb, 0xb2),
                surface_container_high: HexColor::rgb(0xd5, 0xc4, 0xa1),
                surface_container_highest: HexColor::rgb(0xbd, 0xae, 0x93),
                on_surface: HexColor::rgb(0x3c, 0x38, 0x36),
                on_surface_variant: HexColor::rgb(0x66, 0x5c, 0x54),
                outline: HexColor::rgb(0x7c, 0x6f, 0x64),
                outline_variant: HexColor::rgb(0x92, 0x83, 0x74),
            },
        },
        PreConfiguredTheme::Nord => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0x88, 0xc0, 0xd0),
                on_primary: HexColor::rgb(0x2e, 0x34, 0x40),
                secondary: HexColor::rgb(0x81, 0xa1, 0xc1),
                on_secondary: HexColor::rgb(0x2e, 0x34, 0x40),
                tertiary: HexColor::rgb(0xeb, 0xcb, 0x8b),
                on_tertiary: HexColor::rgb(0x2e, 0x34, 0x40),
                error: HexColor::rgb(0xbf, 0x61, 0x6a),
                on_error: HexColor::rgb(0x2e, 0x34, 0x40),
                surface: HexColor::rgb(0x24, 0x29, 0x33),
                surface_container_low: HexColor::rgb(0x2e, 0x34, 0x40),
                surface_container: HexColor::rgb(0x3b, 0x42, 0x52),
                surface_container_high: HexColor::rgb(0x43, 0x4c, 0x5e),
                surface_container_highest: HexColor::rgb(0x4c, 0x56, 0x6a),
                on_surface: HexColor::rgb(0xec, 0xef, 0xf4),
                on_surface_variant: HexColor::rgb(0xd8, 0xde, 0xe9),
                outline: HexColor::rgb(0x4c, 0x56, 0x6a),
                outline_variant: HexColor::rgb(0x43, 0x4c, 0x5e),
            },
            light: Palette::fallback_light(),
        },
        PreConfiguredTheme::TokyoNight => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0x7a, 0xa2, 0xf7),
                on_primary: HexColor::rgb(0x1a, 0x1b, 0x26),
                secondary: HexColor::rgb(0xbb, 0x9a, 0xf7),
                on_secondary: HexColor::rgb(0x1a, 0x1b, 0x26),
                tertiary: HexColor::rgb(0xe0, 0xaf, 0x68),
                on_tertiary: HexColor::rgb(0x1a, 0x1b, 0x26),
                error: HexColor::rgb(0xf7, 0x76, 0x8e),
                on_error: HexColor::rgb(0x1a, 0x1b, 0x26),
                surface: HexColor::rgb(0x16, 0x16, 0x1e),
                surface_container_low: HexColor::rgb(0x1a, 0x1b, 0x26),
                surface_container: HexColor::rgb(0x24, 0x28, 0x3b),
                surface_container_high: HexColor::rgb(0x2f, 0x35, 0x49),
                surface_container_highest: HexColor::rgb(0x41, 0x48, 0x68),
                on_surface: HexColor::rgb(0xc0, 0xca, 0xf5),
                on_surface_variant: HexColor::rgb(0x9a, 0xa5, 0xce),
                outline: HexColor::rgb(0x56, 0x5f, 0x89),
                outline_variant: HexColor::rgb(0x41, 0x48, 0x68),
            },
            light: Palette::fallback_light(),
        },
        PreConfiguredTheme::Solarised => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0x26, 0x8b, 0xd2),
                on_primary: HexColor::rgb(0x00, 0x2b, 0x36),
                secondary: HexColor::rgb(0x2a, 0xa1, 0x98),
                on_secondary: HexColor::rgb(0x00, 0x2b, 0x36),
                tertiary: HexColor::rgb(0xb5, 0x89, 0x00),
                on_tertiary: HexColor::rgb(0x00, 0x2b, 0x36),
                error: HexColor::rgb(0xdc, 0x32, 0x2f),
                on_error: HexColor::rgb(0x00, 0x2b, 0x36),
                surface: HexColor::rgb(0x00, 0x21, 0x2b),
                surface_container_low: HexColor::rgb(0x00, 0x2b, 0x36),
                surface_container: HexColor::rgb(0x07, 0x36, 0x42),
                surface_container_high: HexColor::rgb(0x58, 0x6e, 0x75),
                surface_container_highest: HexColor::rgb(0x65, 0x7b, 0x83),
                on_surface: HexColor::rgb(0x93, 0xa1, 0xa1),
                on_surface_variant: HexColor::rgb(0x83, 0x94, 0x96),
                outline: HexColor::rgb(0x58, 0x6e, 0x75),
                outline_variant: HexColor::rgb(0x07, 0x36, 0x42),
            },
            light: Palette::fallback_light(),
        },
        PreConfiguredTheme::Monokai => ModelTheme {
            dark: Palette {
                primary: HexColor::rgb(0xa6, 0xe2, 0x2e),
                on_primary: HexColor::rgb(0x27, 0x28, 0x22),
                secondary: HexColor::rgb(0x66, 0xd9, 0xef),
                on_secondary: HexColor::rgb(0x27, 0x28, 0x22),
                tertiary: HexColor::rgb(0xfd, 0x97, 0x1f),
                on_tertiary: HexColor::rgb(0x27, 0x28, 0x22),
                error: HexColor::rgb(0xf9, 0x26, 0x72),
                on_error: HexColor::rgb(0xff, 0xff, 0xff),
                surface: HexColor::rgb(0x1e, 0x1f, 0x1c),
                surface_container_low: HexColor::rgb(0x27, 0x28, 0x22),
                surface_container: HexColor::rgb(0x38, 0x38, 0x30),
                surface_container_high: HexColor::rgb(0x49, 0x48, 0x3e),
                surface_container_highest: HexColor::rgb(0x75, 0x71, 0x5e),
                on_surface: HexColor::rgb(0xf8, 0xf8, 0xf2),
                on_surface_variant: HexColor::rgb(0xcf, 0xcf, 0xc2),
                outline: HexColor::rgb(0x75, 0x71, 0x5e),
                outline_variant: HexColor::rgb(0x49, 0x48, 0x3e),
            },
            light: Palette::fallback_light(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_toggle() {
        let mut theme = Theme::default();
        assert_eq!(theme.mode, ColorScheme::Dark);
        theme.toggle();
        assert_eq!(theme.mode, ColorScheme::Light);
        theme.toggle();
        assert_eq!(theme.mode, ColorScheme::Dark);
    }

    #[test]
    fn test_pre_configured_themes() {
        for preset in [
            PreConfiguredTheme::Catppuccin,
            PreConfiguredTheme::Gruvbox,
            PreConfiguredTheme::Nord,
            PreConfiguredTheme::TokyoNight,
            PreConfiguredTheme::Solarised,
            PreConfiguredTheme::Monokai,
        ] {
            let t = pre_configured_theme(preset);
            assert_ne!(t.dark.primary, t.dark.on_primary);
            assert_ne!(t.light.primary, t.light.on_primary);
        }
    }

    #[test]
    fn test_seed_color_generation_full_args() {
        let settings = SeedColorTheme {
            seed_color: HexColor::rgb(0x41, 0x5f, 0x91),
            contrast: 0.2,
            opacity: model::Alpha(255),
            scheme_type: configuration::settings::theme::SchemeType::SchemeVibrant,
            lightness_dark: Some(0.1),
            lightness_light: Some(0.2),
        };
        let res = generate_from_seed(&settings);
        assert!(res.is_some(), "matugen failed to generate from seed");
        let theme = res.unwrap();
        assert_ne!(theme.dark.primary, theme.dark.surface);
        assert_ne!(theme.light.primary, theme.light.surface);
    }

    #[test]
    fn test_wallpaper_extraction_fallback() {
        let serp_wp = Path::new("/home/syncbtx/.cache/serpantinum/wallpaper/current_wallpaper.png");
        if serp_wp.exists() {
            let settings = WallpaperTheme {
                index: Some(1),
                contrast: 0.1,
                opacity: model::Alpha(255),
                scheme_type: configuration::settings::theme::SchemeType::SchemeFruitSalad,
                prefer: Some(configuration::settings::theme::ColorPreference::Saturation),
                resize_filter: Some(configuration::settings::theme::ResizeFilter::Gaussian),
                lightness_dark: Some(-0.1),
                lightness_light: Some(0.1),
                fallback_color: Some(HexColor::rgb(0x1e, 0x1e, 0x2e)),
            };
            let res = generate_from_wallpaper(serp_wp, &settings);
            assert!(
                res.is_some(),
                "matugen failed on serpantinum wallpaper placeholder with full Section 10 args"
            );
            let theme = res.unwrap();
            assert_ne!(theme.dark.primary, theme.dark.surface);
            assert_ne!(theme.light.primary, theme.light.surface);
        }
    }
}

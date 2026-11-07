use std::path::PathBuf;

use amane::Service;
use configuration::{
    resolution,
    resolve::Resolve,
    settings::{
        ColorScheme, PreConfiguredTheme, SeedColorTheme, ThemeSettings, ThemeSource, WallpaperTheme,
    },
};
use model::palette::{ModelTheme, Palette};

#[derive(Debug, Clone, Default)]
pub struct ThemeCache {
    pub wallpaper: Option<(WallpaperTheme, Option<PathBuf>)>,
    pub seed_color: Option<SeedColorTheme>,
    pub pre_configured: Option<PreConfiguredTheme>,
}

#[derive(Debug, Clone, Default)]
pub struct ThemeStore {
    pub wallpaper: Option<ModelTheme>,
    pub seed_color: Option<ModelTheme>,
    pub pre_configured: Option<ModelTheme>,
}

#[derive(Debug, Clone)]
pub struct ThemeEngine {
    pub store: ThemeStore,
    pub cache: ThemeCache,
    pub source: ThemeSource,
    pub mode: ColorScheme,
    pub opacity: u8,
}

impl Service for ThemeEngine {
    fn new() -> Self {
        Self {
            store: ThemeStore::default(),
            cache: ThemeCache::default(),
            source: ThemeSource::Wallpaper,
            mode: ColorScheme::default(),
            opacity: 255,
        }
    }
}

#[allow(unused)]
impl ThemeEngine {
    pub fn active_palette(&self) -> Palette {
        let theme = match self.source {
            ThemeSource::Wallpaper => self.store.wallpaper,
            ThemeSource::SeedColor => self.store.seed_color,
            ThemeSource::PreConfigured => self.store.pre_configured,
        };
        let theme = match theme {
            Some(t) => t,
            None => {
                println!(
                    "{}",
                    "Selected theme doesn't exist, falling back to the default"
                );
                ModelTheme::default()
            }
        };
        match self.mode {
            ColorScheme::Dark => theme.dark,
            ColorScheme::Light => theme.light,
        }
    }

    pub fn ensure_palette(&mut self, settings: &ThemeSettings) {
        match self.source {
            ThemeSource::Wallpaper => {
                let current_wp = resolution::resolve_active_wallpaper();
                let needs_resolve = self.store.wallpaper.is_none()
                    || self.cache.wallpaper != Some((settings.wallpaper, current_wp.clone()));
                if needs_resolve {
                    if let Some(resolved) = settings.wallpaper.resolve() {
                        self.store.wallpaper = Some(resolved);
                        self.cache.wallpaper = Some((settings.wallpaper, current_wp));
                    }
                }
            }
            ThemeSource::SeedColor => {
                let needs_resolve = self.store.seed_color.is_none()
                    || self.cache.seed_color != Some(settings.seed_color);
                if needs_resolve {
                    if let Some(resolved) = settings.seed_color.resolve() {
                        self.store.seed_color = Some(resolved);
                        self.cache.seed_color = Some(settings.seed_color);
                    }
                }
            }
            ThemeSource::PreConfigured => {
                let current_theme = settings.pre_configured;
                let needs_resolve = self.store.pre_configured.is_none()
                    || self.cache.pre_configured != Some(current_theme);
                if needs_resolve {
                    self.store.pre_configured = Some(settings.pre_configured.resolve());
                    self.cache.pre_configured = Some(current_theme);
                }
            }
        }
    }

    pub fn update(&mut self, settings: &ThemeSettings) {
        self.mode = settings.mode;
        self.opacity = settings.active_opacity().0;
        self.source = settings.active;

        self.ensure_palette(settings);
        self.sync_ui();
    }

    pub fn toggle_mode(&mut self) -> ColorScheme {
        self.mode = match self.mode {
            ColorScheme::Dark => ColorScheme::Light,
            ColorScheme::Light => ColorScheme::Dark,
        };
        self.sync_ui();
        self.mode
    }

    pub fn toggle_source(&mut self, settings: &ThemeSettings) -> ThemeSource {
        self.source = match self.source {
            ThemeSource::Wallpaper => ThemeSource::SeedColor,
            ThemeSource::SeedColor => ThemeSource::PreConfigured,
            ThemeSource::PreConfigured => ThemeSource::Wallpaper,
        };
        self.opacity = settings.active_opacity().0;
        self.ensure_palette(settings);
        self.sync_ui();
        self.source
    }

    pub fn sync_ui(&self) {
        let mut ui_theme = ui::service::Theme::write();
        ui_theme.palette = self.active_palette();
        ui_theme.opacity = self.opacity;
    }
}

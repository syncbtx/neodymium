use amane::{LayerWindow, Monitor, Service};
use configuration::{
    NEODYMIUM_ACTIVE_WALLPAPER, NEODYMIUM_USER_SETTINGS_PATH, SERPANTINUM_WALLPAPER,
    engine::SettingsEngine, resolve::Resolve, settings::AppSettings,
};
use std::path::Path;

fn resolve_active_wallpaper() -> Option<&'static Path> {
    if NEODYMIUM_ACTIVE_WALLPAPER.exists() {
        Some(NEODYMIUM_ACTIVE_WALLPAPER.as_path())
    } else if SERPANTINUM_WALLPAPER.exists() {
        Some(SERPANTINUM_WALLPAPER.as_path())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ShellState {
    pub settings: AppSettings,
}

impl amane::Service for ShellState {
    fn new() -> Self {
        let _ = SettingsEngine::init();
        let settings = SettingsEngine::load();
        let wp_path = resolve_active_wallpaper();
        ui::services::Theme::write().apply(&settings.theme, wp_path);
        Self { settings }
    }

    fn listen() {
        let path = NEODYMIUM_USER_SETTINGS_PATH.to_string_lossy().to_string();
        eprintln!("watching {path}");

        for _ in amane::watch_file(&path) {
            match SettingsEngine::reload() {
                Ok(settings) => {
                    let wp_path = resolve_active_wallpaper();
                    ui::services::Theme::write().apply(&settings.theme, wp_path);
                    ShellState::write().settings = settings;
                    println!("reload successful");
                }
                Err(e) => eprintln!("reload failed: {e}"),
            }
        }
    }
}

fn top(monitor: &Monitor) -> LayerWindow {
    ui::bar::view(&ShellState::read().settings.bars.top.resolve(), monitor)
}

fn right(monitor: &Monitor) -> LayerWindow {
    ui::bar::view(&ShellState::read().settings.bars.right.resolve(), monitor)
}

fn bottom(monitor: &Monitor) -> LayerWindow {
    ui::bar::view(&ShellState::read().settings.bars.bottom.resolve(), monitor)
}

fn left(monitor: &Monitor) -> LayerWindow {
    ui::bar::view(&ShellState::read().settings.bars.left.resolve(), monitor)
}

fn main() -> Result<(), String> {
    let _ = SettingsEngine::init();

    let (font, scc, factor) = {
        let state = ShellState::read();
        let font = state.settings.general.font.path_or_name.clone();
        let scc = state.settings.bars.resolve().screen_corner_claimer;
        let factor = state.settings.general.scale.clamp(0.4, 2.0);
        (font, scc, factor)
    };

    let mut app = amane::App::new().font(&font).scale_factor(factor);

    for view in ui::bars::ordered(top, right, bottom, left, scc) {
        app = app.window_per_monitor(view);
    }

    app.run();

    Ok(())
}

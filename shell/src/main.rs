use amane::{LayerWindow, Monitor, Service};
use configuration::{
    NEODYMIUM_USER_SETTINGS_PATH, engine::SettingsEngine, resolve::Resolve, settings::AppSettings,
};

#[derive(Debug)]
pub struct ShellState {
    pub settings: AppSettings,
}

impl amane::Service for ShellState {
    fn new() -> Self {
        let _ = SettingsEngine::init();
        let settings = SettingsEngine::load();
        Self { settings }
    }

    fn listen() {
        let path = NEODYMIUM_USER_SETTINGS_PATH.to_string_lossy().to_string();
        eprintln!("watching {path}");

        for _ in amane::watch_file(&path) {
            match SettingsEngine::reload() {
                Ok(settings) => {
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

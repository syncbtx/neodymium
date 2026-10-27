use amane::{LayerWindow, Monitor, Service};
use configuration::{
    NEODYMIUM_USER_SETTINGS_PATH, engine::SettingsEngine, resolve::Resolve,
    settings::GeneralSettings,
};
use model::bar::Bars;

#[derive(Debug)]
pub struct ShellState {
    pub bars: Bars,
    pub general: GeneralSettings,
}

impl amane::Service for ShellState {
    fn new() -> Self {
        let _ = SettingsEngine::init();
        let settings = SettingsEngine::load();
        // println!("{:#?}", settings);
        let mut state = Self {
            bars: settings.bars.resolve(),
            general: settings.general,
        };
        state.update();
        state
    }

    fn listen() {
        let path = NEODYMIUM_USER_SETTINGS_PATH.to_string_lossy().to_string();
        eprintln!("watching {path}");

        for _ in amane::watch_file(&path) {
            match SettingsEngine::reload() {
                Ok(settings) => {
                    let mut state = ShellState::write();
                    state.bars = settings.bars.resolve();
                    state.general = settings.general;
                }
                Err(e) => {
                    eprintln!("reload failed");
                    eprintln!("{}", e)
                }
            }
        }
    }
}

fn top_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.bars.top, monitor)
}

fn right_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.bars.right, monitor)
}

fn bottom_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.bars.bottom, monitor)
}

fn left_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.bars.left, monitor)
}

fn main() -> Result<(), String> {
    let _ = SettingsEngine::init();
    let settings = SettingsEngine::load();
    let general = settings.general;
    let _ = settings.bars;

    amane::App::new()
        .font(&general.font.path_or_name)
        .window_per_monitor(top_bar_view)
        .window_per_monitor(right_bar_view)
        .window_per_monitor(bottom_bar_view)
        .window_per_monitor(left_bar_view)
        .scale_factor(general.scale.clamp(0.4, 2.0))
        .run();

    Ok(())
}

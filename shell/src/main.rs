use amane::{App, LayerWindow, Monitor, Service};
use configuration::{
    NEODYMIUM_USER_SETTINGS_PATH, engine::SettingsEngine, resolve::Resolve,
    settings::GeneralSettings,
};
use model::bar::Bars;
use ui::bar::bar_view;

#[derive(Debug)]
pub struct ShellState {
    pub bars: Bars,
    pub general: GeneralSettings,
}

impl amane::Service for ShellState {
    fn new() -> Self {
        let _ = SettingsEngine::init();
        let settings = SettingsEngine::load();
        println!("{:#?}", settings);
        Self {
            bars: settings.bars.resolve(),
            general: settings.general,
        }
    }

    fn listen() {
        let changes = amane::watch_file(&NEODYMIUM_USER_SETTINGS_PATH.to_string_lossy());
        changes.for_each(|_| {
            if let Some(settings) = SettingsEngine::reload() {
                let mut state = Self::write();
                state.bars = settings.bars.resolve();
                state.general = settings.general;
            }
        });
    }
}

fn top_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    bar_view(&state.bars.top, monitor)
}

fn right_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    bar_view(&state.bars.right, monitor)
}

fn bottom_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    bar_view(&state.bars.bottom, monitor)
}

fn left_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    bar_view(&state.bars.left, monitor)
}

fn main() -> Result<(), String> {
    let state = ShellState::read();
    // println!("{:#?}", state);
    App::new()
        .font(&state.general.font.path_or_name)
        .window_per_monitor(top_bar_view)
        .window_per_monitor(right_bar_view)
        .window_per_monitor(bottom_bar_view)
        .window_per_monitor(left_bar_view)
        .run();

    Ok(())
}

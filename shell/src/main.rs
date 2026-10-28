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
        // println!("{:#?}", settings);
        let mut state = Self { settings };
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
                    state.settings = settings;
                    println!("reload successful")
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
    ui::bar::view(&state.settings.bars.top.resolve(), monitor)
}

fn right_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.settings.bars.right.resolve(), monitor)
}

fn bottom_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.settings.bars.bottom.resolve(), monitor)
}

fn left_bar_view(monitor: &Monitor) -> LayerWindow {
    let state = ShellState::read();
    ui::bar::view(&state.settings.bars.left.resolve(), monitor)
}

fn main() -> Result<(), String> {
    let _ = SettingsEngine::init();

    let general = {
        let state = ShellState::read();
        state.settings.general.clone()
    };

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

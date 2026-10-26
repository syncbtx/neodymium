use amane::{App, Service};
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

fn main() -> Result<(), String> {
    let shell = ShellState::new();

    println!("{:#?}", shell);

    ui::bar::init(shell.bars);

    let views = ui::bar::views();

    if views.is_empty() {
        eprintln!("No bars enabled in configuration.");
        return Ok(());
    }

    let mut app = App::new().font(&shell.general.font.path_or_name);
    for view in views {
        app = app.window_per_monitor(view);
    }

    app.run();
    Ok(())
}

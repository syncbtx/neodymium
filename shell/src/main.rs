use std::{ffi::OsStr, os::unix::process::CommandExt, process::Command, time::Duration};

use amane::App;
use configuration::engine::ConfigEngine;
use inotify::{Inotify, WatchMask};
use model::bar::Bars;

fn main() -> Result<(), String> {
    let engine = ConfigEngine::init()?;
    let bars = Bars::from_config(&engine.core_config.bars);
    let general = &engine.core_config.general;

    // Must happen BEFORE the watcher starts: it writes into the watched
    // directory and would otherwise trigger a reload loop.
    if let Err(e) = ConfigEngine::export_schemas() {
        eprintln!("failed to export schemas: {e}");
    }

    ui::bar::init(bars);
    let views = ui::bar::views();

    if views.is_empty() {
        eprintln!("No bars enabled in configuration.");
        return Ok(());
    }

    let mut app = App::new().font(&general.font.path_or_name);
    for view in views {
        app = app.window_per_monitor(view);
    }

    spawn_config_watcher();

    app.run();
    Ok(())
}

/// Only real JSON config files trigger a reload. Generated schemas,
/// dotfiles, and editor temp/swap files are ignored.
fn is_config_file(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    name.ends_with(".json") && !name.ends_with(".schema.json") && !name.starts_with('.')
}

fn spawn_config_watcher() {
    std::thread::spawn(|| {
        if let Err(e) = watch_loop() {
            eprintln!("config watcher stopped: {e}");
        }
    });
}

fn watch_loop() -> std::io::Result<()> {
    let mut ino = Inotify::init()?;
    ino.watches().add(
        configuration::resolution::preferences_dir(),
        // Editors usually write a temp file and rename it over the target,
        // which shows up as MOVED_TO/CREATE rather than MODIFY.
        WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO | WatchMask::CREATE,
    )?;

    let mut buf = [0u8; 4096];
    loop {
        let hit = ino
            .read_events_blocking(&mut buf)?
            .any(|e| e.name.map_or(false, is_config_file));
        if !hit {
            continue;
        }

        // Debounce: one save often fires several events.
        std::thread::sleep(Duration::from_millis(75));

        // Validate first. A typo keeps the running shell alive.
        match ConfigEngine::init() {
            Ok(_) => restart(),
            Err(e) => eprintln!("config reload failed, keeping current config: {e}"),
        }
    }
}

/// Replace the current process with a fresh copy of itself.
/// On success this never returns.
fn restart() {
    match std::env::current_exe() {
        Ok(exe) => {
            let err = Command::new(exe).args(std::env::args_os().skip(1)).exec();
            eprintln!("restart failed: {err}");
        }
        Err(e) => eprintln!("cannot locate own executable: {e}"),
    }
}

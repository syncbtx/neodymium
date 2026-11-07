use amane::{Apps, DesktopApp, Service};
use std::path::{Path, PathBuf};

pub struct FocusedWindow {
    pub title: String,
    pub id: String,
    pub icon: Option<PathBuf>,
}

impl FocusedWindow {
    fn query() -> (String, String) {
        let out = amane::output("niri msg --json focused-window");
        let v: serde_json::Value = serde_json::from_str(&out).unwrap_or_default();
        let field = |key: &str| {
            v.get(key)
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string()
        };
        (field("title"), field("app_id"))
    }

    fn exec_name(exec: &str) -> &str {
        let first = exec.split_whitespace().next().unwrap_or("");
        first.rsplit('/').next().unwrap_or(first)
    }

    fn find_app<'a>(apps: &'a [DesktopApp], id: &str) -> Option<&'a DesktopApp> {
        if id.is_empty() {
            return None;
        }

        let tail = id.rsplit_once('.').map(|(_, t)| t);
        let head = id.split_once('-').map(|(h, _)| h);

        let passes: [&dyn Fn(&DesktopApp, &str) -> bool; 3] = [
            &|a, k| a.icon().is_some_and(|i| i.eq_ignore_ascii_case(k)),
            &|a, k| a.name().eq_ignore_ascii_case(k),
            &|a, k| Self::exec_name(a.exec()).eq_ignore_ascii_case(k),
        ];

        let keys = [Some(id), tail, head];

        for key in keys.into_iter().flatten().filter(|k| !k.is_empty()) {
            for pass in passes {
                if let Some(app) = apps.iter().find(|a| pass(a, key)) {
                    return Some(app);
                }
            }
        }
        None
    }

    fn resolve(id: &str) -> Option<PathBuf> {
        let apps = Apps::read();
        Self::find_app(apps.list(), id)?
            .icon_path()
            .map(Path::to_path_buf)
    }
}

impl amane::Service for FocusedWindow {
    fn new() -> Self {
        let (title, id) = Self::query();
        let icon = Self::resolve(&id);
        Self { title, id, icon }
    }

    fn listen() {
        for _ in amane::lines("niri msg event-stream") {
            let (title, id) = Self::query();

            let (title_changed, id_changed) = {
                let current = Self::read();
                (current.title != title, current.id != id)
            };

            if !title_changed && !id_changed {
                continue;
            }

            let icon = if id_changed {
                Some(Self::resolve(&id))
            } else {
                None
            };

            let mut current = Self::write();
            current.title = title;
            if let Some(icon) = icon {
                current.icon = icon;
            }
            current.id = id;
        }
    }
}

use std::{env, path::PathBuf, sync::LazyLock};

use serde_json::Value;

const HOME_UNSET_ERR_MESSAGE: &str = "CRITICAL: unable to resolve home directory, $HOME is unset";

pub static NEODYMIUM_CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| config_dir());
pub static NEODYMIUM_STATE_DIR: LazyLock<PathBuf> = LazyLock::new(|| state_dir());
pub static NEODYMIUM_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| cache_dir());

pub static NEODYMIUM_USER_SETTINGS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| NEODYMIUM_CONFIG_DIR.join("settings.jsonc"));

pub static NEODYMIUM_DEFAULT_SETTINGS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| NEODYMIUM_STATE_DIR.join("default.jsonc"));

pub static NEODYMIUM_SCHEMA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| NEODYMIUM_STATE_DIR.join("schema.json"));

pub fn config_dir() -> PathBuf {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(path) => PathBuf::from(path).join("neodymium"),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".config/neodymium"),
    }
}

pub fn state_dir() -> PathBuf {
    match env::var_os("XDG_STATE_HOME") {
        Some(path) => PathBuf::from(path).join("neodymium"),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".local/state/neodymium"),
    }
}

pub fn cache_dir() -> PathBuf {
    match env::var_os("XDG_CACHE_HOME") {
        Some(path) => PathBuf::from(path).join("neodymium"),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".cache/neodymium"),
    }
}

pub fn parse_domain(text: &str) -> Result<Value, String> {
    let mut value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if let Value::Object(map) = &mut value {
        map.remove("$schema");
    }
    Ok(value)
}

// pub fn resolve_domain<D: ConfigDomain>() -> Result<D, String> {
//     let mut base = serde_json::to_value(D::default()).map_err(|e| e.to_string())?;

//     let default_path = default_file(D::SCHEMA_NAME);
//     if default_path.exists() {
//         if let Ok(text) = std::fs::read_to_string(&default_path) {
//             if let Ok(val) = parse_domain(&text) {
//                 merge(&mut base, &val);
//             }
//         }
//     }

//     let user_path = user_file(D::SCHEMA_NAME);
//     if user_path.exists() {
//         if let Ok(text) = std::fs::read_to_string(user_path) {
//             if let Ok(val) = parse_domain(&text) {
//                 merge(&mut base, &val);
//             }
//         }
//     }

//     Ok(serde_json::from_value(base).unwrap_or_default())
// }

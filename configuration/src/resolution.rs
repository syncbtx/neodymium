use std::{env, path::PathBuf, sync::LazyLock};

use serde_json::{Map, Value};

use crate::settings::ConfigDomain;

const HOME_UNSET_ERR_MESSAGE: &str = "CRITICAL: unable to resolve home directory, $HOME is unset";

pub static NEODYMIUM_CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| config_dir());
pub static NEODYMIUM_STATE_DIR: LazyLock<PathBuf> = LazyLock::new(|| state_dir());
pub static NEODYMIUM_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| cache_dir());

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

pub fn schema_dir() -> PathBuf {
    NEODYMIUM_CONFIG_DIR.join("schema")
}

pub fn fallback_dir() -> PathBuf {
    NEODYMIUM_CONFIG_DIR.join("fallback")
}

pub fn preferences_dir() -> PathBuf {
    NEODYMIUM_CONFIG_DIR.join("preferences")
}

pub fn schema_file(domain: &str) -> PathBuf {
    schema_dir().join(format!("{}.schema.json", domain))
}

pub fn fallback_file(domain: &str) -> PathBuf {
    fallback_dir().join(format!("{}.json", domain))
}

pub fn preferences_file(domain: &str) -> PathBuf {
    preferences_dir().join(format!("{}.json", domain))
}

pub fn parse_domain(text: &str) -> Result<Value, String> {
    let mut value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if let Value::Object(map) = &mut value {
        map.remove("$schema");
    }
    Ok(value)
}

pub fn merge(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Object(b), Value::Object(o)) => {
            for (k, v) in o {
                match b.get_mut(k) {
                    Some(slot) => merge(slot, v),
                    None => {
                        b.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        (slot, value) => *slot = value.clone(),
    }
}

pub fn diff(base: &Value, target: &Value) -> Option<Value> {
    if base == target {
        return None;
    };

    match (base, target) {
        (Value::Object(b), Value::Object(t)) => {
            let mut out = Map::new();
            for (k, tv) in t {
                match b.get(k) {
                    Some(bv) => {
                        if let Some(d) = diff(bv, tv) {
                            out.insert(k.clone(), d);
                        }
                    }
                    None => {
                        out.insert(k.clone(), tv.clone());
                    }
                }
            }
            if out.is_empty() {
                return None;
            } else {
                return Some(Value::Object(out));
            }
        }

        _ => Some(target.clone()),
    }
}

pub fn resolve_domain<D: ConfigDomain>() -> Result<D, String> {
    let mut base = serde_json::to_value(D::default()).map_err(|e| e.to_string())?;

    let fallback_path = fallback_file(D::SCHEMA_NAME);
    if fallback_path.exists() {
        if let Ok(text) = std::fs::read_to_string(&fallback_path) {
            if let Ok(val) = parse_domain(&text) {
                merge(&mut base, &val);
            }
        }
    }

    let preferences_path = preferences_file(D::SCHEMA_NAME);
    if preferences_path.exists() {
        if let Ok(text) = std::fs::read_to_string(preferences_path) {
            if let Ok(val) = parse_domain(&text) {
                merge(&mut base, &val);
            }
        }
    }

    Ok(serde_json::from_value(base).unwrap_or_default())
}

use std::{env, path::PathBuf, sync::LazyLock};

pub mod domain;
pub mod resolution;

const HOME_UNSET_ERR_MESSAGE: &str = "CRITICAL: unable to resolve home directory, $HOME is unset";

pub static NEODYMIUM_CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| config_dir());
pub static NEODYMIUM_STATE_DIR: LazyLock<PathBuf> = LazyLock::new(|| state_dir());
pub static NEODYMIUM_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| cache_dir());

pub fn config_dir() -> PathBuf {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(path) => path.into(),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".config/neodymium"),
    }
}

pub fn state_dir() -> PathBuf {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(path) => path.into(),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".local/state/neodymium"),
    }
}

pub fn cache_dir() -> PathBuf {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(path) => path.into(),
        None => env::home_dir()
            .expect(HOME_UNSET_ERR_MESSAGE)
            .join(".cache/neodymium"),
    }
}

use schemars::{JsonSchema, schema_for};

pub trait ConfigSchema: JsonSchema {
    const SCHEMA_NAME: &'static str;

    fn schema_json() -> String {
        let mut schema = serde_json::to_value(schema_for!(Self)).expect("schema generation failed");

        if let Some(props) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            props.insert(
                "$schema".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Path or URL of this schema, for editors. Ignored by Neodymium."
                }),
            );
        }

        serde_json::to_string_pretty(&schema).expect("schema serialization failed")
    }

    fn save_schema() -> std::io::Result<()> {
        let dir = NEODYMIUM_CONFIG_DIR.join("schema");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.schema.json", Self::SCHEMA_NAME));
        std::fs::write(path, Self::schema_json())
    }
}

pub trait ConfigValidate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

pub enum ConfigValidationError {
    OutOfRange,
}

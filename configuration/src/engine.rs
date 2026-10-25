use std::fs;

use schemars::schema_for;

use crate::{
    domain::{ConfigDomain, bars::BarsConfig, core::CoreConfig, general::GeneralConfig},
    resolution::{self, fallback_dir, preferences_dir, preferences_file, schema_dir, schema_file},
};

#[derive(Debug)]
pub struct Settings {
    pub core: CoreConfig,
    pub edited: CoreConfig,
}

#[derive(Debug)]
pub struct ConfigEngine {
    pub core_config: CoreConfig,
}

impl Default for ConfigEngine {
    fn default() -> Self {
        Self {
            core_config: Default::default(),
        }
    }
}

impl ConfigEngine {
    pub fn init() -> Result<Self, String> {
        std::fs::create_dir_all(schema_dir()).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(fallback_dir()).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(preferences_dir()).map_err(|e| e.to_string())?;

        let core_config = CoreConfig::aggregate()?;
        Ok(Self { core_config })
    }

    pub fn export_schemas() -> Result<(), String> {
        Self::export_schema::<BarsConfig>()?;
        Self::export_schema::<GeneralConfig>()?;
        println!("\x1b[32mconfig schemas exported\x1b[0m");
        Ok(())
    }

    fn export_schema<D: ConfigDomain>() -> Result<(), String> {
        let mut schema = serde_json::to_value(schema_for!(D)).map_err(|e| e.to_string())?;
        if let Some(props) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            props.insert(
                "$schema".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Path or URL of this schema, for editors. Ignored by Neodymium."
                }),
            );
        };
        let pretty = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        std::fs::write(schema_file(D::SCHEMA_NAME), pretty).map_err(|e| e.to_string())
    }

    pub fn save_changes<D: ConfigDomain>(&self, updated: D) -> Result<(), String> {
        let base = serde_json::to_value(D::default()).map_err(|e| e.to_string())?;
        let target = serde_json::to_value(updated).map_err(|e| e.to_string())?;

        let sparse = resolution::diff(&base, &target)
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

        let pretty = serde_json::to_string_pretty(&sparse).map_err(|e| e.to_string())?;
        let path = preferences_file(D::SCHEMA_NAME);
        fs::write(path, pretty).map_err(|e| e.to_string())
    }
}

use std::collections::BTreeSet;
use fontconfig::{Fontconfig, ObjectSet, Pattern, list_fonts, FC_FAMILY};
use schemars::schema_for;
use serde_json::{Map, Value};

use crate::{
    NEODYMIUM_CACHE_DIR, NEODYMIUM_CONFIG_DIR, NEODYMIUM_DEFAULT_SETTINGS_PATH,
    NEODYMIUM_SCHEMA_PATH, NEODYMIUM_STATE_DIR, NEODYMIUM_USER_SETTINGS_PATH,
    settings::AppSettings, validate::Sanitize,
};

pub struct SettingsEngine;

impl SettingsEngine {
    pub fn init() -> Result<(), String> {
        std::fs::create_dir_all(&*NEODYMIUM_CONFIG_DIR).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&*NEODYMIUM_STATE_DIR).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&*NEODYMIUM_CACHE_DIR).map_err(|e| e.to_string())?;

        Self::export_schema()
    }

    pub fn list_system_fonts() -> Vec<String> {
        let mut families = BTreeSet::new();

        if let Some(fc) = Fontconfig::new() {
            if let Ok(pattern) = Pattern::new(&fc) {
                let mut os = ObjectSet::new(&fc).ok();
                if let Some(ref mut os) = os {
                    let _ = os.add(FC_FAMILY);
                }
                if let Ok(fontset) = list_fonts(&pattern, os.as_ref()) {
                    for pat in fontset.iter() {
                        if let Ok(family) = pat.get_string(FC_FAMILY) {
                            let trimmed = family.trim();
                            if !trimmed.is_empty() {
                                families.insert(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }

        families.into_iter().collect()
    }

    pub fn export_schema() -> Result<(), String> {
        let mut schema =
            serde_json::to_value(schema_for!(AppSettings)).map_err(|e| e.to_string())?;

        if let Some(props) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            props.insert(
                "$schema".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Path or URL to the schema file for editor validation."
                }),
            );
        }

        let fonts = Self::list_system_fonts();
        if !fonts.is_empty() {
            if let Some(font_prop) = schema.pointer_mut("/$defs/Font/properties/pathOrName") {
                *font_prop = serde_json::json!({
                    "anyOf": [
                        {
                            "type": "string",
                            "enum": fonts
                        },
                        {
                            "type": "string",
                            "pattern": r"^(/|~|\./|sans-serif|monospace|serif).*"
                        }
                    ]
                });
            }
        }

        let pretty = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        std::fs::write(&*NEODYMIUM_SCHEMA_PATH, pretty).map_err(|e| e.to_string())
    }

    pub fn load() -> AppSettings {
        let mut base =
            serde_json::to_value(AppSettings::default()).unwrap_or(Value::Object(Map::new()));

        if NEODYMIUM_DEFAULT_SETTINGS_PATH.exists() {
            println!(
                "DEFAULT PATH: {:#?}",
                NEODYMIUM_DEFAULT_SETTINGS_PATH.as_path()
            );
            println!("DEFAULT PATH EXISTS: true");
            match std::fs::read_to_string(&*NEODYMIUM_DEFAULT_SETTINGS_PATH) {
                Ok(json) => match Self::parse_json(&json) {
                    Ok(overlay) => {
                        Self::merge(&mut base, &overlay);
                    }
                    Err(e) => println!("PARSE ERR: {:#?}", e),
                },
                Err(e) => println!("READ ERR: {:#?}", e),
            }
        } else {
            println!(
                "DEFAULT PATH DOES NOT EXIST: {:#?}",
                NEODYMIUM_DEFAULT_SETTINGS_PATH.display()
            );
        }

        if NEODYMIUM_USER_SETTINGS_PATH.exists() {
            if let Ok(json) = std::fs::read_to_string(&*NEODYMIUM_USER_SETTINGS_PATH) {
                if let Ok(overlay) = Self::parse_json(&json) {
                    Self::merge(&mut base, &overlay);
                }
            }
        }

        let mut settings: AppSettings = match serde_json::from_value(base.clone()) {
            Ok(s) => s,
            Err(e) => {
                println!("DESERIALIZE ERROR FROM BASE: {}", e);
                println!("FALLING BACK TO DEFAULTS: {}", e);
                AppSettings::default()
            }
        };
        settings.sanitize();
        settings
    }

    fn read_layer(path: &std::path::Path, base: &mut Value) -> Result<(), String> {
        if !path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if json.trim().is_empty() {
            return Err(format!("{}: empty (mid-write?)", path.display()));
        }
        let overlay = Self::parse_json(&json).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::merge(base, &overlay);
        Ok(())
    }

    pub fn reload() -> Result<AppSettings, String> {
        let mut base = serde_json::to_value(AppSettings::default()).map_err(|e| e.to_string())?;
        Self::read_layer(&NEODYMIUM_DEFAULT_SETTINGS_PATH, &mut base)?;
        Self::read_layer(&NEODYMIUM_USER_SETTINGS_PATH, &mut base)?;
        let mut settings: AppSettings = serde_json::from_value(base).map_err(|e| e.to_string())?;
        settings.sanitize();
        Ok(settings)
    }

    pub fn parse_json(json: &str) -> Result<Value, serde_json::Error> {
        let stripped = json_comments::StripComments::new(json.as_bytes());
        let mut value: Value = serde_json::from_reader(stripped)?;
        if let Value::Object(ref mut map) = value {
            map.remove("$schema");
        }
        Ok(value)
    }

    pub fn merge(base: &mut Value, overlay: &Value) {
        match (base, overlay) {
            (Value::Object(b), Value::Object(o)) => {
                for (k, v) in o {
                    match b.get_mut(k) {
                        Some(slot) => Self::merge(slot, v),
                        None => {
                            b.insert(k.clone(), v.clone());
                        }
                    }
                }
            }

            (slot, value) => *slot = value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::section::SectionSettings;

    #[test]
    fn test_export_schema() {
        SettingsEngine::init().unwrap();
        let contents = std::fs::read_to_string(&*NEODYMIUM_SCHEMA_PATH).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let enum_values = schema
            .pointer("/$defs/Font/properties/pathOrName/anyOf/0/enum")
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(!enum_values.is_empty());
    }

    #[test]
    fn test_font_fallback_hierarchy() {
        use model::{Font, bar::module::Module};
        let mut settings = AppSettings::default();
        settings.general.font.path_or_name = "GeneralFont".to_string();

        let dummy_section = |font: Option<Font>| SectionSettings {
            font,
            modules: vec![Module::Divider],
            ..Default::default()
        };

        settings.bars.top.font = None;
        settings.bars.top.sections = vec![dummy_section(None)];

        settings.bars.bottom.font = Some(Font {
            path_or_name: "BarFont".to_string(),
            ..Default::default()
        });
        settings.bars.bottom.sections = vec![
            dummy_section(None),
            dummy_section(Some(Font {
                path_or_name: "SectionFont".to_string(),
                ..Default::default()
            })),
        ];

        settings.sanitize();

        assert_eq!(
            settings.bars.top.font.as_ref().unwrap().path_or_name,
            "GeneralFont"
        );
        assert_eq!(
            settings.bars.top.sections[0]
                .font
                .as_ref()
                .unwrap()
                .path_or_name,
            "GeneralFont"
        );

        assert_eq!(
            settings.bars.bottom.font.as_ref().unwrap().path_or_name,
            "BarFont"
        );
        assert_eq!(
            settings.bars.bottom.sections[0]
                .font
                .as_ref()
                .unwrap()
                .path_or_name,
            "BarFont"
        );
        assert_eq!(
            settings.bars.bottom.sections[1]
                .font
                .as_ref()
                .unwrap()
                .path_or_name,
            "SectionFont"
        );
    }
}


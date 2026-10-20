use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::ConfigDomain;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AutohideConfig {
    pub delay_ms: u32,
    pub duration_ms: u32,
    pub trigger_zone_size: u32,
    pub handle_size: u32,
}

impl Default for AutohideConfig {
    fn default() -> Self {
        Self {
            delay_ms: 150,
            duration_ms: 300,
            trigger_zone_size: 5,
            handle_size: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct GeneralConfig {
    pub autohide: AutohideConfig,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            autohide: AutohideConfig::default(),
        }
    }
}

pub const FALLBACK_GENERAL_JSON: &str = include_str!("../../fallback/general.json");

impl ConfigDomain for GeneralConfig {
    const SCHEMA_NAME: &'static str = "general";

    const EMBEDDED_FALLBACK: &'static str = FALLBACK_GENERAL_JSON;
}

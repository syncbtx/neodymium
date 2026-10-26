use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Module {
    Spacer(u32),
    #[serde(rename_all = "camelCase")]
    Workspaces {
        #[serde(default, alias = "collape_unused", alias = "collapeUnused", alias = "collapse_unused")]
        collapse_unused: bool,
    },
    ActiveWindowTitle,
    #[serde(rename_all = "camelCase")]
    Calendar {
        #[serde(default)]
        time: bool,
        #[serde(default)]
        date: bool,
        #[serde(default)]
        compact: bool,
    },
}

impl Default for Module {
    fn default() -> Self {
        Self::ActiveWindowTitle
    }
}

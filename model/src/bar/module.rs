use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Module {
    Spacer(u32),
    Workspaces {
        collape_unused: bool,
    },
    ActiveWindowTitle,
    Calendar {
        time: bool,
        date: bool,
        compact: bool,
    },
}

impl Default for Module {
    fn default() -> Self {
        Self::ActiveWindowTitle
    }
}

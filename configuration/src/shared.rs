use colored::Colorize;
use hex_color::HexColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Edge {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Border {
    pub width: u32,
    pub color: HexColor,
}

#[derive(Debug, Copy, Clone, Serialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Alpha(pub u8);

impl Default for Alpha {
    fn default() -> Self {
        Self(255)
    }
}

impl<'de> Deserialize<'de> for Alpha {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = serde_json::Value::deserialize(deserializer)?;

        match val.as_u64() {
            Some(n) if n <= 255 => Ok(Alpha(n as u8)),
            Some(_) => {
                eprintln!(
                    "{} {} {} {}",
                    "[WARN]".yellow(),
                    "opacity is out of range".white(),
                    "0..=255".purple(),
                    "defaulting to 255".white()
                );
                Ok(Alpha::default())
            }
            None => {
                eprintln!(
                    "{} {} {} {}",
                    "[WARN]".yellow(),
                    "invalid opacity ".white(),
                    "0..=255".purple(),
                    "defaulting to 255".white()
                );
                Ok(Alpha::default())
            }
        }
    }
}

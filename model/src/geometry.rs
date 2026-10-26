use std::{borrow::Cow, fmt::Display, str::FromStr};

use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

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
pub struct Insets<T: Default> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum MainAxisSize {
    #[default]
    Auto,
    Percent(u32),
    Pixels(u32),
}

impl Display for MainAxisSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainAxisSize::Auto => write!(f, "auto"),
            MainAxisSize::Percent(pct) => write!(f, "{pct}%"),
            MainAxisSize::Pixels(px) => write!(f, "{px}px"),
        }
    }
}

impl FromStr for MainAxisSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Self::Auto);
        };

        if let Some(pct) = s.strip_suffix("%") {
            return pct
                .parse::<u32>()
                .map(Self::Percent)
                .map_err(|_| format!("invalid percentage: {s:?}"));
        };

        if let Some(px) = s.strip_suffix("px") {
            return px
                .parse::<u32>()
                .map(Self::Pixels)
                .map_err(|_| format!("invalid pixels: {s:?}"));
        };

        Err(format!(
            "expected \"auto\" | percentage like \"75%\" | pixels like \"1024px\" but found {s:?}"
        ))
    }
}

impl Serialize for MainAxisSize {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for MainAxisSize {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = Cow::<str>::deserialize(d)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl JsonSchema for MainAxisSize {
    fn schema_name() -> Cow<'static, str> {
        "majorAxisSize".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "pattern": "^(auto|[0-9]+(\\.[0-9]+)?%|[0-9]+px)$",
            "description": "Length of the bar along its edge: \"auto\" | \"50%\" | \"1920px\"."
        })
    }
}

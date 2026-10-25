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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum MajorAxisSize {
    #[default]
    Auto,
    Percent(u32),
    Pixel(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Insets<T: Default> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

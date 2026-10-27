use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Layer {
    #[default]
    Top,
    Background,
    Bottom,
    Overlay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ScreenCornerClaimer {
    #[default]
    HorizontalBars,
    VerticalBars,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Justify {
    Start,

    Center,

    End,

    // first and last child touch the edges, the rest of the space goes between them
    #[default]
    SpaceBetween,

    // every child gets the same space on both sides, so the edges get half a gap
    SpaceAround,

    // the edges and every gap between children get the same space
    SpaceEvenly,
}

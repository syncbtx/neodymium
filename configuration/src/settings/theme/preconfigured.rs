use model::ModelTheme;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{constants, resolve::Resolve};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PreConfiguredTheme {
    #[default]
    Catppuccin,
    Gruvbox,
    Nord,
    TokyoNight,
    Solarised,
    Monokai,
}

impl Resolve for PreConfiguredTheme {
    type Output = ModelTheme;

    fn resolve(&self) -> Self::Output {
        match self {
            PreConfiguredTheme::Catppuccin => constants::themes::CATPPUCCIN,
            PreConfiguredTheme::Gruvbox => constants::themes::GRUVBOX,
            PreConfiguredTheme::Nord => constants::themes::NORD,
            PreConfiguredTheme::TokyoNight => constants::themes::TOKYONIGHT,
            PreConfiguredTheme::Solarised => constants::themes::SOLARISED,
            PreConfiguredTheme::Monokai => constants::themes::MONOKAI,
        }
    }
}

use model::Font;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{resolve::Resolve, validate::Sanitize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Autohide {
    pub delay_ms: u32,
    pub duration_ms: u32,
    pub trigger_zone_size: u32,
    pub handle_size: u32,
}

impl Default for Autohide {
    fn default() -> Self {
        Self {
            delay_ms: 150,
            duration_ms: 300,
            trigger_zone_size: 5,
            handle_size: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct GeneralSettings {
    pub autohide: Autohide,
    pub scale: f32,
    pub font: Font,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            autohide: Autohide::default(),
            scale: 1.0,
            font: Font::default(),
        }
    }
}

impl Resolve for GeneralSettings {
    type Output = ();

    fn resolve(&self) -> Self::Output {
        ()
    }
}

impl Sanitize for GeneralSettings {
    fn sanitize(&mut self) {
        self.scale = self.scale.clamp(0.25, 4.0);

        self.autohide.delay_ms = self.autohide.delay_ms.min(5000);
        self.autohide.duration_ms = self.autohide.duration_ms.clamp(50, 2000);
        self.autohide.trigger_zone_size = self.autohide.trigger_zone_size.clamp(1, 64);
    }
}

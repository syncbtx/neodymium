pub mod bar;
pub mod bars;
pub mod general;
pub mod section;
pub mod theme;

pub use bars::*;
pub use general::*;
pub use theme::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::validate::Sanitize;

pub trait ConfigDomain: JsonSchema + Serialize + DeserializeOwned + Default {
    const SCHEMA_NAME: &'static str;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AppSettings {
    pub bars: BarsSettings,
    pub general: GeneralSettings,
    pub theme: ThemeSettings,
}

impl Sanitize for AppSettings {
    fn sanitize(&mut self) {
        self.general.sanitize();
        self.theme.sanitize();
        self.bars.sanitize();

        for bar in self.bars.iter_mut() {
            if bar.font.is_none() {
                bar.font = Some(self.general.font.clone());
            }

            for section in &mut bar.sections {
                if section.font.is_none() {
                    section.font = bar.font.clone();
                }

                if section.padding.is_none() {
                    section.padding = Some(bar.section_padding);
                }
            }
        }
    }
}

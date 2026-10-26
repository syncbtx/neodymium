pub mod bars;
pub mod general;

pub use bars::*;
pub use general::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::validate::Sanitize;

pub trait ConfigDomain: JsonSchema + Serialize + DeserializeOwned + Default {
    const SCHEMA_NAME: &'static str;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct AppSettings {
    pub bars: BarsSettings,
    pub general: GeneralSettings,
}

impl Sanitize for AppSettings {
    fn sanitize(&mut self) {
        self.general.sanitize();

        self.bars.sanitize();

        for bar in self.bars.iter_mut() {
            if bar.font.is_none() {
                bar.font = Some(self.general.font.clone());
            }
        }
    }
}

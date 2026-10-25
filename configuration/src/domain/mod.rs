pub mod bars;
pub mod core;
pub mod general;

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

pub trait ConfigDomain: JsonSchema + Serialize + DeserializeOwned + Default {
    const SCHEMA_NAME: &'static str;
}

pub mod bars;
pub mod core;
pub mod general;

use schemars::{JsonSchema, schema_for};
use serde::{Serialize, de::DeserializeOwned};

use crate::resolution::NEODYMIUM_CONFIG_DIR;

pub trait ConfigDomain: JsonSchema + Serialize + DeserializeOwned + Default {
    const SCHEMA_NAME: &'static str;

    fn schema_json() -> String {
        let mut schema = serde_json::to_value(schema_for!(Self)).expect("schema generation failed");

        if let Some(props) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            props.insert(
                "$schema".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Path or URL of this schema, for editors. Ignored by Neodymium."
                }),
            );
        }

        serde_json::to_string_pretty(&schema).expect("schema serialization failed")
    }

    fn save_schema() -> std::io::Result<()> {
        let dir = NEODYMIUM_CONFIG_DIR.join("schema");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.schema.json", Self::SCHEMA_NAME));
        std::fs::write(path, Self::schema_json())
    }
}

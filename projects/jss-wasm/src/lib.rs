#![doc = include_str!("../Readme.md")]

use serde_json::Value;
use serde_wasm_bindgen::from_value;
use std::str::FromStr;
use wasm_bindgen::{prelude::*, JsObject};

use jss_core::{JssError, JssSchema};

#[wasm_bindgen]
pub struct Jss {
    internal: JssSchema,
}

#[wasm_bindgen]
impl Jss {
    /// Create a new JSS instance.
    #[wasm_bindgen(constructor)]
    pub fn from_string(jss: &str) -> Result<Self, JsError> {
        let jss = JssSchema::from_str(jss)?;
        Ok(Self { internal: jss })
    }

    /// Get the JSS instance.
    pub fn from_json_schema(object: JsValue) -> Result<Self, JsError> {
        let jss = JssSchema::try_from(object)?;
        Ok(Self { internal: jss })
    }

    /// Get the JSS instance.
    pub fn validate(&self, object: JsValue) -> Result<Vec<JsError>, JsError> {
        let value: Value = from_value(object)?;
        Ok(self.internal.validate(&value))
    }

    /// Get the JSS instance.
    pub fn is_valid(&self, object: JsValue) -> Result<bool, JsError> {
        let value: Value = from_value(object)?;
        Ok(self.internal.is_valid(&value))
    }

    /// Get random schema
    pub fn mock(&self) -> JsValue {
        unimplemented!("random")
    }
}

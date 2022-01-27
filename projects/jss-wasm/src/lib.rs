#![doc = include_str ! ("../Readme.md")]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Jss {
    internal: String,
}

#[wasm_bindgen]
impl Jss {
    /// Create a new JSS instance.
    #[wasm_bindgen(constructor)]
    pub fn from_string(jss: &str) -> Self {
        Self {
            internal: jss.to_string(),
        }
    }

    /// Get the JSS instance.
    pub fn from_json_schema(schema: &str) -> Self {
        Self {
            internal: schema.to_string(),
        }
    }

    /// Get the JSS instance.
    pub fn validate(&self, json: &JsValue) -> Result<String, JsValue> {
        Ok(self.internal.clone())
    }

    /// Get random schema
    pub fn random(&self) -> JsValue {
        JsValue::from_str("test")
    }
}

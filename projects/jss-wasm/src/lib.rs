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

// #[wasm_bindgen]
impl Jss {
    /// Create a new JSS instance.
    // #[wasm_bindgen(constructor)]
    pub fn from_string(jss: &str) -> Result<Self, JsError> {
        match JssSchema::from_str(jss) {
            Ok(o) => Ok(Self { internal: o }),
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    /// Get the JSS instance.
    pub fn from_json_schema(object: JsValue) -> Self {
        let value: Value = match from_value(object) {
            Ok(o) => Ok(o),
            Err(e) => Err(JsError::new(&e.to_string())),
        }?;
        let jss = match JssSchema::try_from(value) {
            Ok(o) => Ok(o),
            Err(e) => Err(JsError::new(&e.to_string())),
        }?;
        Self { internal: jss }
    }

    /// Get the JSS instance.
    pub fn validate(&self, json: &JsValue) -> Result<String, JsError> {
        let value: Value = match from_value(object) {
            Ok(o) => Ok(o),
            Err(e) => Err(JsError::new(&e.to_string())),
        }?;
        self.internal.validate(&value)
    }

    /// Get random schema
    pub fn random(&self) -> JsValue {
        JsValue::from_str("test")
    }
}

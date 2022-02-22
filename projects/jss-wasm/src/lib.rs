#![doc = include_str!("../Readme.md")]

use std::str::FromStr;
use wasm_bindgen::prelude::*;

use jss_core::JssSchema;

#[wasm_bindgen]
pub struct Jss {
    internal: JssSchema,
}

#[wasm_bindgen]
impl Jss {
    /// Create a new JSS instance.
    #[wasm_bindgen(constructor)]
    pub fn from_string(jss: &str) -> Result<Jss, JsError> {
        Ok(Self { internal: JssSchema::from_str(jss)? })
    }

    /// Get the JSS instance.
    pub fn from_json_schema(object: JsValue) -> Result<Jss, JsError> {
        Ok(Self { internal: JssSchema::try_from(object)? })
    }

    /// Get the JSS instance.
    pub fn validate(&self, object: JsValue) -> Vec<JsValue> {
        self.internal.validate_js(object)
    }

    /// Get the JSS instance.
    pub fn is_valid(&self, object: JsValue) -> bool {
        self.internal.validate_js(object).is_empty()
    }

    /// Get random schema
    pub fn mock(&self) -> JsValue {
        unimplemented!("random")
    }
}

#[test]
fn ready() {
    println!("it works!")
}

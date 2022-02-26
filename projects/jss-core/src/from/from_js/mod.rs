use crate::JssSchema;
use json_value::JsonValue;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{JsError, JsValue};

impl TryFrom<JsValue> for JssSchema {
    type Error = JsError;

    fn try_from(js: JsValue) -> Result<Self, Self::Error> {
        let value = match from_value::<JsonValue>(js) {
            Ok(o) => o,
            Err(e) => return Err(JsError::from(e)),
        };
        match JssSchema::try_from(value) {
            Ok(o) => Ok(o),
            Err(e) => Err(JsError::from(e)),
        }
    }
}

impl JssSchema {
    pub fn validate_js(&self, object: JsValue) -> Vec<JsValue> {
        let value = match from_value::<JsonValue>(object) {
            Ok(o) => o,
            Err(e) => return vec![JsValue::from(JsError::from(e))],
        };
        return self.validate(&value).into_iter().map(|e| JsValue::from(e.to_string())).collect();
    }
    pub fn to_json_schema(&self) -> JsValue {
        let json = JsonValue::from(self);
        to_value(&json).unwrap()
    }
}

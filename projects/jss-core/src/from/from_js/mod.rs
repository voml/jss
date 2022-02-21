use crate::{JssError, JssSchema};
use json_value::JsonValue;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{JsError, JsValue};

impl TryFrom<JsValue> for JssSchema {
    type Error = JssError;

    fn try_from(js: JsValue) -> Result<Self, Self::Error> {
        let value: JsonValue = from_value(js)?;
        return JssSchema::try_from(value);
    }
}

impl JssSchema {
    pub fn validate_js(&self, object: JsValue) -> Vec<JsError> {
        let value = match from_value::<JsonValue>(object) {
            Ok(o) => o,
            Err(e) => return vec![JsError::from(e)],
        };
        return self.validate(&value).into_iter().map(JsError::from).collect();
    }
}

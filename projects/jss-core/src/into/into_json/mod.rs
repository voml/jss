use crate::{JssError, JssSchema, Result};
use json_value::{JsonObject, JsonValue};
use jsonschema::{Draft, JSONSchema};
use std::str::FromStr;

impl JssSchema {
    pub fn as_json_schema(&self) -> JsonValue {
        JsonValue::from(self.clone())
    }
    pub fn as_validator(&self) -> Result<JSONSchema> {
        let schema = self.as_json_schema();
        let compiled = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema)?;
        return Ok(compiled);
    }
    pub fn validate(&self, s: &str) -> Vec<JssError> {
        let mut out = vec![];
        let value = match JsonValue::from_str(s) {
            Ok(o) => o,
            Err(e) => {
                out.push(JssError::from(e));
                return out;
            }
        };
        let validator = match self.as_validator() {
            Ok(o) => o,
            Err(e) => {
                out.push(e);
                return out;
            }
        };
        match validator.validate(&value) {
            Ok(_) => {}
            Err(e) => {
                for i in e {
                    out.push(JssError::from(i))
                }
            }
        }
        return out;
    }
    pub fn is_valid(&self, s: &str) -> bool {
        self.validate(s).is_empty()
    }
}

impl From<JssSchema> for JsonValue {
    fn from(jss: JssSchema) -> Self {
        if jss.is_anything() {
            return JsonValue::Bool(true);
        }
        if jss.is_noting() {
            return JsonValue::Bool(false);
        }
        let mut top = JsonObject::default();
        JsonValue::Object(top)
    }
}

impl From<>

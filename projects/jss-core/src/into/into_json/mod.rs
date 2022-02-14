use crate::{JssError, JssSchema, JssType, JssValue, Result};
use json_value::{to_string_pretty, JsonObject, JsonValue};
use jsonschema::{Draft, JSONSchema};

impl JssSchema {
    pub fn as_json_schema(&self) -> String {
        to_string_pretty(&JsonValue::from(self))
    }
    pub fn as_validator(&self) -> Result<JSONSchema> {
        let schema = JsonValue::from(self);
        let compiled = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema)?;
        return Ok(compiled);
    }
    pub fn validate(&self, value: &JsonValue) -> Vec<JssError> {
        let mut out = vec![];
        let validator = match self.as_validator() {
            Ok(o) => o,
            Err(e) => {
                out.push(e);
                return out;
            }
        };
        match validator.validate(value) {
            Ok(_) => {}
            Err(e) => {
                for i in e {
                    out.push(JssError::from(i))
                }
            }
        }
        return out;
    }
    pub fn is_valid(&self, s: &JsonValue) -> bool {
        self.validate(s).is_empty()
    }
}

impl From<&JssSchema> for JsonValue {
    fn from(jss: &JssSchema) -> Self {
        if jss.is_anything() {
            return JsonValue::Bool(true);
        }
        if jss.is_noting() {
            return JsonValue::Bool(false);
        }
        let mut object = JsonObject::default();
        if jss.is_top() {
            object.insert("title".to_string(), jss.get_name().into());
        }
        object.insert("type".to_string(), jss.get_type().into());
        if jss.has_description() {
            object.insert("description".to_string(), jss.get_description().into());
        }
        for (k, v) in jss.attributes() {
            object.insert(k.into(), v.clone().into());
        }
        let properties: JsonObject = jss.properties().map(|(k, v)| (k.clone(), v.into())).collect();
        object.insert("properties".to_string(), properties.into());

        JsonValue::Object(object)
    }
}

impl From<JssValue> for JsonValue {
    fn from(jss: JssValue) -> Self {
        match jss {
            JssValue::Null => JsonValue::Null,
            JssValue::Boolean(v) => JsonValue::Bool(v),
            JssValue::Number(v) => JsonValue::Number(v),
            JssValue::String(v) => JsonValue::String(v),
            JssValue::Url(v) => JsonValue::String(v),
            JssValue::Regex(v) => JsonValue::String(v),
            JssValue::Array(v) => v.into_iter().map(JsonValue::from).collect(),
            JssValue::Object(v) => v.into_iter().map(|(k, v)| (k, JsonValue::from(v))).collect(),
        }
    }
}

impl From<JssType> for JsonValue {
    fn from(jss: JssType) -> Self {
        JsonValue::String(jss.to_string())
    }
}

impl From<&JssType> for JsonValue {
    fn from(jss: &JssType) -> Self {
        JsonValue::String(jss.to_string())
    }
}

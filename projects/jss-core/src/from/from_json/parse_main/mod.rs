use super::*;
use crate::{JssKind, JssSchema, JssValue};
use json_value::{JsonMaybeObject, JsonValueCheck, JsonValueWrap};

impl From<JsonValue> for JssValue {
    fn from(v: JsonValue) -> Self {
        match v {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(v) => Self::Boolean(v),
            JsonValue::Number(v) => Self::Number(v),
            JsonValue::String(v) => Self::String(v),
            JsonValue::Array(v) => Self::Array(v.into_iter().map(|v| v.into()).collect()),
            JsonValue::Object(v) => Self::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

impl JssSchema {
    pub fn parse_json_schema_as_jss(top: JsonValue) -> Result<Self, Vec<JssError>> {
        let mut errors = vec![];
        match Self::parse_json_schema(top, &mut errors) {
            Ok(o) => Ok(o),
            Err(e) => {
                errors.push(e);
                Err(errors)
            }
        }
    }

    pub fn parse_json_schema(top: JsonValue, errors: Errors) -> Result<Self> {
        let mut top = top;
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Accepts anything, as long as it’s valid JSON
        if top.is_true() || top.is_empty() {
            return Ok(JssSchema::anything());
        }
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Schema that matches nothing.
        if top.is_false() {
            return Ok(JssSchema::nothing());
        }
        if top.is_null() || top.is_array() {
            return Err(JssError::undefined_variable("TODO"));
        }

        let mut jss = JssSchema::top();

        jss.parse_steps(true, &mut top, errors);
        jss.consume_rest(top, errors);

        Ok(jss)
    }
}

impl JssSchema {
    pub fn parse_value(name: String, value: JsonValue, errors: Errors) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.set_name(name);
        jss.parse_steps(false, &mut value, errors);
        jss.consume_rest(value, errors);
        Ok(jss)
    }
    fn parse_steps(&mut self, is_top: bool, value: &mut JsonValue, errors: Errors) {
        self.parse_type(value, errors);
        self.extend_properties("properties", is_top, value, errors);
        self.extend_definition("$defs", value, errors);
        self.extend_definition("definitions", value, errors);
    }
    fn consume_rest(&mut self, value: JsonValue, _: Errors) {
        let object = match value.into_object() {
            None => return,
            Some(s) => s,
        };
        for (k, v) in object {
            if k.starts_with("$") {
                self.keywords.insert(k, v.into())
            }
            else {
                self.attribute.insert(k, v.into())
            };
        }
    }
}

impl JssSchema {
    fn extend_properties(&mut self, key: &str, is_top: bool, value: &mut JsonValue, errors: Errors) {
        if let Some(object) = value.extract_key_as_object(key) {
            for (key, value) in object {
                match JssSchema::parse_value(key.to_owned(), value, errors) {
                    Ok(mut o) => {
                        if is_top {
                            o.kind = JssKind::PropertyTop
                        }
                        self.insert_property(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    fn extend_definition(&mut self, key: &str, value: &mut JsonValue, errors: Errors) {
        if let Some(object) = value.extract_key_as_object(key) {
            for (key, value) in object {
                match JssSchema::parse_value(key.to_owned(), value, errors) {
                    Ok(mut o) => {
                        o.kind = JssKind::Definition;
                        self.definition.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }
}

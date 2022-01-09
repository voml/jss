use crate::validation::Validation;
use json_value::JsonValue;
use jsonschema::JSONSchema;
use jss_error::{JssError, Result};
use serde_json::Value;

#[derive(Debug)]
pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String,
    Number,
    Object,
}

impl Default for JssType {
    fn default() -> Self {
        Self::Undefined
    }
}

impl JssType {
    fn parse_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => {
                unimplemented!()
            }
            Value::Bool(_) => {
                unimplemented!()
            }
            Value::Number(_) => {
                unimplemented!()
            }
            Value::String(s) => Self::parse_string(s),
            Value::Array(_) => {
                unimplemented!()
            }
            Value::Object(_) => {
                unimplemented!()
            }
        }
    }
    fn parse_string(value: &str) -> Result<Self> {
        let out = match value {
            "string" => Self::String,
            "object" => Self::Object,
            _ => unimplemented!("{}", value),
        };
        Ok(out)
    }
}

#[derive(Debug)]
pub struct JssSchema {
    ty: JssType,
}

impl Default for JssSchema {
    fn default() -> Self {
        Self { ty: Default::default() }
    }
}

impl JssSchema {
    pub fn anything() -> Self {
        Self { ty: JssType::Anything }
    }
    pub fn nothing() -> Self {
        Self { ty: JssType::Nothing }
    }
}

impl JssSchema {
    pub fn parse_json_schema(top: &Value) -> Validation<Self, JssError> {
        let mut errors = vec![];
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Accepts anything, as long as it’s valid JSON
        if top.is_true() || top.is_empty() {
            return Validation::success(JssSchema::anything(), errors);
        }
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Schema that matches nothing.
        if top.is_false() {
            return Validation::success(JssSchema::nothing(), errors);
        }
        if top.is_null() || top.is_array() {
            return Validation::failure(JssError::undefined_variable("TODO"), errors);
        }

        let mut jss = JssSchema::default();

        if let Some(v) = top.get_key("type") {
            match JssType::parse_value(v) {
                Ok(t) => jss.ty = t,
                Err(e) => errors.push(e),
            }
        }
        if let Some(v) = top.get_key("properties") {
            match JssType::parse_value(v) {
                Ok(t) => jss.ty = t,
                Err(e) => errors.push(e),
            }
        }

        todo!()
    }
}

#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    println!("{:#?}", JssSchema::parse_json_schema(&top))
}

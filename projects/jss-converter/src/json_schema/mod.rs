use crate::validation::Validation;
use json_value::JsonValue;
use jsonschema::JSONSchema;
use jss_error::{JssError, Result};
use serde_json::{Map, Value};

pub(crate) mod types;

pub use self::types::JssType;

#[derive(Debug)]
pub struct JssSchema {
    annotation: JssAnnotation,
    properties: Vec<JssProperty>,
}

#[derive(Debug)]
pub struct JssProperty {
    annotation: JssAnnotation,
}
#[derive(Debug)]
pub struct JssAnnotation {
    typing: JssType,
}

impl JssAnnotation {
    pub fn parse_type(&mut self, value: Value, errors: &mut Vec<JssError>) {
        match JssType::parse_value(&value) {
            Ok(t) => self.typing = t,
            Err(e) => errors.push(e),
        }
    }
}

impl Default for JssAnnotation {
    fn default() -> Self {
        Self { typing: Default::default() }
    }
}

impl Default for JssSchema {
    fn default() -> Self {
        Self { annotation: Default::default(), properties: vec![] }
    }
}

impl JssSchema {
    #[inline]
    pub fn anything() -> Self {
        Self { annotation: JssAnnotation { typing: JssType::Anything }, ..Default::default() }
    }
    #[inline]
    pub fn nothing() -> Self {
        Self { annotation: JssAnnotation { typing: JssType::Nothing }, ..Default::default() }
    }
}

impl JssSchema {
    pub fn insert_properties(&mut self, value: Map<String, Value>, errors: &mut Vec<JssError>) {}

    pub fn insert_definition(&mut self, value: Map<String, Value>, errors: &mut Vec<JssError>) {}
}

impl JssSchema {
    pub fn parse_json_schema(top: Value) -> Validation<Self, JssError> {
        let mut top = top;
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

        if let Some(s) = top.extract_key("type") {
            jss.annotation.parse_type(s, &mut errors)
        }
        if let Some(s) = top.extract_key_as_object("properties") {
            jss.insert_properties(s, &mut errors)
        }
        if let Some(s) = top.extract_key_as_object("$defs") {
            jss.insert_definition(s, &mut errors)
        }
        Validation::success(jss, errors)
    }
}

#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    println!("{:#?}", JssSchema::parse_json_schema(top))
}

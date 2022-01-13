use crate::validation::Validation;
use indexmap::IndexMap;
use json_value::{JsonMaybeObject, JsonObject, JsonValueCheck, JsonValueWrap};
use jss_error::{JssError, Result};
use serde_json::Value;

pub(crate) mod types;

pub use self::types::JssType;

#[derive(Debug)]
pub struct JssSchema {
    annotation: JssAnnotation,
    properties: IndexMap<String, JssProperty>,
    definition: IndexMap<String, JssProperty>,
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
    pub fn parse_type(&mut self, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(s) = value.extract_key("type") {
            self.for_type(s, errors);
            return;
        }
        if let Some(s) = value.extract_key("$ref") {
            self.for_ref(s, errors)
        }
    }
    fn for_type(&mut self, value: Value, errors: &mut Vec<JssError>) {
        match JssType::parse_value(&value) {
            Ok(t) => self.typing = t,
            Err(e) => errors.push(e),
        }
    }
    fn for_ref(&mut self, value: Value, errors: &mut Vec<JssError>) {
        match value.into_string() {
            None => {
                errors.push(JssError::runtime_error("$ref must string"));
                return;
            }
            Some(s) => match JssType::parse_ref(s) {
                Ok(o) => self.typing = o,
                Err(e) => errors.push(e),
            },
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
        Self { annotation: Default::default(), properties: Default::default(), definition: Default::default() }
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

impl Default for JssProperty {
    fn default() -> Self {
        Self { annotation: Default::default() }
    }
}

impl JssProperty {
    pub fn parse_value(value: Value, errors: &mut Vec<JssError>) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.annotation.parse_type(&mut value, errors);

        Ok(jss)
    }
}

impl JssSchema {
    pub fn extend_properties(&mut self, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(object) = value.extract_key_as_object("properties") {
            for (key, value) in object {
                match JssProperty::parse_value(value, errors) {
                    Ok(o) => {
                        self.properties.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    pub fn extend_definition(&mut self, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(object) = value.extract_key_as_object("$defs") {
            for (key, value) in object {
                match JssProperty::parse_value(value, errors) {
                    Ok(o) => {
                        self.definition.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }
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

        jss.annotation.parse_type(&mut top, &mut errors);

        jss.extend_properties(&mut top, &mut errors);
        jss.extend_definition(&mut top, &mut errors);

        Validation::success(jss, errors)
    }
}

#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    println!("{:#?}", JssSchema::parse_json_schema(top))
}

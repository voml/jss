use json_value::{JsonMaybeObject, JsonValue, JsonValueWrap};

use crate::{schema::*, Errors, JssError, Result};

impl JssType {
    pub fn parse_value(typing: JsonValue, value: &mut JsonValue, errors: Errors) -> Result<Self> {
        match typing {
            JsonValue::Null => unimplemented!(),
            JsonValue::Bool(_) => unimplemented!(),
            JsonValue::Number(_) => unimplemented!(),
            JsonValue::String(s) => Self::parse_string(&s, value, errors),
            JsonValue::Array(_) => unimplemented!(),
            JsonValue::Object(_) => unimplemented!(),
        }
    }
    fn parse_string(typing: &str, _value: &mut JsonValue, _errors: Errors) -> Result<Self> {
        // let mut t = JssComplexType::default();
        // t.parse(value, errors);
        // Self::Complex(Box::new(t));
        Ok(JssType::from(typing))
    }
    pub fn parse_ref(r: String) -> Result<Self> {
        Ok(Self::Reference(r))
    }
}

impl JssComplexType {
    pub fn parse(&mut self, value: &mut JsonValue, errors: Errors) {
        self.parse_pattern("pattern", value, errors)
    }
    fn parse_pattern(&mut self, key: &str, value: &mut JsonValue, _: Errors) {
        if let Some(s) = value.extract_key_as_string(key) {
            self.pattern = JssValue::Regex(s)
        }
    }
}

impl JssSchema {
    pub(crate) fn parse_type(&mut self, value: &mut JsonValue, errors: Errors) {
        if let Some(s) = value.extract_key("$ref") {
            self.for_ref(s, errors);
            return;
        }
        if let Some(s) = value.extract_key("type") {
            self.for_type(s, value, errors);
        }
    }
    fn for_type(&mut self, typing: JsonValue, value: &mut JsonValue, errors: Errors) {
        match JssType::parse_value(typing, value, errors) {
            Ok(t) => self.set_type(t),
            Err(e) => errors.push(e),
        }
    }
    fn for_ref(&mut self, value: JsonValue, errors: Errors) {
        match value.into_string() {
            None => {
                errors.push(JssError::runtime_error("`$ref` must string"));
                return;
            }
            Some(s) => match JssType::parse_ref(s) {
                Ok(o) => self.set_type(o),
                Err(e) => errors.push(e),
            },
        }
    }
}

impl JssSchema {
    pub(crate) fn parse_name(&mut self, value: &mut JsonValue, _: Errors) {
        let doc = value.extract_key_as_string("title");
        if let Some(s) = doc {
            self.set_name(s);
        }
    }
    pub(crate) fn parse_description(&mut self, value: &mut JsonValue, _: Errors) {
        let doc = value.extract_key_as_string("description").or_else(|| value.extract_key_as_string("$comment"));
        if let Some(s) = doc {
            self.set_description(s);
        }
    }
}

use crate::{Errors, JssError, Result};

use super::*;

impl JssSchema {
    #[inline]
    pub fn anything() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Anything, ..Default::default() }
    }
    #[inline]
    pub fn nothing() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Nothing, ..Default::default() }
    }
    #[inline]
    pub fn top() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Undefined, ..Default::default() }
    }
}

impl JssSchema {
    pub fn parse_json_schema_as_jss(top: Value) -> Result<Self, Vec<JssError>> {
        let mut errors = vec![];
        match Self::parse_json_schema(top, &mut errors) {
            Ok(o) => Ok(o),
            Err(e) => {
                errors.push(e);
                Err(errors)
            }
        }
    }

    pub fn parse_json_schema(top: Value, errors: Errors) -> Result<Self> {
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
    pub fn parse_value(name: String, value: Value, errors: Errors) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.name = Some(name);
        jss.parse_steps(false, &mut value, errors);
        jss.consume_rest(value, errors);
        Ok(jss)
    }
    fn parse_steps(&mut self, is_top: bool, value: &mut Value, errors: Errors) {
        self.parse_type(value, errors);
        self.extend_properties("properties", is_top, value, errors);
        self.extend_definition("$defs", value, errors);
        self.extend_definition("definitions", value, errors);
    }
    fn consume_rest(&mut self, value: Value, _: Errors) {
        let object = match value.into_object() {
            None => return,
            Some(s) => s,
        };
        for (k, v) in object {
            if k.starts_with("$") {
                self.keywords.insert(k, v.into())
            }
            else {
                self.annotation.insert(k, v.into())
            };
        }
    }
}

impl JssSchema {
    pub fn extend_properties(&mut self, key: &str, is_top: bool, value: &mut Value, errors: Errors) {
        if let Some(object) = value.extract_key_as_object(key) {
            for (key, value) in object {
                match JssSchema::parse_value(key.to_owned(), value, errors) {
                    Ok(mut o) => {
                        if is_top {
                            o.kind = JssKind::PropertyTop
                        }
                        self.properties.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    pub fn extend_definition(&mut self, key: &str, value: &mut Value, errors: Errors) {
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

impl Default for JssStringType {
    fn default() -> Self {
        Self { pattern: JssValue::string("") }
    }
}

impl From<JssStringType> for JssType {
    fn from(v: JssStringType) -> Self {
        Self::String(Box::new(v))
    }
}
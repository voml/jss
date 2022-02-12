use crate::{schema::*, Errors, JssError, Result};

impl JssType {
    pub fn parse_value(typing: JsonValue, value: &mut JsonValue, errors: Errors) -> Result<Self> {
        match typing {
            JsonValue::Null => {
                unimplemented!()
            }
            JsonValue::Bool(_) => {
                unimplemented!()
            }
            JsonValue::Number(_) => {
                unimplemented!()
            }
            JsonValue::String(s) => Self::parse_string(&s, value, errors),
            JsonValue::Array(_) => {
                unimplemented!()
            }
            JsonValue::Object(_) => {
                unimplemented!()
            }
        }
    }
    fn parse_string(typing: &str, value: &mut JsonValue, errors: Errors) -> Result<Self> {
        let out = match typing {
            "string" => {
                let mut t = JssStringType::default();
                t.parse(value, errors);
                Self::Complex(Box::new(t))
            }
            "object" => Self::Object,
            _ => unimplemented!("{}", typing),
        };
        Ok(out)
    }
    pub fn parse_ref(r: String) -> Result<Self> {
        Ok(Self::Reference(r))
    }
}

impl JssStringType {
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
    pub fn parse_type(&mut self, value: &mut JsonValue, errors: Errors) {
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
            Ok(t) => self.typing = t,
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
                Ok(o) => self.typing = o,
                Err(e) => errors.push(e),
            },
        }
    }
}

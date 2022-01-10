use super::*;


impl JssType {
    pub fn parse_value(typing: Value, value: &mut Value, errors: Errors) -> Result<Self> {
        match typing {
            Value::Null => {
                unimplemented!()
            }
            Value::Bool(_) => {
                unimplemented!()
            }
            Value::Number(_) => {
                unimplemented!()
            }
            Value::String(s) => Self::parse_string(&s, value, errors),
            Value::Array(_) => {
                unimplemented!()
            }
            Value::Object(_) => {
                unimplemented!()
            }
        }
    }
    fn parse_string(typing: &str, value: &mut Value, errors: Errors) -> Result<Self> {
        let out = match typing {
            "string" => {
                let mut t = JssStringType::default();
                t.parse(value, errors);
                Self::String(Box::new(t))
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
    pub fn parse(&mut self, value: &mut Value, errors: Errors) {
        self.parse_pattern("pattern", value, errors)
    }
    fn parse_pattern(&mut self, key: &str, value: &mut Value, _: &mut Vec<JssError>) {
        if let Some(s) = value.extract_key_as_string(key) {
            self.pattern = JssValue::Regex(s)
        }
    }
}

impl JssSchema {
    pub fn parse_type(&mut self, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(s) = value.extract_key("$ref") {
            self.for_ref(s, errors);
            return;
        }
        if let Some(s) = value.extract_key("type") {
            self.for_type(s, value, errors);
        }
    }
    fn for_type(&mut self, typing: Value, value: &mut Value, errors: &mut Vec<JssError>) {
        match JssType::parse_value(typing, value, errors) {
            Ok(t) => self.typing = t,
            Err(e) => errors.push(e),
        }
    }
    fn for_ref(&mut self, value: Value, errors: &mut Vec<JssError>) {
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

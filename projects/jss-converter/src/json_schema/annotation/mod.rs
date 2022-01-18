use super::*;

impl JssSchema {
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

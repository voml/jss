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

        let mut jss = JssSchema::top();

        jss.parse_steps(&mut top, &mut errors);

        Validation::success(jss, errors)
    }
}

impl JssSchema {
    pub fn parse_value(value: Value, errors: &mut Vec<JssError>) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.parse_steps(&mut value, errors);
        Ok(jss)
    }
    #[inline]
    fn parse_steps(&mut self, value: &mut Value, errors: &mut Vec<JssError>) {
        self.parse_type(value, errors);
        self.extend_properties("properties", value, errors);
        self.extend_definition("$defs", value, errors);
    }
}

impl JssSchema {
    pub fn extend_properties(&mut self, key: &str, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(object) = value.extract_key_as_object(key) {
            for (key, value) in object {
                match JssSchema::parse_value(value, errors) {
                    Ok(o) => {
                        self.properties.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    pub fn extend_definition(&mut self, key: &str, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(object) = value.extract_key_as_object(key) {
            for (key, value) in object {
                match JssSchema::parse_value(value, errors) {
                    Ok(o) => {
                        self.definition.insert(key, o);
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
    }
    pub fn parse_pattern(&mut self, key: &str, value: &mut Value, errors: &mut Vec<JssError>) {
        if let Some(s) = value.extract_key_as_string(key) {
            unimplemented!("{}", s)
        }
    }
}

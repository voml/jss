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
    pub fn parse_json_schema_as_jss(top: Value) -> Result<String> {
        Self::parse_json_schema(top).result().map(|jss| jss.to_string())
    }

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

        jss.parse_steps(true, &mut top, &mut errors);
        jss.consume_rest(top, &mut errors);

        Validation::success(jss, errors)
    }
}

impl JssSchema {
    pub fn parse_value(name: String, value: Value, errors: &mut Vec<JssError>) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.name = Some(name);
        jss.parse_steps(false, &mut value, errors);
        jss.consume_rest(value, errors);
        Ok(jss)
    }
    fn parse_steps(&mut self, is_top: bool, value: &mut Value, errors: &mut Vec<JssError>) {
        self.parse_type(value, errors);
        self.extend_properties("properties", is_top, value, errors);
        self.extend_definition("$defs", value, errors);
        self.extend_definition("definitions", value, errors);
    }
    fn consume_rest(&mut self, value: Value, _: &mut Vec<JssError>) {
        let object = match value.into_object() {
            None => return,
            Some(s) => s,
        };
        for (k, v) in object {
            if k.starts_with("$") {
                self.keywords.insert(k, v.into())
            } else {
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

    pub fn extend_definition(&mut self, key: &str, value: &mut Value, errors: &mut Vec<JssError>) {
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



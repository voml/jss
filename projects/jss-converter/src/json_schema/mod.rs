use jsonschema::JSONSchema;
use serde_json::Value;
use jss_error::git2::SubmoduleUpdate::Default;
use jss_error::JssError;


pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String,
    Number,
}

impl Default for JssType {
    fn default() -> Self {
        Self::Undefined
    }
}

impl JssType {
    fn parse_value(value: &Value) {
        match value {
            Value::Null => { unimplemented!() }
            Value::Bool(_) => { unimplemented!() }
            Value::Number(_) => { unimplemented!() }
            Value::String(_) => { unimplemented!() }
            Value::Array(_) => { unimplemented!() }
            Value::Object(_) => { unimplemented!() }
        }
    }
    fn parse_string(value: &Value) {}
}

#[derive(Debug)]
pub struct JssSchema {
    ty: JssType,
}

impl Default for JssSchema {
    fn default() -> Self {
        Self {
            ty: Default::default()
        }
    }
}

impl JssSchema {
    pub fn anything() -> Self {
        Self {
            ty: JssType::Anything
        }
    }
    pub fn nothing() -> Self {
        Self {
            ty: JssType::Nothing
        }
    }
}

pub enum Validation<T, E> {
    Success {
        result: T,
        errors: Vec<E>,
    },
    Failure {
        fatal: E,
        errors: Vec<E>,
    },
}

impl<T, E> Validation<T, E> {
    pub fn success(result: T, errors: Vec<E>) -> Validation<T, E> {
        Self::Success {
            result,
            errors,
        }
    }
    pub fn failure(fatal: E, errors: Vec<E>) -> Validation<T, E> {
        Self::Failure {
            fatal,
            errors,
        }
    }

    pub fn result(self) -> Result<T, E> {
        match self {
            Self::Success { result, .. } => {
                Ok(result)
            }
            Self::Failure { fatal, .. } => {
                Err(fatal)
            }
        }
    }

    pub fn errors(&self) -> &Vec<E> {
        match self {
            Self::Success { errors, .. } => { errors }
            Self::Failure { errors, .. } => { errors }
        }
    }

    pub fn unwrap(self) {
        match self {
            Self::Success { result, .. } => {
                result
            }
            Self::Failure { .. } => {
                panic!("called `Validation::unwrap()` on an `Failure` value")
            }
        }
    }
}

impl JssSchema {
    pub fn parse_json_schema() -> Validation<Self, JssError> {
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
        if top.is_null() {
            return Validation::failure();
        }

        let mut jss = JssSchema::default();
    }
}


#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    JssSchema::parse_json_schema()

    println!("{:#?}", JssSchema::try_from(top))
}
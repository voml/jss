use crate::validation::Validation;
use indexmap::IndexMap;
use json_value::{JsonMaybeObject, JsonValueCheck, JsonValueWrap};
use jss_error::{JssError, Result};
use serde_json::Value;

mod annotation;
mod schema;
mod types;

pub enum JssKind {
    Scheme,
    Property,
}

#[derive(Debug)]
pub struct JssProperty {
    kind: JssKind,
    typing: JssType,
    properties: IndexMap<String, JssProperty>,
    definition: IndexMap<String, JssProperty>,
}

#[derive(Debug)]
pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String,
    Number,
    Object,
    Reference(String),
}

impl JssProperty {
    pub fn parse_value(value: Value, errors: &mut Vec<JssError>) -> Result<Self> {
        let mut value = value;
        let mut jss = Self::default();
        jss.annotation.parse_type(&mut value, errors);

        Ok(jss)
    }
}

use crate::validation::Validation;
use indexmap::IndexMap;
use json_value::{JsonMaybeObject, JsonValueCheck, JsonValueWrap};
use jss_error::{JssError, Result};
use serde_json::Value;

mod annotation;
mod schema;
mod traits;
mod types;

#[derive(Debug)]
pub enum JssKind {
    Scheme,
    Property,
}

#[derive(Debug)]
pub struct JssSchema {
    kind: JssKind,
    typing: JssType,
    properties: IndexMap<String, JssSchema>,
    definition: IndexMap<String, JssSchema>,
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

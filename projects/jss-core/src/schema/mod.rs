mod parse_main;
mod parse_type;
mod traits;
mod value;

use crate::validation::Validation;
use indexmap::IndexMap;
use json_value::{
    serde_json::{Number, Value},
    JsonMaybeObject, JsonValueCheck, JsonValueWrap,
};
use jss_error::{JssError, Result};
use std::fmt::{Debug, Display, Formatter};

pub struct JssSchema {
    kind: JssKind,
    name: Option<String>,
    description: Option<String>,
    typing: JssType,
    properties: IndexMap<String, JssSchema>,
    definition: IndexMap<String, JssSchema>,
    annotation: IndexMap<String, JssValue>,
    keywords: IndexMap<String, JssValue>,
}

#[derive(Debug)]
pub enum JssKind {
    Scheme,
    Property,
    PropertyTop,
    Definition,
}

#[derive(PartialEq)]
pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String(Box<JssStringType>),
    Number,
    Object,
    Reference(String),
}

#[derive(PartialEq)]
pub struct JssStringType {
    /// Jss String
    pattern: JssValue,
}

/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(Clone, Eq, PartialEq)]
pub enum JssValue {
    /// Represents a null value.
    Null,

    /// Represents a boolean.
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    Number(Number),

    /// Represents a string.
    String(String),
    /// Represents a url.
    Url(String),
    /// Represents a regex.
    Regex(String),
    /// Represents an array.
    Array(Vec<JssValue>),

    /// Represents an object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    Object(IndexMap<String, JssValue>),
}

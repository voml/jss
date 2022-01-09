use crate::validation::Validation;
use indexmap::IndexMap;
use json_value::{JsonMaybeObject, JsonValueCheck, JsonValueWrap};
use jss_error::{JssError, Result};
use serde_json::{Number, Value};

mod parse_main;
mod parse_type;
mod traits;

#[derive(Debug)]
pub enum JssKind {
    Scheme,
    PropertyUntyped,
    PropertyString(Box<JssSchemaString>),
}

pub struct JssSchema {
    kind: JssKind,
    typing: JssType,
    properties: IndexMap<String, JssSchema>,
    definition: IndexMap<String, JssSchema>,
    annotation: IndexMap<String, JssValue>,
    keywords: IndexMap<String, JssValue>,
    //
}

pub struct JssSchemaString {
    pattern: String,
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

/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(Clone, Eq, PartialEq)]
pub enum JssValue {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,

    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),

    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<JssValue>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(IndexMap<String, JssValue>),
}

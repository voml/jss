use std::fmt::{Debug, Display, Formatter};

use indexmap::{map::Iter, IndexMap};

use json_value::Number;

#[cfg(feature = "mock")]
mod mock;
mod traits;
mod typing;
mod value;

/// The schema node
#[derive(Clone)]
pub struct JssSchema {
    /// The kind of this node
    pub kind: JssKind,
    /// The name of the node
    name: Option<String>,
    /// The documentations of the node
    description: String,
    /// type definition
    typing: JssType,
    /// .key = value
    property: IndexMap<String, JssSchema>,
    /// Top level node only
    /// ^key = value
    definition: IndexMap<String, JssSchema>,
    /// key = value
    attribute: IndexMap<String, JssValue>,
    // $key = value
    // keywords: IndexMap<String, JssValue>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JssKind {
    Scheme,
    Property,
    PropertyTop,
    Definition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String,
    Integer,
    Number,
    Array,
    Object,
    Reference(String),
    Complex(Box<JssComplexType>),
}

#[derive(Clone, PartialEq)]
pub struct JssComplexType {
    /// Jss String
    pub pattern: JssValue,
}

/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(Clone, Eq, PartialEq)]
pub enum JssValue {
    /// Represents a null value.
    Null,

    /// Represents a boolean.
    Boolean(bool),

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

use std::fmt::{Debug, Display, Formatter};

use indexmap::IndexMap;

use json_value::{JsonMaybeObject, JsonValue, JsonValueCheck, JsonValueWrap, Number};

mod parse_main;
mod parse_type;
mod traits;
mod value;

/// The schema node
pub struct JssSchema {
    /// The kind of this node
    pub kind: JssKind,
    /// The name of the node
    name: Option<String>,
    /// The documentations of the node
    pub description: Option<String>,
    /// type definition
    typing: JssType,
    /// .key = value
    property: IndexMap<String, JssSchema>,
    /// Top level node only
    /// ^key = value
    pub definition: IndexMap<String, JssSchema>,
    /// key = value
    pub attribute: IndexMap<String, JssValue>,
    pub keywords: IndexMap<String, JssValue>,
}

impl JssSchema {
    pub fn get_name(&self) -> &str {
        debug_assert!(self.name.is_some());
        match &self.name {
            Some(s) => s.as_str(),
            None => "",
        }
    }
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = Some(name.into())
    }

    pub fn get_type(&self) -> &JssType {
        &self.typing
    }
    pub fn set_type<S>(&mut self, typing: S)
    where
        S: Into<JssType>,
    {
        self.typing = typing.into()
    }

    pub fn insert_property<K, V>(&mut self, key: K, value: V) -> Option<JssSchema>
    where
        K: Into<String>,
        V: Into<JssSchema>,
    {
        self.property.insert(key.into(), value.into())
    }
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
    String,
    Integer,
    Number,
    Array,
    Object,
    Reference(String),
    Complex(Box<JssStringType>),
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

use std::fmt::{Debug, Display, Formatter};

use indexmap::{map::Iter, IndexMap};

use json_value::Number;

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
    pub definition: IndexMap<String, JssSchema>,
    /// key = value
    attribute: IndexMap<String, JssValue>,
    // $key = value
    // keywords: IndexMap<String, JssValue>,
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

    pub fn has_description(&self) -> bool {
        !self.description.is_empty()
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description<S>(&mut self, description: S)
    where
        S: Into<String>,
    {
        self.description = description.into()
    }

    pub fn properties(&self) -> Iter<'_, String, JssSchema> {
        self.property.iter()
    }
    pub fn insert_property<K, V>(&mut self, key: K, value: V) -> Option<JssSchema>
    where
        K: Into<String>,
        V: Into<JssSchema>,
    {
        self.property.insert(key.into(), value.into())
    }

    pub fn attributes(&self) -> Iter<'_, String, JssValue> {
        self.attribute.iter()
    }
    pub fn insert_attribute<K, V>(&mut self, key: K, value: V) -> Option<JssValue>
    where
        K: Into<String>,
        V: Into<JssValue>,
    {
        self.attribute.insert(key.into(), value.into())
    }
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

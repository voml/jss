use crate::{
    json_value::{JsonArray, JsonObject},
    JsonValue,
};
use serde_json::{Map, Value};

impl JsonValue for Value {
    fn get_key(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(s) => s.get(key),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Bool(s) => Some(s),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<&dyn JsonArray<ValueType = Self>> {
        match self {
            Self::Array(s) => Some(s),
            _ => None,
        }
    }

    fn as_object(&self) -> Option<&dyn JsonObject<ValueType = Self>> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn is_null(&self) -> bool {
        matches!(self, Null)
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Array(s) if s.is_empty() => true,
            Self::Object(s) if s.is_empty() => true,
            _ => false,
        }
    }
}

impl<K, V> JsonObject for Map<K, V> {
    type ValueType = V;
}

impl<V> JsonArray for Vec<V> {
    type ValueType = V;
}

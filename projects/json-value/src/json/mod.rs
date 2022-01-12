use crate::JsonValue;
use serde_json::{Map, Value};

impl JsonValue for Value {
    fn get_key(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(s) => s.get(key),
            _ => None,
        }
    }
    fn extract_key(&mut self, key: &str) -> Option<Value> {
        match self {
            Self::Object(s) => s.remove(key),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Bool(s) => Some(s),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(s) => Some(s),
            _ => None,
        }
    }

    fn as_object(&self) -> Option<&Map<String, Value>> {
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

    fn into_object(self) -> Option<Map<String, Self>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}

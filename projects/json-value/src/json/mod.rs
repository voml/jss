use crate::{JsonMaybeArray, JsonMaybeObject, JsonValueCheck, JsonValueWrap};
use serde_json::{Map, Value};

impl JsonValueWrap for Value {
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

    fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_object(&self) -> Option<&Map<String, Value>> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }

    fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Bool(s) => Some(s),
            _ => None,
        }
    }

    fn into_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn into_array(self) -> Option<Vec<Self>> {
        match self {
            Self::Array(s) => Some(s),
            _ => None,
        }
    }

    fn into_object(self) -> Option<Map<String, Self>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl JsonValueCheck for Value {
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Array(s) if s.is_empty() => true,
            Self::Object(s) if s.is_empty() => true,
            _ => false,
        }
    }
}

impl JsonMaybeArray for Value {
    fn get_index(&self, index: usize) -> Option<&Self> {
        match self {
            Self::Array(s) => s.get(index),
            _ => None,
        }
    }

    fn mut_index(&mut self, index: usize) -> Option<&mut Self> {
        match self {
            Self::Array(s) => s.get_mut(index),
            _ => None,
        }
    }
}

impl JsonMaybeObject for Value {
    fn get_key(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(s) => s.get(key),
            _ => None,
        }
    }

    fn mut_key(&mut self, key: &str) -> Option<&mut Self> {
        match self {
            Self::Object(s) => s.get_mut(key),
            _ => None,
        }
    }

    fn extract_key(&mut self, key: &str) -> Option<Value> {
        match self {
            Self::Object(s) => s.remove(key),
            _ => None,
        }
    }
}

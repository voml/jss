use super::*;

mod display;

impl Default for JssKind {
    fn default() -> Self {
        Self::PropertyUntyped
    }
}

impl Default for JssType {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Default for JssSchema {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            typing: Default::default(),
            properties: Default::default(),
            definition: Default::default(),
            annotation: Default::default(),
            keywords: Default::default(),
        }
    }
}

impl From<Value> for JssValue {
    fn from(v: Value) -> Self {
        match v {
            Value::Null => Self::Null,
            Value::Bool(v) => Self::Bool(v),
            Value::Number(v) => Self::Number(v),
            Value::String(v) => Self::String(v),
            Value::Array(v) => Self::Array(v.into_iter().map(|v| v.into()).collect()),
            Value::Object(v) => Self::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

use super::*;

impl JssValue {
    pub fn string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }
}

impl JssValue {
    pub fn is_empty(&self) -> bool {
        match self {
            JssValue::String(s) => s.is_empty(),
            JssValue::Url(s) => s.is_empty(),
            JssValue::Regex(s) => s.is_empty(),
            JssValue::Array(s) => s.is_empty(),
            JssValue::Object(s) => s.is_empty(),
            _ => false,
        }
    }
}

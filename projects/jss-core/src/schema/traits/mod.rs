use super::*;

mod debug;
mod display;

impl Default for JssKind {
    fn default() -> Self {
        Self::Property
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
            name: None,
            description: None,
            typing: Default::default(),
            properties: Default::default(),
            definition: Default::default(),
            annotation: Default::default(),
            keywords: Default::default(),
        }
    }
}

impl From<JsonValue> for JssValue {
    fn from(v: JsonValue) -> Self {
        match v {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(v) => Self::Bool(v),
            JsonValue::Number(v) => Self::Number(v),
            JsonValue::String(v) => Self::String(v),
            JsonValue::Array(v) => Self::Array(v.into_iter().map(|v| v.into()).collect()),
            JsonValue::Object(v) => Self::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

impl JssSchema {
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = Some(name.into())
    }
}

use super::*;
use std::fmt::{Debug, Formatter};

impl Debug for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut match self.kind {
            JssKind::Scheme => f.debug_struct("JssScheme"),
            JssKind::PropertyUntyped => f.debug_struct("JssProperty"),
            _ => f.debug_struct("JssProperty"),
        };
        match &self.kind {
            JssKind::Scheme => {
                w.field("type", &self.typing);
                w.field("definition", &self.definition);
            }
            JssKind::PropertyUntyped => {
                w.field("type", &self.typing);
            }
            JssKind::PropertyString(s) => {
                w.field("type", &s);
            }
        }
        w.field("keywords", &self.keywords);
        w.field("annotations", &self.annotation);
        w.field("properties", &self.properties);
        w.finish()
    }
}

impl Debug for JssSchemaString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("string");
        if !self.pattern.is_empty() {
            w.field("pattern", &self.pattern);
        }
        w.finish()
    }
}

impl Debug for JssValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool(v) => Debug::fmt(v, f),
            Self::Number(v) => Debug::fmt(v, f),
            Self::String(v) => Debug::fmt(v, f),
            Self::Array(v) => Debug::fmt(v, f),
            Self::Object(v) => Debug::fmt(v, f),
        }
    }
}

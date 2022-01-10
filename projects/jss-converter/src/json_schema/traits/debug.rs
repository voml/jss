use super::*;
use std::fmt::{Debug, Formatter};


impl Debug for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut match self.kind {
            JssKind::Scheme => f.debug_struct("JssScheme"),
            JssKind::Property => f.debug_struct("JssProperty"),
        };
        w.field("type", &self.typing);
        if let JssKind::Scheme = &self.kind {
            w.field("definition", &self.definition);
        }
        w.field("keywords", &self.keywords);
        w.field("annotations", &self.annotation);
        w.field("properties", &self.properties);
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
            Self::Url(v) => f.write_str(v),
            Self::Regex(v) => f.write_str(v),
            Self::Array(v) => Debug::fmt(v, f),
            Self::Object(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for JssType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => { f.write_str("Undefined") }
            Self::Anything => { f.write_str("Anything") }
            Self::Nothing => { f.write_str("Nothing") }
            Self::String(v) => { Debug::fmt(v, f) }
            Self::Number => { f.write_str("Number") }
            Self::Object => { f.write_str("Object") }
            Self::Reference(v) => { Debug::fmt(v, f) }
        }
    }
}

impl Debug for JssStringType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("string");
        if !self.pattern.is_empty() {
            w.field("pattern", &self.pattern);
        }
        w.finish()
    }
}
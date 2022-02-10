use super::*;

impl Debug for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut match self.kind {
            JssKind::Scheme => f.debug_struct("JssScheme"),
            JssKind::Property | JssKind::PropertyTop => f.debug_struct("JssProperty"),
            JssKind::Definition => f.debug_struct("JssDefinition"),
        };
        w.field("type", &self.typing);
        if let JssKind::Scheme = &self.kind {
            w.field("definition", &self.definition);
        }
        w.field("keywords", &self.keywords);
        w.field("annotations", &self.attribute);
        w.field("properties", &self.property);
        w.finish()
    }
}

impl Debug for JssValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool(v) => Debug::fmt(v, f),
            Self::Number(v) => Display::fmt(v, f),
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
            Self::Undefined => f.write_str("Undefined"),
            Self::Anything => f.write_str("Anything"),
            Self::Nothing => f.write_str("Nothing"),
            JssType::String => f.write_str("String"),

            Self::Integer => f.write_str("Integer"),
            Self::Number => f.write_str("Number"),
            Self::Object => f.write_str("Object"),
            Self::Reference(v) => Debug::fmt(v, f),
            Self::Complex(v) => Debug::fmt(v, f),
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

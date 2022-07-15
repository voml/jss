use super::*;

impl From<String> for JssType {
    fn from(s: String) -> Self {
        JssType::from(s.as_str())
    }
}

impl From<&str> for JssType {
    fn from(s: &str) -> Self {
        match s {
            "object" => JssType::Object,
            "integer" => JssType::Integer,
            "number" => JssType::Number,
            "string" => JssType::String,
            "array" => JssType::Array,
            _ => unimplemented!("{:?}", s),
        }
    }
}

impl Default for JssStringType {
    fn default() -> Self {
        Self { pattern: JssValue::string("") }
    }
}

impl From<JssStringType> for JssType {
    fn from(v: JssStringType) -> Self {
        Self::Complex(Box::new(v))
    }
}

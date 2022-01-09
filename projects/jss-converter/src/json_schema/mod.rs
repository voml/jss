use jsonschema::JSONSchema;
use serde_json::Value;


pub trait JsonValue {
    fn get_key(&self, key: &str) -> Option<&Value>;
    fn as_string(&self) -> Option<&String>;
    #[inline]
    fn get_string_key(&self, key: &str) -> Option<&String> {
        self.get_key(key).and_then(|f| f.as_string())
    }

    fn is_true(&self) -> bool;
    fn is_false(&self) -> bool;
    fn is_empty(&self) -> bool;
}

pub enum Type {
    Anything,
    Nothing,
    String,
    Number,
}

impl JsonValue for Value {
    fn get_key(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Object(s) => {
                s.get(key)
            }
            _ => None
        }
    }

    fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => { Some(s) }
            _ => None
        }
    }

    fn is_true(&self) -> bool {
        matches!(self, Self::Bool(true))
    }

    fn is_false(&self) -> bool {
        matches!(self, Self::Bool(false))
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Array(s) if s.is_empty() => { true }
            Self::Object(s) if s.is_empty() => { true }
            _ => false
        }
    }
}

pub struct JssSchema {}

impl JssSchema {
    pub fn anything() {}
    pub fn nothing() {}
}


#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    // https://json-schema.org/understanding-json-schema/basics.html#id1
    // Accepts anything, as long as it’s valid JSON
    if top.is_true() || top.is_empty() {
        return JssSchema::anything();
    }
    // https://json-schema.org/understanding-json-schema/basics.html#id1
    // Schema that matches nothing.
    if top.is_false() {
        return JssSchema::nothing();
    }


    println!("{:#?}", top)
}
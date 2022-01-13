use super::*;

#[derive(Debug)]
pub enum JssType {
    Undefined,
    Anything,
    Nothing,
    String,
    Number,
    Object,
    Reference(String),
}

impl Default for JssType {
    fn default() -> Self {
        Self::Undefined
    }
}

impl JssType {
    pub fn parse_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => {
                unimplemented!()
            }
            Value::Bool(_) => {
                unimplemented!()
            }
            Value::Number(_) => {
                unimplemented!()
            }
            Value::String(s) => Self::parse_string(s),
            Value::Array(_) => {
                unimplemented!()
            }
            Value::Object(_) => {
                unimplemented!()
            }
        }
    }
    fn parse_string(value: &str) -> Result<Self> {
        let out = match value {
            "string" => Self::String,
            "object" => Self::Object,
            _ => unimplemented!("{}", value),
        };
        Ok(out)
    }
    pub fn parse_ref(r: String) -> Result<Self> {
        Ok(Self::Reference(r))
    }
}

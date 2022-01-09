use jsonschema::JSONSchema;
use serde_json::Value;
use jss_error::JssError;


pub enum JssType {
    Anything,
    Nothing,
    String,
    Number,
}


#[derive(Debug)]
pub struct JssSchema {
    ty: JssType
}

impl JssSchema {
    pub fn anything()-> Self {
        Self {
            ty: JssType::Anything
        }
    }
    pub fn nothing()-> Self {
        Self {
            ty: JssType::Nothing
        }
    }
}

impl TryFrom<Value> for JssSchema {
    type Error = JssError;
    fn try_from(top: Value) -> Result<Self, Self::Error> {
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Accepts anything, as long as it’s valid JSON
        if top.is_true() || top.is_empty() {
            return Ok(JssSchema::anything());
        }
        // https://json-schema.org/understanding-json-schema/basics.html#id1
        // Schema that matches nothing.
        if top.is_false() {
            return Ok(JssSchema::nothing());
        }




    }
}

#[test]
fn test() {
    let top = include_str!("ref.json").parse::<Value>().unwrap();
    println!("{:#?}", JssSchema::try_from(top))
}
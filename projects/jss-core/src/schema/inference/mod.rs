use crate::{JssSchema, JssType};
use json_value::{JsonValue, Number};

impl JssSchema {
    pub fn inference(data: JsonValue) -> Self {
        match data {
            JsonValue::Null => {
                unimplemented!()
            }
            JsonValue::Bool(_) => {
                unimplemented!()
            }
            JsonValue::Number(n) => infer_number(n),
            JsonValue::String(_) => {
                unimplemented!()
            }
            JsonValue::Array(_) => {
                unimplemented!()
            }
            JsonValue::Object(_) => {
                unimplemented!()
            }
        }
    }
}

fn infer_number(data: Number) -> JssSchema {
    let mut jss = JssSchema::property();
    jss.typing = JssType::Number;
    todo!("{}", data)
}

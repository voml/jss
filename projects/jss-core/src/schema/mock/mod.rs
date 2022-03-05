use crate::{JssKind, JssSchema, JssType, JssValue};

impl JssSchema {
    pub fn mock(&self) -> JssValue {
        match self.typing {
            JssType::Undefined => {
                unimplemented!()
            }
            JssType::Anything => {
                unimplemented!()
            }
            JssType::Nothing => {
                unimplemented!()
            }
            JssType::String => {
                unimplemented!()
            }
            JssType::Integer => self.mock_integer(),
            JssType::Number => {
                unimplemented!()
            }
            JssType::Array => {
                unimplemented!()
            }
            JssType::Object => {
                unimplemented!()
            }
            JssType::Reference(_) => {
                unimplemented!()
            }
            JssType::Complex(_) => {
                unimplemented!()
            }
        }
    }
    fn mock_integer(&self) -> JssValue {
        match self.typing {
            JssType::Integer => self.mock(),
            _ => {
                unimplemented!()
            }
        }
    }
}

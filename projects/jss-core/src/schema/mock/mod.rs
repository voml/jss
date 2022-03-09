use crate::{JssSchema, JssType, JssValue};

impl JssSchema {
    pub fn mock(&self) -> JssValue {
        self.mock_top()
    }
    fn mock_top(&self) -> JssValue {
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
            JssType::Number => self.mock_number(),
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
        unimplemented!()
    }
    fn mock_number(&self) -> JssValue {
        unimplemented!()
    }
}

use crate::{JssError, JssSchema};
use wasm_bindgen::{JsError, JsValue};

impl From<JsValue> for JssSchema {
    fn from(_: JsValue) -> Self {
        todo!()
    }
}

impl From<JssError> for JsError {
    fn from(_: JssError) -> Self {
        todo!()
    }
}

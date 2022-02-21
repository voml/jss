use crate::{JssError, JssErrorKind};
use serde_wasm_bindgen::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        JssError { kind: Box::new(JssErrorKind::RuntimeError(e.to_string())), line: 0, column: 0 }
    }
}

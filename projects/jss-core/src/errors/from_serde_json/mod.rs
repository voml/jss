use crate::{JssError, JssErrorKind};
use serde_json::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        let kind = JssErrorKind::SyntaxError(e.to_string());
        Self { kind: Box::new(kind), line: e.line() as u32, column: e.column() as u32 }
    }
}

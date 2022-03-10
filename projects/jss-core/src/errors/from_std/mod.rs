use crate::{JssError, JssErrorKind};
use std::num::ParseFloatError;

impl From<ParseFloatError> for JssError {
    fn from(e: ParseFloatError) -> Self {
        JssError::syntax_error(e.to_string())
    }
}

impl From<std::io::Error> for JssError {
    fn from(e: std::io::Error) -> Self {
        JssError { kind: Box::new(JssErrorKind::IOError(e)), line: 0, column: 0 }
    }
}

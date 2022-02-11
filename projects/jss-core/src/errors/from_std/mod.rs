use crate::JssError;
use std::num::ParseFloatError;

impl From<ParseFloatError> for JssError {
    fn from(e: ParseFloatError) -> Self {
        JssError::syntax_error(e.to_string())
    }
}

use crate::{JssError, JssErrorKind};
use jsonschema::ValidationError;

impl From<ValidationError<'_>> for JssError {
    fn from(e: ValidationError<'_>) -> Self {
        let kind = JssErrorKind::ValidationFail(e.to_string());
        Self { kind: Box::new(kind), line: 0, column: 0 }
    }
}

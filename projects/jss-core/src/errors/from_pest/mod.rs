use jss_pest::{pest::error::LineColLocation, Error, Rule};

use crate::{JssError, JssErrorKind};

impl From<Error<Rule>> for JssError {
    fn from(e: Error<Rule>) -> Self {
        let kind = JssErrorKind::SyntaxError(e.to_string());
        match e.line_col {
            LineColLocation::Pos(_) => Self { kind: Box::new(kind), line: 0, column: 0 },
            LineColLocation::Span(start, _) => Self { kind: Box::new(kind), line: start.0 as u32, column: start.0 as u32 },
        }
    }
}

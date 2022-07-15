use std::{
    error::Error,
    fmt::{Display, Formatter},
};

mod from_serde_json;
mod from_validate;

/// All result about jss
pub type Result<T, E = JssError> = std::result::Result<T, E>;
/// Errors Collector
pub type Errors<'a> = &'a mut Vec<JssError>;

/// Error type for all jss operators
#[derive(Debug)]
pub struct JssError {
    /// Actual error kind
    pub kind: Box<JssErrorKind>,
    pub line: u32,
    pub column: u32,
}

/// Actual error data for the error
#[derive(Debug)]
pub enum JssErrorKind {
    /// The error type for I/O operations
    IOError(std::io::Error),
    /// The error type which is returned from formatting a message into a
    /// stream.
    FormatError(std::fmt::Error),
    /// The error type which is
    SyntaxError(String),
    /// The error type which is occurred at runtime
    RuntimeError(String),
    /// The error type which is
    TypeMismatch(String),
    /// The error type which is
    ValidationFail(String),
    /// Runtime error when variable is undefined
    UndefinedVariable {
        /// The name of the undefined variable
        name: String,
    },
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

impl JssError {
    pub fn undefined_variable<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        let kind = JssErrorKind::UndefinedVariable { name: msg.into() };
        Self { kind: Box::new(kind), line: 0, column: 0 }
    }
    pub fn runtime_error<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        let kind = JssErrorKind::RuntimeError(msg.into());
        Self { kind: Box::new(kind), line: 0, column: 0 }
    }
}

impl Error for JssError {}

impl Display for JssError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:indent$}{}", self.kind, indent = 4)
    }
}

impl Display for JssErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => {
                write!(f, "{}", e)
            }
            Self::FormatError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(msg) => {
                f.write_str("SyntaxError: ")?;
                f.write_str(msg)
            }
            Self::TypeMismatch(msg) => {
                f.write_str("TypeError: ")?;
                f.write_str(msg)
            }
            Self::RuntimeError(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            }
            JssErrorKind::ValidationFail(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            }
            Self::UndefinedVariable { name } => {
                write!(f, "RuntimeError: Variable {} not found in scope", name)
            }
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            }
        }
    }
}

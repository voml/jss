use std::error::Error;
use std::fmt::{Display, Formatter};

/// All result about jss
pub type Result<T> = std::result::Result<T, JssError>;
/// Error type for all jss operators
#[derive(Debug)]
pub struct JssError {
    /// Actual error kind
    pub kind: Box<JssErrorKind>,
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
    /// The error type which is
    TypeMismatch(String),
    /// The error type which is occurred at runtime
    RuntimeError(String),
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

macro_rules! error_msg {
    ($name:ident => $t:ident) => {
        /// Constructor of [`JssErrorKind::$t`]
        pub fn $name(msg: impl Into<String>) -> JssError {
            let kind = JssErrorKind::$t(msg.into());
            Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
        }
    };
    ($($name:ident => $t:ident),+ $(,)?) => (
        impl JssError { $(error_msg!($name=>$t);)+ }
    );
}

error_msg![
    syntax_error => SyntaxError,
    type_mismatch => TypeMismatch,
    runtime_error => RuntimeError,
];

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

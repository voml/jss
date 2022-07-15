#![doc = include_str!("../Readme.md")]

pub use self::{
    errors::{Errors, JssError, JssErrorKind, Result},
    schema::{JssKind, JssSchema, JssType, JssValue},
};

mod errors;
mod from;
mod into;
mod schema;

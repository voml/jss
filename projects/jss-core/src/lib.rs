#![doc = include_str!("../Readme.md")]

pub use self::{
    errors::{Errors, JssError, JssErrorKind, Result},
    from::parse_json,
    schema::{JssKind, JssSchema, JssType, JssValue},
};

mod errors;
mod from;
mod into;
mod schema;

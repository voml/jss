#![doc = include_str!("../Readme.md")]

mod json;
mod methods;

pub use self::methods::*;
pub use serde_json::{Error, Number, Value};

pub type JsonObject = serde_json::Map<String, Value>;

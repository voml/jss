#![doc = include_str!("../Readme.md")]

use serde_json::Value;
pub use serde_json::{Error, Number};

pub use self::methods::*;

mod json;
mod methods;

pub type JsonValue = Value;
pub type JsonObject = serde_json::Map<String, Value>;

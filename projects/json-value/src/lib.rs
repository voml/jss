#![doc = include_str!("../Readme.md")]

pub use self::methods::to_string_pretty;
use serde_json::Value;
pub use serde_json::{Error, Number};

pub use self::traits::*;

mod json;
mod methods;
mod traits;

pub type JsonValue = Value;
pub type JsonObject = serde_json::Map<String, Value>;

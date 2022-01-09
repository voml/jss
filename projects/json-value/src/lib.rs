#![doc = include_str!("../Readme.md")]

mod json;
mod methods;

pub use methods::JsonValue;
pub use serde_json::Map;

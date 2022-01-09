#![doc = include_str!("../Readme.md")]

mod json;
mod methods;

pub use methods::*;
pub use serde_json::{Map, Value as Json};

pub type JsonObject = Map<String, Json>;

#![doc = include_str!("../Readme.md")]

mod json;
mod methods;

pub use methods::*;


pub extern crate serde_json;

pub type JsonObject = serde_json::Map<String, serde_json::Value>;

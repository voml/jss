#![doc = include_str!("../Readme.md")]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn from_json_string(json: &str) -> String {
    json.to_string()
}

#[wasm_bindgen]
pub fn into_json_string(jss: &str) -> String {
    "jss".to_string()
}
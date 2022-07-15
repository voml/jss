use crate::Result;
use json_value::JsonValue;
use std::str::FromStr;

mod from_json;
mod from_text;

pub fn parse_json(json: &str) -> Result<JsonValue> {
    let json = JsonValue::from_str(json)?;
    return Ok(json);
}

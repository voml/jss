use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

/// Serialize a [`JsonValue`] into a [`String`] with pretty formatting.
pub fn to_string_pretty(json: &Value, space: usize) -> String {
    let buf = Vec::new();
    let tab = " ".repeat(space);
    let formatter = PrettyFormatter::with_indent(tab.as_bytes());
    let mut ser = Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}

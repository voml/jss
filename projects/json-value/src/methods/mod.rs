use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

pub fn to_string_pretty(json: &Value) -> String {
    let buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}

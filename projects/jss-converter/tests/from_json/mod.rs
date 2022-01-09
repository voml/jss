use super::*;

#[test]
fn test_string() {
    convert_from_json_schema(include_str!("string.json"), include_str!("string.yaml"))
}

#[test]
fn test_ref() {
    convert_from_json_schema(include_str!("ref.json"), include_str!("ref.yaml"))
}

#[test]
fn test_structuring() {
    convert_from_json_schema(include_str!("structuring.json"), include_str!("structuring.yaml"))
}

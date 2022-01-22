use super::*;

#[test]
fn test_string() {
    // convert_from_json_schema_debug(include_str!("string.json"), include_str!("string.yaml"));
    convert_from_json_schema(include_str!("string.json"), include_str!("string.jss"));
}

#[test]
fn test_ref() {
    // convert_from_json_schema_debug(include_str!("ref.json"), include_str!("ref.yaml"));
    convert_from_json_schema(include_str!("ref.json"), include_str!("ref.jss"));
}

#[test]
fn test_structuring() {
    // convert_from_json_schema_debug(include_str!("structuring.json"), include_str!("structuring.yaml"));
    convert_from_json_schema(include_str!("structuring.json"), include_str!("structuring.jss"));
}

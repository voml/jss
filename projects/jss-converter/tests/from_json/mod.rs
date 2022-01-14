use super::*;

#[test]
fn test() {
    convert_from_json_schema(include_str!("ref.json"), include_str!("ref.yaml"))
}

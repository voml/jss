use jss_convert::JssSchema;
use json_value::serde_json::Value;

mod from_json;

#[test]
fn ready() {
    println!("it works!")
}

fn convert_from_json_schema(source: &str, target: &str) {
    let top = source.parse::<Value>().unwrap();
    let jss = JssSchema::parse_json_schema(top);
    assert_eq!(format!("{:#?}", jss), target)
}

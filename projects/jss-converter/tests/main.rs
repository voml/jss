use jss_convert::JssObject;
use serde_json::Value;

mod from_json;

#[test]
fn ready() {
    println!("it works!")
}

fn convert_from_json_schema(source: &str, target: &str) {
    let top = source.parse::<Value>().unwrap();
    let jss = JssObject::parse_json_schema(top);
    assert_eq!(format!("{:#?}", jss), target)
}

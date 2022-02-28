use jss_core::{parse_json, JssSchema};
use std::str::FromStr;

#[test]
fn test() {
    let json = include_str!("open-api.json");
    parse_json(json).unwrap();
    let jss = JssSchema::parse_json_schema(parse_json(json).unwrap()).unwrap();
    println!("{}", jss)
}

#[test]
fn into_json() {
    let json = include_str!("raw.jss");
    let jss = JssSchema::from_str(json).unwrap();
    println!("{}", jss.as_json_schema())
}

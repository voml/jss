use jss_core::{parse_json, JssSchema, Result};
use std::str::FromStr;

fn check_jss(jss: &str, debug: &str, json: &str) -> Result<()> {
    let jss = JssSchema::from_str(jss)?;
    assert_eq!(format!("{:#?}", jss), debug);
    assert_eq!(jss.as_json_schema(), json);
    Ok(())
}

#[test]
fn test_basic() {
    let text = include_str!("simple/basic.jss");
    let target = include_str!("simple/basic.txt");
    let json = include_str!("simple/basic.json");
    check_jss(text, target, json).unwrap();
}

#[test]
fn test_nesting() {
    let text = include_str!("simple/nesting.jss");
    let target = include_str!("simple/nesting.txt");
    let json = include_str!("simple/nesting.json");
    check_jss(text, target, json).unwrap();
}

#[test]
fn test_properties() {
    let text = include_str!("simple/properties.jss");
    let target = include_str!("simple/properties.txt");
    let json = include_str!("simple/properties.json");
    check_jss(text, target, json).unwrap();
}

#[test]
fn test() {
    let json = include_str!("simple/properties_deeper.json");
    parse_json(json).unwrap();
    let jss = JssSchema::parse_json_schema(parse_json(json).unwrap()).unwrap();
    println!("{}", jss)
}

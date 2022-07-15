use std::str::FromStr;

use jss_core::JssSchema;

#[test]
fn test_basic() {
    let text = include_str!("simple/basic.jss");
    let target = include_str!("simple/basic.txt");
    let r = JssSchema::from_str(text).unwrap();
    println!("{:#?}", r.validate("{}"));
    assert_eq!(format!("{:#?}", r), target);
}

#[test]
fn test_nesting() {
    let text = include_str!("simple/nesting.jss");
    let target = include_str!("simple/nesting.txt");
    let r = JssSchema::from_str(text).unwrap();
    assert_eq!(format!("{:#?}", r), target)
}

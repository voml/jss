use jss_core::JssSchema;
use std::str::FromStr;
#[test]
fn test() {
    let text = include_str!("simple/basic.jss");
    let r = JssSchema::from_str(text).unwrap();
    println!("{:#?}", r);
}

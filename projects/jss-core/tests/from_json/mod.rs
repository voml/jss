use jss_core::{parse_json, JssSchema};

#[test]
fn test() {
    let json = include_str!("open-api.json");
    parse_json(json).unwrap();
    let jss = JssSchema::parse_json_schema(parse_json(json).unwrap()).unwrap();
    println!("{}", jss)
}

use crate::JssError;
use jsonschema::{Draft, JSONSchema};
use serde_json::{json, Value};

#[test]
fn test() -> Result<(), JssError> {
    let schema: Value = serde_json::from_str(
        r#"
        {"maxLength": 5}
    "#,
    )?;
    let compiled_schema = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema)?;
    println!("{:#?}", compiled_schema);
    Ok(())
}

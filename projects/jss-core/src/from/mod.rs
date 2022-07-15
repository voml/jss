mod from_text;

use crate::JssError;
use json_value::Value;
use jsonschema::{Draft, JSONSchema};
use std::str::FromStr;

fn valid_from_string() -> Result<(), JssError> {
    let schema = Value::from_str(
        r#"
        {"maxLength": 5}
    "#,
    )?;
    let compiled_schema = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema)?;
    println!("{:#?}", compiled_schema);
    Ok(())
}

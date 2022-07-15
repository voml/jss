mod from_json;
mod from_text;

use crate::JssError;
use json_value::JsonValue;
use jsonschema::{Draft, JSONSchema};
use std::str::FromStr;

fn valid_from_string() -> Result<(), JssError> {
    let schema = JsonValue::from_str(
        r#"
        {"maxLength": 5}
    "#,
    )?;
    let compiled_schema = JSONSchema::options().with_draft(Draft::Draft7).compile(&schema)?;
    println!("{:#?}", compiled_schema);
    Ok(())
}

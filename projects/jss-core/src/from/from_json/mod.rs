mod parse_main;
mod parse_type;

use crate::{Errors, JssError, JssSchema, Result};
use json_value::JsonValue;

impl TryFrom<JsonValue> for JssSchema {
    type Error = JssError;

    fn try_from(value: JsonValue) -> Result<Self> {
        let mut errors = vec![];
        Self::try_parse_json_schema(value, &mut errors)
    }
}

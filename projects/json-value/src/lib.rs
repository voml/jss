#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
// #![doc(html_logo_url = "")]
// #![doc(html_favicon_url = "")]

pub use self::methods::to_string_pretty;
use serde_json::Value;
pub use serde_json::{Error, Number};

pub use self::traits::*;

mod json;
mod methods;
mod traits;

/// Represents any valid JSON value.
pub type JsonValue = Value;
/// Represents any valid JSON object.
pub type JsonObject = serde_json::Map<String, Value>;

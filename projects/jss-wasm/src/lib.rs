#![doc = include_str!("../Readme.md")]
#![allow(non_snake_case)]

use std::str::FromStr;
use wasm_bindgen::prelude::*;

use jss_core::JssSchema;

#[wasm_bindgen]
pub struct Schema {
    internal: JssSchema,
}

#[wasm_bindgen]
impl Schema {
    /// Create a new JSS instance.
    ///
    ///
    /// ```js
    /// import Schema from "jss-wasmbind";
    ///
    /// const schema = new Schema(`
    /// /// A product in the catalog
    /// schema Product: object {
    ///     $schema: https://json-schema.org/draft/2020-12/schema
    ///     $id: https://example.com/product.schema.json
    ///     required: ["productId"]
    /// }
    ///
    /// /// The unique identifier for a product
    /// properties productId: integer;
    /// `)
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn from_string(jss: &str) -> Result<Schema, JsError> {
        Ok(Self { internal: JssSchema::from_str(jss)? })
    }

    /// Get the JSS instance.
    pub fn fromJsonSchema(object: JsValue) -> Result<Schema, JsError> {
        Ok(Self { internal: JssSchema::try_from(object)? })
    }

    /// Check if the object satisfies the schema
    ///
    /// ```js
    /// // true
    /// schema.isValid({
    ///     productId: 1,
    ///     productName: "A green door",
    ///     price: 12.50,
    ///     tags: ["home", "green"]
    /// })
    /// // false
    /// schema.isValid([])
    // ```
    pub fn validate(&self, object: JsValue) -> Vec<JsValue> {
        self.internal.validate_js(object)
    }

    /// Get the JSS instance.
    pub fn isValid(&self, object: JsValue) -> bool {
        self.internal.validate_js(object).is_empty()
    }

    /// Get random schema
    pub fn mock(&self) -> JsValue {
        unimplemented!("random")
    }

    pub fn toString(&self) -> String {
        self.internal.to_string()
    }

    pub fn toJsonSchema(&self) -> JsValue {
        self.internal.to_json_schema()
    }
}

/// Create a new JSS instance.
///
/// ```js
/// import jss from "jss-wasmbind";
///
/// const schema = jss`
/// /// A product in the catalog
/// schema Product: object {
///     $schema: https://json-schema.org/draft/2020-12/schema
///     $id: https://example.com/product.schema.json
///     required: ["productId"]
/// }
///
/// /// The unique identifier for a product
/// properties productId: integer;
/// `
/// ```
#[wasm_bindgen]
pub fn jss(jss: &str) -> Result<Schema, JsError> {
    Schema::from_string(jss)
}

#[test]
fn ready() {
    println!("it works!")
}

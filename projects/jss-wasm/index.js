const Schema = require("./pkg").Schema

const schema = new Schema(`
/// A product in the catalog
schema Product: object {
    $schema: https://json-schema.org/draft/2020-12/schema
    $id: https://example.com/product.schema.json
    required: ["productId"]
}

/// The unique identifier for a product
properties productId: integer;
`)

let v = schema.validate({
    productId: 1,
    productName: "A green door",
    price: 12.50,
    tags: ["home", "green"]
});

console.log(v);

console.log(schema.validate([]));

console.log(schema.toString());

console.log(schema.toJsonSchema())
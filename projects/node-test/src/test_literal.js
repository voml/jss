import { jss } from "jss-wasmbind";

const schema = jss`
/// A product in the catalog
schema Product: object {
    $schema: https://json-schema.org/draft/2020-12/schema
    $id: https://example.com/product.schema.json
    required: ["productId"]
}

/// The unique identifier for a product
properties productId: integer;
`

var v = schema.validate({
    productId: 1,
    productName: "A green door",
    price: 12.50,
    tags: ["home", "green"]
})

console.log(v);

var v = schema.validate([])

console.log(v);



console.log(schema.toString());

# Json Schema Simplified

```sh
npm i jss-wasmbind
```

## Example

### Build the schema

- build from jss

```js
import wasm from "jss-wasmbind";

const schema = wasm.jss`
/// A product in the catalog
schema Product: object {
    $schema: https://json-schema.org/draft/2020-12/schema
    $id: https://example.com/product.schema.json
    required: ["productId"]
}

/// The unique identifier for a product
properties productId: integer;
`
```

- build from json

### Check if the object satisfies the schema

```js
// true
schema.isValid({
    productId: 1,
    productName: "A green door",
    price: 12.50,
    tags: ["home", "green"]
})
// false
schema.isValid([])
```

### Find why the object does not satisfy the schema

```js
schema.validate([])
```

### Generate json schema for use with other json schema ecological libraries

```js
schema.toJSON()
```


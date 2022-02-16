/// A product from Acme's catalog
schema Product: object {
    $schema: "https://json-schema.org/draft/2020-12/schema"
    $id: "https://example.com/product.schema.json"
    required: [
        "productId",
        "productName",
        "price",
    ]
}
/// The unique identifier for a product
property productId: integer {
}

/// Name of the product
property productName: string {
}

/// The price of the product
property price: number {
    exclusiveMinimum: 0
}

/// Tags for the product
property tags: array {
    minItems: 1
    uniqueItems: true
    items: {
        "type": "string",
    }
}

property dimensions: object {
    .length: number
    .width: number
    .height: number
    required: [
        "length",
        "width",
        "height",
    ]
}

/// Coordinates of the warehouse where the product is located.
property warehouseLocation: "https://example.com/geographical-location.schema.json" {
}

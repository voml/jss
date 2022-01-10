schema _: object {
    $id: "https://example.com/schemas/customer"
    required: [
        "first_name",
        "last_name",
        "shipping_address",
        "billing_address",
    ]
}
name: JssProperty {
    type: string,
    keywords: {},
    annotations: {},
    properties: {},
}
first_name: JssProperty {
    type: "#/$defs/name",
    keywords: {},
    annotations: {},
    properties: {},
}
last_name: JssProperty {
    type: "#/$defs/name",
    keywords: {},
    annotations: {},
    properties: {},
}
shipping_address: JssProperty {
    type: "/schemas/address",
    keywords: {},
    annotations: {},
    properties: {},
}
billing_address: JssProperty {
    type: "/schemas/address",
    keywords: {},
    annotations: {},
    properties: {},
}

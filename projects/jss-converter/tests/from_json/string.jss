schema _: object {
    $id: "https://example.com/schemas/customer"
    $schema: "https://json-schema.org/draft/2020-12/schema"
}
prop first_name: string {
}

prop last_name: string {
    maxLength: 3
    minLength: 2
}

prop shipping_address: string {
}

prop billing_address: "/schemas/address" {
}


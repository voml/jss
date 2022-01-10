schema _: object {
    $id: "https://example.com/schemas/customer"
    $schema: "https://json-schema.org/draft/2020-12/schema"
}
.first_name: string {
}

.last_name: string {
    maxLength: 3
    minLength: 2
}

.shipping_address: string {
}

.billing_address: /schemas/address {
}


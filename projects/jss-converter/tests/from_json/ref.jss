schema _: object {
    $id: "https://example.com/schemas/customer"
    required: [
        "first_name",
        "last_name",
        "shipping_address",
        "billing_address",
    ]
}
def name: string {
}

.first_name: #/$defs/name {
}

.last_name: #/$defs/name {
}

.shipping_address: /schemas/address {
}

.billing_address: /schemas/address {
}


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

prop first_name: "#/$defs/name" {
}

prop last_name: "#/$defs/name" {
}

prop shipping_address: "/schemas/address" {
}

prop billing_address: "/schemas/address" {
}


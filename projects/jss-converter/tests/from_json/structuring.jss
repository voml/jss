schema _: object {
    $id: "https://example.com/schemas/customer"
    $schema: "https://json-schema.org/draft/2020-12/schema"
    required: [
        "first_name",
        "last_name",
        "shipping_address",
        "billing_address",
    ]
}
def address: object {
    $id: "/schemas/address"
    $schema: "http://json-schema.org/draft-07/schema#"
    required: [
        "street_address",
        "city",
        "state",
    ]
    .street_address: string {
    }


    .city: string {
    }


    .state: "#/definitions/state" {
    }


}

prop first_name: string {
}

prop last_name: string {
}

prop shipping_address: "/schemas/address" {
}

prop billing_address: "/schemas/address" {
}


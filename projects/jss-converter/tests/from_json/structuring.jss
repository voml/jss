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
    property street_address: string {
    }


    property city: string {
    }


    property state: #/definitions/state {
    }


}

.first_name: string {
}

.last_name: string {
}

.shipping_address: /schemas/address {
}

.billing_address: /schemas/address {
}


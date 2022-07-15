{
    "$id": "https://example.com/schemas/customer",
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "first_name": { "type": "string" },
        "last_name": {
            "type": "string",
            "minLength": 2,
            "maxLength": 3
        },
        "shipping_address": {
            "type": "string",
            "pattern": "^(\\([0-9]{3}\\))?[0-9]{3}-[0-9]{4}$"
        },
        "billing_address": {
            "$ref": "/schemas/address"
        }
    }
}
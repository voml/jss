define License: object {
    additionalProperties: false
    required: [
        "name",
    ]
    patternProperties: {
        "^x-": {},
    }
    .name: string
    .url: string {
        format: "uri-reference"
    }
}
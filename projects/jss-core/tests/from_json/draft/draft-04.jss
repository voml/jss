schema _: object {
    $schema: "http://json-schema.org/draft-01/hyper-schema#"
    id: "http://json-schema.org/draft-01/schema#"
    default: {}
    optional: true
}
prop type {
    default: "any"
    items: {
        "type": [
            "string",
            {
                "$ref": "#",
            },
        ],
    }
    optional: true
}

property properties: object {
    optional: true
    default: {}
    additionalProperties: {
        "$ref": "#",
    }
}

prop items {
    default: {}
    items: {
        "$ref": "#",
    }
    optional: true
}

prop optional: boolean {
    default: false
    optional: true
}

prop additionalProperties {
    default: {}
    optional: true
}

prop requires {
    optional: true
}

prop minimum: number {
    optional: true
}

prop maximum: number {
    optional: true
}

prop minimumCanEqual: boolean {
    default: true
    optional: true
    requires: "minimum"
}

prop maximumCanEqual: boolean {
    default: true
    optional: true
    requires: "maximum"
}

prop minItems: number {
    default: 0
    optional: true
    minimum: 0
}

prop maxItems: number {
    minimum: 0
    optional: true
}

prop pattern: string {
    format: "regex"
    optional: true
}

prop minLength: number {
    default: 0
    optional: true
    minimum: 0
}

prop maxLength: number {
    optional: true
}

prop enum: number {
    minItems: 1
    optional: true
}

prop title: string {
    optional: true
}

prop description: string {
    optional: true
}

prop format: string {
    optional: true
}

prop contentEncoding: string {
    optional: true
}

prop default: number {
    optional: true
}

prop maxDecimal: number {
    minimum: 0
    optional: true
}

prop disallow {
    optional: true
    items: {
        "type": "string",
    }
}

prop extends {
    default: {}
    items: {
        "$ref": "#",
    }
    optional: true
}


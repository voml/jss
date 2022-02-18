schema _ {
    $schema: "http://json-schema.org/draft-01/hyper-schema#"
    id: "http://json-schema.org/draft-01/hyper-schema#"
    extends: {
        "$ref": "http://json-schema.org/draft-01/schema#",
    }
    links: [
        {
            "href": "{$ref}",
            "rel": "full",
        },
        {
            "href": "{$schema}",
            "rel": "describedby",
        },
        {
            "href": "{id}",
            "rel": "self",
        },
    ]
    fragmentResolution: "dot-delimited"
}
prop links: number {
    optional: true
    items: {
        "$ref": "http://json-schema.org/draft-01/links#",
    }
}

prop fragmentResolution: string {
    default: "dot-delimited"
    optional: true
}

prop root: boolean {
    default: false
    optional: true
}

prop readonly: boolean {
    default: false
    optional: true
}

prop pathStart: string {
    format: "uri"
    optional: true
}

prop mediaType: string {
    format: "media-type"
    optional: true
}

prop alternate: number {
    optional: true
    items: {
        "$ref": "#",
    }
}


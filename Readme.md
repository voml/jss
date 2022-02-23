Json Schema Simplified
======================

A better way to write schemas

## Formats

### Conversion

#### From (Text/Binary)

```jss
/// The description of OpenAPI v3.1.x documents without schema validation 
/// as defined by https://spec.openapis.org/oas/v3.1.0
schema _: object {
    $id: "https://spec.openapis.org/oas/3.0/schema/2021-09-28"
    $schema: "http://json-schema.org/draft-04/schema#"
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    required: [
        "openapi",
        "info",
        "paths",
    ]
}
```


```json

```



## Developers



Json Schema Simplified
======================

A better way to write schemas

![](https://user-images.githubusercontent.com/17541209/179352862-4d2288d9-84e5-45f5-acd9-731ac8a21cfc.png)

- Jss Version: [open-api.jss](https://github.com/voml/jss/blob/dev/projects/jss-core/tests/from_json/open-api.jss)
- Json Schema
  Version: [open-api.json](https://github.com/voml/jss/blob/dev/projects/jss-core/tests/from_json/open-api.json)

## Tools and Implement

### Intellij

- [Jss Intellij](https://plugins.jetbrains.com/plugin/18376-jss-support)

### Rust

- [jss-rs](https://crates.io/crates/jss-core)
- jss-cli: TODO
- jss-mock: TODO

### Node

- Wasm bind: [jss-wasm](https://github.com/voml/jss/tree/dev/projects/jss-wasm)
- Online Playground: [replit.com@jss](https://replit.com/@voml/Json-Schema-Simplified#index.js)

## Syntax

### description

Aka. doc-comment, a comment starts with `///`

### schema

Top-level definitions, which specify constraints for all top-level fields

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

<details>
<summary>Equivalent json schema</summary>

```json
{
    "title": "_",
    "type": "object",
    "description": "The description of OpenAPI v3.1.x documents without schema validation\nas defined by https://spec.openapis.org/oas/v3.1.0",
    "$id": "https://spec.openapis.org/oas/3.0/schema/2021-09-28",
    "$schema": "http://json-schema.org/draft-04/schema#",
    "additionalProperties": false,
    "patternProperties": {
        "^x-": {}
    },
    "required": [
        "openapi",
        "info",
        "paths"
    ],
    "properties": {}
}
```

</details>

### define

Top-level definition, which specifies all references

```jss
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
```

<details>
<summary>Equivalent json schema</summary>

```json

```

</details>

### property

Recursive definition, specifying the properties of each item

`property` can be abbreviated as `.`

```jss
/// Dimensions for the product
property dimensions: object {
    .length: number
    .width: number
    .height: number

    required: ["length", "width", "height"]
}
```

<details>
<summary>Equivalent json schema</summary>

```json

```

</details>

## Evolution

- CLI Tools
- Mock data generator based on jss
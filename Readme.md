Json Schema Simplified
======================

A better way to write schemas

![](https://user-images.githubusercontent.com/17541209/179352862-4d2288d9-84e5-45f5-acd9-731ac8a21cfc.png)

## Tools and Implement


### Jetbrain
- [Jss Intellij](https://github.com/oovm/jss-intellij)

### Rust
- [jss-rs](https://github.com/voml/jss-rs)

### Node
- 
- Online Playground: https://replit.com/@voml/Json-Schema-Simplified#index.js

## Syntax

### schema

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

```

</details>

### define

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

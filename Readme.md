Panduck document conversion tool
================================

Conversion tool by rust, inspired by [pandoc]().

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



#### Into (Rust + Text)

- [x] HTML
- [x] yew (+html_vdom)
- [x] sycamore (+html_dom)
- [ ] pdf
- [ ] ebook
- [ ] zola

### Highlights

- Text(txt)



## Developers



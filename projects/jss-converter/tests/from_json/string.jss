schema _: object {
    $id: "https://example.com/schemas/customer"
    $schema: "https://json-schema.org/draft/2020-12/schema"
}
property _: string {
}
property _: string {
    maxLength: Number(
        3,
    )
    minLength: Number(
        2,
    )
}
property _: string {
}
property _: /schemas/address {
}

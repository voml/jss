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

define Reference: object {
    patternProperties: {
        "^\\$ref$": {
            "type": "string",
            "format": "uri-reference",
        },
    }
    required: [
        "$ref",
    ]
}

define Info: object {
    additionalProperties: false
    required: [
        "title",
        "version",
    ]
    patternProperties: {
        "^x-": {},
    }
    .title: string;
    .description: string;
    .termsOfService: string {
        format: "uri-reference"
    }
    .contact: "#/definitions/Contact";
    .license: "#/definitions/License";
    .version: string;
}

define Contact: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .name: string {
        minLength: 1
    }
    .url: string {
        format: "uri-reference"
    }
    .email: string {
        format: "email"
    }
}

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

define Server: object {
    additionalProperties: false
    required: [
        "url",
    ]
    patternProperties: {
        "^x-": {},
    }
    .url: string {
    }
    .description: string {
    }
    .variables: object {
        additionalProperties: {
            "$ref": "#/definitions/ServerVariable",
        }
    }


}

define ServerVariable: object {
    additionalProperties: false
    required: [
        "default",
    ]
    patternProperties: {
        "^x-": {},
    }
    .enum: array {
        items: {
            "type": "string",
        }
    }


    .default: string {
    }


    .description: string {
    }


}

define Components: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .schemas: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Schema",
                    },
                    {
                        "$ref": "#/definitions/Reference",
                    },
                ],
            },
        }
    }


    .responses: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Response",
                    },
                ],
            },
        }
    }


    .parameters: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Parameter",
                    },
                ],
            },
        }
    }


    .examples: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Example",
                    },
                ],
            },
        }
    }


    .requestBodies: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/RequestBody",
                    },
                ],
            },
        }
    }


    .headers: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Header",
                    },
                ],
            },
        }
    }


    .securitySchemes: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/SecurityScheme",
                    },
                ],
            },
        }
    }


    .links: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Link",
                    },
                ],
            },
        }
    }


    .callbacks: object {
        patternProperties: {
            "^[a-zA-Z0-9\\.\\-_]+$": {
                "oneOf": [
                    {
                        "$ref": "#/definitions/Reference",
                    },
                    {
                        "$ref": "#/definitions/Callback",
                    },
                ],
            },
        }
    }


}

define Schema: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .title: string {
    }


    .multipleOf: number {
        exclusiveMinimum: true
        minimum: 0
    }


    .maximum: number {
    }


    .exclusiveMaximum: "boolean" {
        default: false
    }


    .minimum: number {
    }


    .exclusiveMinimum: "boolean" {
        default: false
    }


    .maxLength: integer {
        minimum: 0
    }


    .minLength: integer {
        default: 0
        minimum: 0
    }


    .pattern: string {
        format: "regex"
    }


    .maxItems: integer {
        minimum: 0
    }


    .minItems: integer {
        default: 0
        minimum: 0
    }


    .uniqueItems: "boolean" {
        default: false
    }


    .maxProperties: integer {
        minimum: 0
    }


    .minProperties: integer {
        default: 0
        minimum: 0
    }


    .required: array {
        uniqueItems: true
        items: {
            "type": "string",
        }
        minItems: 1
    }


    .enum: array {
        uniqueItems: false
        items: {}
        minItems: 1
    }


    .type: string {
        enum: [
            "array",
            "boolean",
            "integer",
            "number",
            "object",
            "string",
        ]
    }


    .not {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .allOf: array {
        items: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Schema",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .oneOf: array {
        items: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Schema",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .anyOf: array {
        items: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Schema",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .items {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .properties: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Schema",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .additionalProperties {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
            {
                "type": "boolean",
            },
        ]
        default: true
    }


    .description: string {
    }


    .format: string {
    }


    .default {
    }


    .nullable: "boolean" {
        default: false
    }


    .discriminator: "#/definitions/Discriminator" {
    }


    .readOnly: "boolean" {
        default: false
    }


    .writeOnly: "boolean" {
        default: false
    }


    .example {
    }


    .externalDocs: "#/definitions/ExternalDocumentation" {
    }


    .deprecated: "boolean" {
        default: false
    }


    .xml: "#/definitions/XML" {
    }


}

define Discriminator: object {
    required: [
        "propertyName",
    ]
    .propertyName: string {
    }


    .mapping: object {
        additionalProperties: {
            "type": "string",
        }
    }


}

define XML: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .name: string {
    }


    .namespace: string {
        format: "uri"
    }


    .prefix: string {
    }


    .attribute: "boolean" {
        default: false
    }


    .wrapped: "boolean" {
        default: false
    }


}

define Response: object {
    additionalProperties: false
    required: [
        "description",
    ]
    patternProperties: {
        "^x-": {},
    }
    .description: string {
    }


    .headers: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Header",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .content: object {
        additionalProperties: {
            "$ref": "#/definitions/MediaType",
        }
    }


    .links: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Link",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


}

define MediaType: object {
    allOf: [
        {
            "$ref": "#/definitions/ExampleXORExamples",
        },
    ]
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .schema {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .example {
    }


    .examples: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Example",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .encoding: object {
        additionalProperties: {
            "$ref": "#/definitions/Encoding",
        }
    }


}

define Example: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .summary: string {
    }


    .description: string {
    }


    .value {
    }


    .externalValue: string {
        format: "uri-reference"
    }


}

define Header: object {
    allOf: [
        {
            "$ref": "#/definitions/ExampleXORExamples",
        },
        {
            "$ref": "#/definitions/SchemaXORContent",
        },
    ]
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .description: string {
    }


    .required: "boolean" {
        default: false
    }


    .deprecated: "boolean" {
        default: false
    }


    .allowEmptyValue: "boolean" {
        default: false
    }


    .style: string {
        default: "simple"
        enum: [
            "simple",
        ]
    }


    .explode: "boolean" {
    }


    .allowReserved: "boolean" {
        default: false
    }


    .schema {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .content: object {
        maxProperties: 1
        additionalProperties: {
            "$ref": "#/definitions/MediaType",
        }
        minProperties: 1
    }


    .example {
    }


    .examples: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Example",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


}

define Paths: object {
    additionalProperties: false
    patternProperties: {
        "^\\/": {
            "$ref": "#/definitions/PathItem",
        },
        "^x-": {},
    }
}

define PathItem: object {
    additionalProperties: false
    patternProperties: {
        "^(get|put|post|delete|options|head|patch|trace)$": {
            "$ref": "#/definitions/Operation",
        },
        "^x-": {},
    }
    ."$ref": string {
    }


    .summary: string {
    }


    .description: string {
    }


    .servers: array {
        items: {
            "$ref": "#/definitions/Server",
        }
    }


    .parameters: array {
        uniqueItems: true
        items: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Parameter",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


}

define Operation: object {
    additionalProperties: false
    required: [
        "responses",
    ]
    patternProperties: {
        "^x-": {},
    }
    .tags: array {
        items: {
            "type": "string",
        }
    }


    .summary: string {
    }


    .description: string {
    }


    .externalDocs: "#/definitions/ExternalDocumentation" {
    }


    .operationId: string {
    }


    .parameters: array {
        uniqueItems: true
        items: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Parameter",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .requestBody {
        oneOf: [
            {
                "$ref": "#/definitions/RequestBody",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .responses: "#/definitions/Responses" {
    }


    .callbacks: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Callback",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


    .deprecated: "boolean" {
        default: false
    }


    .security: array {
        items: {
            "$ref": "#/definitions/SecurityRequirement",
        }
    }


    .servers: array {
        items: {
            "$ref": "#/definitions/Server",
        }
    }


}

define Responses: object {
    additionalProperties: false
    minProperties: 1
    patternProperties: {
        "^[1-5](?:\\d{2}|XX)$": {
            "oneOf": [
                {
                    "$ref": "#/definitions/Response",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        },
        "^x-": {},
    }
    .default {
        oneOf: [
            {
                "$ref": "#/definitions/Response",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


}

define SecurityRequirement: object {
    additionalProperties: {
        "type": "array",
        "items": {
            "type": "string",
        },
    }
}

define Tag: object {
    additionalProperties: false
    required: [
        "name",
    ]
    patternProperties: {
        "^x-": {},
    }
    .name: string {
    }


    .description: string {
    }


    .externalDocs: "#/definitions/ExternalDocumentation" {
    }


}

define ExternalDocumentation: object {
    additionalProperties: false
    required: [
        "url",
    ]
    patternProperties: {
        "^x-": {},
    }
    .description: string {
    }


    .url: string {
        format: "uri-reference"
    }


}

/// Example and examples are mutually exclusive
define ExampleXORExamples {
    not: {
        "required": [
            "example",
            "examples",
        ],
    }
}

/// Schema and content are mutually exclusive, at least one is required
define SchemaXORContent {
    oneOf: [
        {
            "required": [
                "schema",
            ],
        },
        {
            "required": [
                "content",
            ],
            "description": "Some properties are not allowed if content is present",
            "allOf": [
                {
                    "not": {
                        "required": [
                            "style",
                        ],
                    },
                },
                {
                    "not": {
                        "required": [
                            "explode",
                        ],
                    },
                },
                {
                    "not": {
                        "required": [
                            "allowReserved",
                        ],
                    },
                },
                {
                    "not": {
                        "required": [
                            "example",
                        ],
                    },
                },
                {
                    "not": {
                        "required": [
                            "examples",
                        ],
                    },
                },
            ],
        },
    ]
    not: {
        "required": [
            "schema",
            "content",
        ],
    }
}

define Parameter: object {
    allOf: [
        {
            "$ref": "#/definitions/ExampleXORExamples",
        },
        {
            "$ref": "#/definitions/SchemaXORContent",
        },
        {
            "$ref": "#/definitions/ParameterLocation",
        },
    ]
    required: [
        "name",
        "in",
    ]
    patternProperties: {
        "^x-": {},
    }
    additionalProperties: false
    .name: string {
    }


    .in: string {
    }


    .description: string {
    }


    .required: "boolean" {
        default: false
    }


    .deprecated: "boolean" {
        default: false
    }


    .allowEmptyValue: "boolean" {
        default: false
    }


    .style: string {
    }


    .explode: "boolean" {
    }


    .allowReserved: "boolean" {
        default: false
    }


    .schema {
        oneOf: [
            {
                "$ref": "#/definitions/Schema",
            },
            {
                "$ref": "#/definitions/Reference",
            },
        ]
    }


    .content: object {
        maxProperties: 1
        additionalProperties: {
            "$ref": "#/definitions/MediaType",
        }
        minProperties: 1
    }


    .example {
    }


    .examples: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Example",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }


}

/// Parameter location
define ParameterLocation {
    oneOf: [
        {
            "description": "Parameter in path",
            "required": [
                "required",
            ],
            "properties": {
                "in": {
                    "enum": [
                        "path",
                    ],
                },
                "style": {
                    "enum": [
                        "matrix",
                        "label",
                        "simple",
                    ],
                    "default": "simple",
                },
                "required": {
                    "enum": [
                        true,
                    ],
                },
            },
        },
        {
            "description": "Parameter in query",
            "properties": {
                "in": {
                    "enum": [
                        "query",
                    ],
                },
                "style": {
                    "enum": [
                        "form",
                        "spaceDelimited",
                        "pipeDelimited",
                        "deepObject",
                    ],
                    "default": "form",
                },
            },
        },
        {
            "description": "Parameter in header",
            "properties": {
                "in": {
                    "enum": [
                        "header",
                    ],
                },
                "style": {
                    "enum": [
                        "simple",
                    ],
                    "default": "simple",
                },
            },
        },
        {
            "description": "Parameter in cookie",
            "properties": {
                "in": {
                    "enum": [
                        "cookie",
                    ],
                },
                "style": {
                    "enum": [
                        "form",
                    ],
                    "default": "form",
                },
            },
        },
    ]
}

define RequestBody: object {
    additionalProperties: false
    required: [
        "content",
    ]
    patternProperties: {
        "^x-": {},
    }
    .description: string {
    }


    .content: object {
        additionalProperties: {
            "$ref": "#/definitions/MediaType",
        }
    }


    .required: "boolean" {
        default: false
    }


}

define SecurityScheme {
    oneOf: [
        {
            "$ref": "#/definitions/APIKeySecurityScheme",
        },
        {
            "$ref": "#/definitions/HTTPSecurityScheme",
        },
        {
            "$ref": "#/definitions/OAuth2SecurityScheme",
        },
        {
            "$ref": "#/definitions/OpenIdConnectSecurityScheme",
        },
    ]
}

define APIKeySecurityScheme: object {
    additionalProperties: false
    required: [
        "type",
        "name",
        "in",
    ]
    patternProperties: {
        "^x-": {},
    }
    .type: string {
        enum: [
            "apiKey",
        ]
    }


    .name: string {
    }


    .in: string {
        enum: [
            "header",
            "query",
            "cookie",
        ]
    }


    .description: string {
    }


}

define HTTPSecurityScheme: object {
    oneOf: [
        {
            "description": "Bearer",
            "properties": {
                "scheme": {
                    "type": "string",
                    "pattern": "^[Bb][Ee][Aa][Rr][Ee][Rr]$",
                },
            },
        },
        {
            "description": "Non Bearer",
            "not": {
                "required": [
                    "bearerFormat",
                ],
            },
            "properties": {
                "scheme": {
                    "not": {
                        "type": "string",
                        "pattern": "^[Bb][Ee][Aa][Rr][Ee][Rr]$",
                    },
                },
            },
        },
    ]
    required: [
        "scheme",
        "type",
    ]
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .scheme: string {
    }


    .bearerFormat: string {
    }


    .description: string {
    }


    .type: string {
        enum: [
            "http",
        ]
    }


}

define OAuth2SecurityScheme: object {
    additionalProperties: false
    required: [
        "type",
        "flows",
    ]
    patternProperties: {
        "^x-": {},
    }
    .type: string {
        enum: [
            "oauth2",
        ]
    }


    .flows: "#/definitions/OAuthFlows" {
    }


    .description: string {
    }


}

define OpenIdConnectSecurityScheme: object {
    additionalProperties: false
    required: [
        "type",
        "openIdConnectUrl",
    ]
    patternProperties: {
        "^x-": {},
    }
    .type: string {
        enum: [
            "openIdConnect",
        ]
    }


    .openIdConnectUrl: string {
        format: "uri-reference"
    }


    .description: string {
    }


}

define OAuthFlows: object {
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .implicit: "#/definitions/ImplicitOAuthFlow" {
    }


    .password: "#/definitions/PasswordOAuthFlow" {
    }


    .clientCredentials: "#/definitions/ClientCredentialsFlow" {
    }


    .authorizationCode: "#/definitions/AuthorizationCodeOAuthFlow" {
    }


}

define ImplicitOAuthFlow: object {
    additionalProperties: false
    required: [
        "authorizationUrl",
        "scopes",
    ]
    patternProperties: {
        "^x-": {},
    }
    .authorizationUrl: string {
        format: "uri-reference"
    }


    .refreshUrl: string {
        format: "uri-reference"
    }


    .scopes: object {
        additionalProperties: {
            "type": "string",
        }
    }


}

define PasswordOAuthFlow: object {
    additionalProperties: false
    required: [
        "tokenUrl",
        "scopes",
    ]
    patternProperties: {
        "^x-": {},
    }
    .tokenUrl: string {
        format: "uri-reference"
    }


    .refreshUrl: string {
        format: "uri-reference"
    }


    .scopes: object {
        additionalProperties: {
            "type": "string",
        }
    }


}

define ClientCredentialsFlow: object {
    additionalProperties: false
    required: [
        "tokenUrl",
        "scopes",
    ]
    patternProperties: {
        "^x-": {},
    }
    .tokenUrl: string {
        format: "uri-reference"
    }


    .refreshUrl: string {
        format: "uri-reference"
    }


    .scopes: object {
        additionalProperties: {
            "type": "string",
        }
    }


}

define AuthorizationCodeOAuthFlow: object {
    additionalProperties: false
    required: [
        "authorizationUrl",
        "tokenUrl",
        "scopes",
    ]
    patternProperties: {
        "^x-": {},
    }
    .authorizationUrl: string {
        format: "uri-reference"
    }


    .tokenUrl: string {
        format: "uri-reference"
    }


    .refreshUrl: string {
        format: "uri-reference"
    }


    .scopes: object {
        additionalProperties: {
            "type": "string",
        }
    }


}

define Link: object {
    not: {
        "description": "Operation Id and Operation Ref are mutually exclusive",
        "required": [
            "operationId",
            "operationRef",
        ],
    }
    additionalProperties: false
    patternProperties: {
        "^x-": {},
    }
    .operationId: string {
    }


    .operationRef: string {
        format: "uri-reference"
    }


    .parameters: object {
        additionalProperties: {}
    }


    .requestBody {
    }


    .description: string {
    }


    .server: "#/definitions/Server" {
    }


}

define Callback: object {
    patternProperties: {
        "^x-": {},
    }
    additionalProperties: {
        "$ref": "#/definitions/PathItem",
    }
}

define Encoding: object {
    additionalProperties: false
    .contentType: string {
    }
    .headers: object {
        additionalProperties: {
            "oneOf": [
                {
                    "$ref": "#/definitions/Header",
                },
                {
                    "$ref": "#/definitions/Reference",
                },
            ],
        }
    }
    .style: string {
        enum: [
            "form",
            "spaceDelimited",
            "pipeDelimited",
            "deepObject",
        ]
    }
    .explode: "boolean" {
    }
    .allowReserved: "boolean" {
        default: false
    }
}

property openapi: string {
    pattern: "^3\\.0\\.\\d(-.+)?$"
}

property info: "#/definitions/Info" {
}

property externalDocs: "#/definitions/ExternalDocumentation" {
}

property servers: array {
    items: {
        "$ref": "#/definitions/Server",
    }
}

property security: array {
    items: {
        "$ref": "#/definitions/SecurityRequirement",
    }
}

property tags: array {
    uniqueItems: true
    items: {
        "$ref": "#/definitions/Tag",
    }
}

property paths: "#/definitions/Paths" {
}

property components: "#/definitions/Components" {
}


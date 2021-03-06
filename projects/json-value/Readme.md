# Json Helper Functions

- documentation: [docs.rs/json-value](https://docs.rs/json-value)

## Methods

| Function    | Description                                                                      |
|:------------|:---------------------------------------------------------------------------------|
| `get_key_T` | Get a reference to the specified key, or a reference if successful               |
| `mut_key_T` | Get mutable reference to the specified key, or a mutable reference if successful |
| `extract_T` | Extract the specified field, if successful, the field will disappear             |
| `is_T`      | Check if the object is the specified type                                        |
| `as_T`      | Get a reference of the specified type, if successful, get a reference            |
| `into_T`    | Convert to the specified type, if the failure data will disappear                |
# henze-ds
Application for retrieving Henze-compliant betting odds from Danske Spil

## Approach

### Generating Rust Types

```mermaid
flowchart LR
    A[**Danske Spil** Endpoint] --> |JSON|B(**Quicktype**)
    B -->|JSON Schema| C(**Typify**)
    C --> D[**Rust Types**]
```

Rust types are defined for the JSON objects fetched via the Danske Spil API in a semi-automatic manner. Parsing the API responses to Rust types rather than manipulating the JSON directly makes for more readable and maintainable code. The JSON returned by the Danske Spil API is converted to a JSON Schema using [Quicktype](https://quicktype.io/), and this schema is in turn input to [Typify](https://github.com/oxidecomputer/typify) to generate corresponding [Rust types](./henze-ds/src/ds_client/schema.rs).
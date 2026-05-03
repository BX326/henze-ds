# henze-ds

A Rust workspace for retrieving and surfacing Henze-compliant betting odds from the Danske Spil API.

## Components

- **[henze-ds](./henze-ds)** - Core library; fetches odds from the Danske Spil API and deserialises them into typed Rust structs generated via [Typify](https://github.com/oxidecomputer/typify)
- **[henze-ds-cli](./henze-ds-cli)** - Command-line interface for querying odds directly
- **[henze-ds-discord](./henze-ds-discord)** - Discord bot that posts a daily selection of Henze bets to a configured channel. Supports AI-assisted bet selection via OpenAI
- **[henze-ds-web](./henze-ds-web)** - Web frontend built with Rocket and Tera templates for browsing odds in a browser
- **[schema-flex](./schema-flex)** - Utility for post-processing the Quicktype-generated JSON schema before type generation. Removes `additionalProperties: false` and converts string enums to plain strings so that unexpected API fields and values are tolerated at runtime

## Regenerating Rust Types

The types in [henze-ds/src/ds_client/schema.rs](./henze-ds/src/ds_client/schema.rs) are generated from the Danske Spil API response. To regenerate them:

```bash
# 1. Fetch fresh JSON from the API and generate a schema
quicktype https://content.sb.danskespil.dk/... -l schema -o henze-ds/src/ds_client/schema.json

# 2. Make the schema flexible
cargo run -p schema-flex -- henze-ds/src/ds_client/schema.json

# 3. Rebuild to regenerate Rust types via Typify
cargo build
```

## Deployment

- Oracle VM multi-instance Discord bot deployment guide: [henze-ds-discord/deploy/oracle/README.md](./henze-ds-discord/deploy/oracle/README.md)
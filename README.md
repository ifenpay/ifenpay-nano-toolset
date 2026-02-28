# nano-toolset

Autonomous AI-to-AI Nano payment toolset with MCP and HTTP APIs for machine-native transactions.

## Why this exists

`nano-toolset` gives agents a deterministic way to execute payment actions without framework lock-in.

## Core capabilities

- Wallet balance retrieval
- Direct Nano transfers
- Payment request creation
- Payment status lookup
- Credits and donation endpoints
- Consistent API envelope (`success`, `data`, `error`)

## Integration modes

1. **MCP over stdio**
   - Methods: `initialize`, `tools/list`, `tools/call`
   - Best for MCP-native agent frameworks

2. **HTTP API**
   - Endpoints under `/wallet`, `/payment`, `/credits`, `/donate`
   - Best for lightweight/custom clients and non-MCP stacks

## Quick start

```bash
cargo run --release
```

You start the plugin process. MCP starts automatically inside the same process.

## Documentation

- `documentation/README.md` — docs index
- `documentation/mcp.md` — MCP integration guide
- `documentation/openapi.yaml` — HTTP API contract
- `documentation/errors.md` — canonical error catalog
- `documentation/ATTRIBUTION.md` — third-party attribution

## Source of truth

- Runtime error enums: `src/enums/ifenpay/errors/`
- Keep `documentation/errors.md` in sync with those enums

## Terms of Use

Use of this toolset is at your own responsibility.

- Validate fit, risk controls, and legal/compliance requirements in your own environment.
- Production deployment decisions (monitoring, key management, incident response, access control) remain your responsibility.

Full terms: `TERMS_OF_USE.md`

## License

- Third-party attributions and license notices: `documentation/ATTRIBUTION.md`
- Third-party bundled notices: `THIRD-PARTY-LICENSE`

Repository license file: `LICENSE`

If you need explicit redistribution or commercial usage terms for this repository itself, refer to the repository owner’s policy.

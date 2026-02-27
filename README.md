# nano-toolset

Framework-agnostic Nano payment toolset for human and autonomous-agent integrations.

## What this project provides

- MCP server over `stdio` (embedded in the plugin binary)
- HTTP API toolset for non-MCP clients
- Wallet, payment, credits, and donation operations
- Deterministic response envelope:

```json
{
  "success": true,
  "data": {},
  "error": null
}
```

## Integration options

1. **MCP (stdio)**
   - Use MCP methods: `initialize`, `tools/list`, `tools/call`
   - Best for MCP-native agent frameworks

2. **HTTP API**
   - Use routes under `/wallet`, `/payment`, `/credits`, `/donate`
   - Best for lightweight or custom clients

## Startup

You start the plugin process. MCP starts automatically inside that process.

Example:

```bash
cargo run --release
```

## Documentation

See the full documentation in the `documentation/` folder:

- `documentation/README.md` — documentation index
- `documentation/mcp.md` — MCP integration guide
- `documentation/openapi.yaml` — HTTP API contract
- `documentation/errors.md` — canonical error catalog
- `documentation/ATTRIBUTION.md` — third-party attribution

## Source of truth for error enums

Runtime error enums are maintained in:

- `src/enums/ifenpay/errors/`

The docs in `documentation/errors.md` should stay in sync with those enums.

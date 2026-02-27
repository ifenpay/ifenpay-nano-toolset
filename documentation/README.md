# nano-toolset Documentation

This folder contains the integration contract for both human developers and autonomous agents.

## What to read first

- `mcp.md` — MCP integration details, supported methods, tools, and examples.
- `openapi.yaml` — HTTP API contract for tool endpoints.
- `errors.md` — Canonical error catalog and handling guidance.

## Integration modes

The plugin supports two integration modes:

1. **MCP over stdio**
   - Methods: `initialize`, `tools/list`, `tools/call`
   - Best for agent frameworks with MCP-native support.

2. **HTTP API**
   - Endpoints under `/wallet`, `/payment`, `/credits`, `/donate`
   - Best for lightweight clients and framework-agnostic integrations.

## Runtime notes

- The plugin manages wallet operations internally.
- Responses use a consistent envelope with `success`, `data`, and `error`.
- Monetary values in this toolset are Nano units (`NANO`) unless explicitly documented otherwise.

# MCP Integration Guide

This plugin exposes a **Model Context Protocol (MCP)** server over `stdio`.
The MCP server is embedded in the main binary and runs together with the HTTP API process.

## Startup behavior

- You start the plugin process yourself (for example with cargo run --release).
- MCP starts automatically inside that plugin process.
- No separate MCP bootstrap command is required.

## Supported MCP methods

- `initialize`
- `tools/list`
- `tools/call`

## Available tools

- `wallet.balance`
- `wallet.send`
- `payment.request`
- `payment.status`
- `credits.get`
- `credits.topup`
- `donate.send`

## Standard response envelope

All tool responses follow this structure:

```json
{
  "success": true,
  "data": {},
  "error": null
}
```

Error responses use the same envelope with `success: false` and a populated `error` object.

## Tool input schemas

### wallet.balance

No input fields.

### wallet.send

```json
{
  "recipient_address": "nano_...",
  "amount": "0.1"
}
```

### payment.request

```json
{
  "receive_address": "nano_...",
  "amount": "1.5",
  "redirect_url": null
}
```

### payment.status

```json
{
  "transaction_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
}
```

### credits.get

No input fields.

### credits.topup

```json
{
  "credits_amount": 1000
}
```

### donate.send

```json
{
  "amount": "0.05"
}
```

## Minimal MCP flow example

1. Call `initialize`.
2. Call `tools/list` and cache tool schemas.
3. Call `tools/call` with validated arguments.
4. Parse the envelope and branch on `success`.

## Agent integration guidance

- Validate required arguments before `tools/call`.
- Treat `error.error` as a stable machine code.
- Use `documentation/errors.md` as the canonical error behavior reference.
- Assume amounts are Nano-denominated (`NANO`) unless stated otherwise by the API response.

# Error Catalog

This file is the canonical error reference for both humans and autonomous agents.

## Error envelope

All failures use this JSON shape:

```json
{
  "success": false,
  "data": null,
  "error": {
    "error": "ERROR_CODE",
    "message": "Human readable message"
  }
}
```

## Coverage parity with `src/enums/ifenpay/errors/`

This catalog includes all upstream-facing error codes defined in:

- `src/enums/ifenpay/errors/account.rs`
- `src/enums/ifenpay/errors/auth.rs`
- `src/enums/ifenpay/errors/credit.rs`
- `src/enums/ifenpay/errors/database.rs`
- `src/enums/ifenpay/errors/rpc.rs`
- `src/enums/ifenpay/errors/transaction.rs`
- `src/enums/ifenpay/errors/websocket.rs`

If a new code is added in any of those files, update this document in the same change.

## How clients should handle errors

1. Branch on `success` first.
2. If `success` is `false`, use `error.error` as the machine decision key.
3. Treat `error.message` as display/debug text, not routing logic.

## Core plugin errors

### API adapter errors (HTTP 502)
- `REQUEST_ERROR` — Upstream request failed.
- `PARSE_ERROR` — Upstream response could not be parsed.
- `INVALID_DATA` — Upstream payload did not match required data.

### Account validation (HTTP 400)
- `INVALID_ADDRESS` — Address must begin with `nano_` or `xrb_`.

### Work server errors (HTTP 502)
- `WORK_SERVER_ERROR` — PoW/work server operation failed.

### Transaction validation (HTTP 500)
- `INVALID_TRANSACTION_ID`
- `INVALID_NEGATIVE_AMOUNT`
- `INVALID_NUMBER_FORMAT`
- `INVALID_WHOLE_NUMBER`
- `TOO_MANY_DECIMAL_PLACES`
- `INVALID_FRACTIONAL_PART`
- `AMOUNT_TOO_LARGE`
- `INSUFFICIENT_FUNDS`

### Credits validation (HTTP 500)
- `INVALID_CREDITS_AMOUNT` — Allowed tiers: `10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000`.

### Block/signing/work internals (HTTP 500)
- `KEY_DERIVATION_FAILED`
- `INVALID_PREVIOUS_HASH`
- `INVALID_REPRESENTATIVE_PUBLIC_KEY`
- `INVALID_LINK`
- `BLOCK_HASH_GENERATION_FAILED`
- `SIGNING_FAILED`
- `INVALID_REPRESENTATIVE_ADDRESS`
- `INVALID_WORK_HEX`
- `INVALID_WORK_LENGTH`
- `INVALID_WORK_ROOT`
- `INVALID_WORK_ROOT_LENGTH`
- `CALCULATE_WORK_FAILED`

## Common upstream service errors

These may be passed through from upstream IFENPAY services:

### Auth/rate-limit (plugin maps to HTTP 401)
- `FORBIDDEN`
- `INVALID_API_KEY`
- `MISSING_API_KEY`
- `PAYLOAD_TOO_LARGE`
- `RATE_LIMIT_EXCEEDED`

### Credits/payment backends (typically HTTP 500)
- `TRANSACTION_CREATION_FAILED`
- `PRICE_CALCULATION_ERROR`
- `TRANSACTION_NOT_CONFIRMED`
- `NO_TRANSACTION_FOUND`
- `API_KEY_NOT_FOUND`

### Storage/transport/runtime backends (typically HTTP 500)
- `DATABASE_SELECT_ERROR`
- `DATABASE_ERROR`
- `DATABASE_UPDATE_ERROR`
- `RPC_READ_ERROR`
- `RPC_RESPONSE_ERROR`
- `RPC_PARSE_ERROR`
- `RPC_REQUEST_ERROR`
- `WEBSOCKET_ADD_ACCOUNT_ERROR`
- `WEBSOCKET_REMOVE_ACCOUNT_ERROR`
- `WEBSOCKET_NOT_INITIALIZED`

## Recommended fallback behavior for autonomous agents

- Retry only on transient connectivity failures (`REQUEST_ERROR`, network timeout).
- Do not retry validation errors (`INVALID_*`, `INSUFFICIENT_FUNDS`) without changing input.
- Surface exact `error.error` to logs/trace for deterministic debugging.

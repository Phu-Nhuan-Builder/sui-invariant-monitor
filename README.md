# Sui Invariant Monitor

Protocol-level invariant & safety monitoring for Sui smart contracts.

> A runtime safety monitor that detects invariant violations even when transactions succeed.

## Overview

This tool provides continuous off-chain monitoring of Sui protocol state, recomputing critical invariants and alerting when violations are detected.

### Key Features

- **5 High-Impact Invariants** monitored continuously
- **Discord/Webhook Alerts** on violations
- **REST API** for integration and UI
- **Developer-focused UI** for demos and inspection

## Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   Sui Network   │────▶│  Rust Backend   │
└─────────────────┘     │  - State Fetch  │
                        │  - Invariants   │
                        │  - Alerting     │
                        │  - REST API     │
                        └────────┬────────┘
                                 │
                        ┌────────▼────────┐
                        │ React Frontend  │
                        │  - Overview     │
                        │  - Details      │
                        └─────────────────┘
```

## Invariants Monitored

| ID | Name | Description |
|----|------|-------------|
| INV-001 | Total Supply Conservation | Supply = Reserves + Borrowed |
| INV-002 | Collateralization Ratio | Borrows must be 150% collateralized |
| INV-003 | Accounting Balance Integrity | Internal balance = On-chain balance |
| INV-004 | Interest Index Monotonicity | Interest index never decreases |
| INV-005 | Liquidity Constraint | Borrowed ≤ Supply |

## Quick Start

### Backend

```bash
cd backend
cp .env.example .env
# Edit .env with your Sui RPC URL and object IDs
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

Open http://localhost:5173 to view the monitor.

## Deployment

### Backend (Fly.io)

```bash
cd backend
fly deploy

# Set secrets
fly secrets set DISCORD_WEBHOOK_URL=your-webhook
fly secrets set MONITORED_OBJECT_IDS=0x123,0x456
```

### Frontend

Build and deploy to any static host:

```bash
cd frontend
VITE_API_URL=https://your-backend.fly.dev npm run build
# Deploy dist/ folder
```

## Configuration

See [backend/README.md](backend/README.md) and [frontend/README.md](frontend/README.md) for detailed configuration.

## Why Web3-Native?

1. **On-chain state is the source of truth** - Fetches directly from Sui RPC
2. **Successful txns can violate invariants** - Catches issues the VM doesn't
3. **Cross-object invariants** - Monitors relationships Move can't express
4. **Continuous surveillance** - Runs without on-chain triggers or gas costs

## License

MIT

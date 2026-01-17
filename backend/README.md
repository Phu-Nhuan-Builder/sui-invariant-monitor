# Sui Invariant Monitor - Backend

Protocol-level invariant monitoring for Sui smart contracts.

## Prerequisites

- Rust 1.75+
- Sui RPC endpoint

## Quick Start

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# - Set SUI_RPC_URL to your Sui RPC endpoint
# - Set MONITORED_OBJECT_IDS to comma-separated object IDs
# - Optionally set DISCORD_WEBHOOK_URL for alerts

# Run in development
cargo run

# Run tests
cargo test
```

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `SUI_RPC_URL` | Sui RPC endpoint | `https://fullnode.mainnet.sui.io:443` |
| `POLLING_INTERVAL_SECS` | Check frequency | `10` |
| `WEBHOOK_URL` | Generic webhook for alerts | - |
| `DISCORD_WEBHOOK_URL` | Discord alerts | - |
| `MONITORED_OBJECT_IDS` | Comma-separated object IDs | - |
| `PORT` | API server port | `8080` |

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/status` | Overall monitoring status |
| GET | `/api/invariants` | List all invariants |
| GET | `/api/invariants/:id` | Get specific invariant |

## Invariants

1. **INV-001: Total Supply Conservation** - Supply equals reserves + borrows
2. **INV-002: Collateralization Ratio** - Borrows must be 150% collateralized
3. **INV-003: Accounting Balance Integrity** - Internal balance matches on-chain
4. **INV-004: Interest Index Monotonicity** - Interest index never decreases
5. **INV-005: Liquidity Constraint** - Borrowed cannot exceed supply

## Deployment

```bash
# Deploy to Fly.io
fly deploy

# Set secrets
fly secrets set DISCORD_WEBHOOK_URL=your-webhook-url
fly secrets set MONITORED_OBJECT_IDS=0x123,0x456
```

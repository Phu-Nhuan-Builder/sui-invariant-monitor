# Sui Invariant Monitor - Backend 🦀

Rust backend server for the Sui Invariant Monitor. Provides REST API for AI-powered contract analysis and real-time invariant monitoring.

## 🌐 Production

- **API Base URL**: https://combo-ripe-nat-occur.trycloudflare.com
- **Health Check**: https://combo-ripe-nat-occur.trycloudflare.com/health
- **Status**: ✅ Running 24/7 on VPS with Supervisor

## 🏗️ Architecture

```
Cloudflare Tunnel (HTTPS)
        ↓
   VPS Backend
   Port: 7681
        ↓
   Supervisor
   (Auto-restart)
        ↓
Sui RPC (Mainnet/Testnet)
```

## ✨ Features

- **AI Analysis**: OpenRouter & Ollama integration for Move module analysis
- **Dynamic Network**: Mainnet/Testnet switching via API parameter
- **Real-time Monitoring**: 10-second polling of protocol state
- **CORS Enabled**: Cross-origin requests from frontend
- **Health Checks**: `/health` endpoint for monitoring
- **Discord Webhooks**: Violation notifications

## 🚀 Quick Start

### Prerequisites

- Rust 1.83+
- OpenSSL development libraries

### Installation

```bash
# Clone repository
git clone https://github.com/phunhuanbuilder/sui-invariant-monitor.git
cd sui-invariant-monitor/backend

# Create .env file
cp .env.example .env

# Edit .env with your configuration
nano .env

# Build and run
cargo run
```

Server starts on `http://localhost:8080`

### Environment Variables

```env
# Sui RPC URL (fallback, API accepts network parameter)
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Server port
PORT=8080

# Logging level (trace, debug, info, warn, error)
RUST_LOG=info

# Polling interval for monitoring (seconds)
POLLING_INTERVAL_SECS=10

# Discord webhook for violation alerts
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/YOUR_WEBHOOK_URL

# Optional: Monitored object IDs (comma-separated)
MONITORED_OBJECT_IDS=

# Optional: Generic webhook URL
WEBHOOK_URL=
```

## 📡 API Reference

### Health & Status

#### GET `/health`
Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "uptime_secs": 12345
}
```

#### GET `/api/status`
Get current monitoring status.

**Response:**
```json
{
  "last_check": "2026-01-18T06:45:12Z",
  "violations": 0,
  "total_invariants": 5,
  "all_ok": true,
  "monitored_objects": []
}
```

### Contract Analysis

#### POST `/api/analyze`
Analyze a Sui package using AI.

**Request:**
```json
{
  "package_id": "0x2",
  "module_name": "coin",
  "llm_provider": "openrouter",
  "api_key": "sk-or-...",
  "model": "anthropic/claude-opus-4.5",
  "network": "mainnet"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Analysis complete",
  "modules": [...],
  "analysis_results": [
    {
      "module_name": "coin",
      "suggested_invariants": [
        {
          "id": "coin_total_supply",
          "name": "Total Supply Conservation",
          "description": "...",
          "severity": "critical",
          "formula": "..."
        }
      ]
    }
  ]
}
```

#### GET `/api/metadata/:package_id/:module_name`
Get metadata for a specific module.

**Query Parameters:**
- `network` (optional): "mainnet" or "testnet"

**Response:**
```json
{
  "package_id": "0x2",
  "module_name": "coin",
  "structs": [...],
  "functions": [...]
}
```

### Invariant Management

#### GET `/api/invariants`
List all monitored invariants.

**Response:**
```json
[
  {
    "id": "total_supply",
    "name": "Total Supply Conservation",
    "description": "...",
    "status": "Ok",
    "last_check": "2026-01-18T06:45:12Z",
    "computation": {...}
  }
]
```

#### GET `/api/invariants/:id`
Get specific invariant details.

#### POST `/api/invariants/add`
Add suggested invariants to monitoring.

**Request:**
```json
{
  "invariants": [
    {
      "id": "coin_total_supply",
      "name": "Total Supply Conservation",
      "description": "...",
      "severity": "critical",
      "formula": "...",
      "package_id": "0x2",
      "module_name": "coin"
    }
  ]
}
```

#### POST `/api/invariants/remove`
Remove invariant from monitoring.

**Request:**
```json
{
  "id": "total_supply"
}
```

### Object Monitoring

#### POST `/api/monitor`
Add object ID to monitoring.

**Request:**
```json
{
  "object_id": "0x123...",
  "network": "mainnet"
}
```

## 🔧 Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run

```bash
# Development mode (with logging)
RUST_LOG=debug cargo run

# Watch mode (auto-reload)
cargo watch -x run
```

### Test

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Lint & Format

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint
cargo clippy

# Fix lints
cargo clippy --fix
```

## 📦 Deployment

### VPS Deployment (Production)

See [`DEPLOY_STEPS.md`](../DEPLOY_STEPS.md) for detailed VPS deployment guide.

**Quick steps:**
1. SSH into VPS
2. Install Rust 1.83+
3. Clone repository
4. Create `.env` file
5. Build release: `cargo build --release`
6. Setup Supervisor service
7. Setup Cloudflare Tunnel for HTTPS

### Docker Deployment

```bash
# Build image
docker build -t sui-monitor-backend .

# Run container
docker run -p 8080:8080 \
  -e SUI_RPC_URL=https://fullnode.mainnet.sui.io:443 \
  -e PORT=8080 \
  sui-monitor-backend
```

### Render.com Deployment

See [`RENDER_DEPLOYMENT.md`](../RENDER_DEPLOYMENT.md) for Render deployment guide.

## 🛠️ Tech Stack

- **Language**: Rust 1.83
- **Web Framework**: Axum 0.7
- **HTTP Client**: Reqwest 0.12 (HTTP/1.1 for OpenRouter)
- **Serialization**: Serde
- **Async Runtime**: Tokio
- **CORS**: Tower-HTTP
- **Logging**: Tracing

## 📁 Project Structure

```
backend/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error types
│   ├── network.rs           # Network resolver
│   ├── api/
│   │   ├── mod.rs           # API module
│   │   ├── routes.rs        # Route definitions
│   │   └── handlers.rs      # Request handlers
│   ├── analysis/
│   │   ├── mod.rs
│   │   ├── llm.rs           # LLM clients (OpenRouter/Ollama)
│   │   └── metadata.rs      # Sui metadata fetching
│   ├── invariants/
│   │   ├── mod.rs
│   │   ├── engine.rs        # Invariant evaluation engine
│   │   ├── types.rs         # Invariant types
│   │   └── definitions/     # Invariant implementations
│   ├── sui_client/
│   │   └── mod.rs           # Sui RPC client
│   ├── aggregator/
│   │   ├── mod.rs
│   │   └── state.rs         # State aggregation
│   └── alerting/
│       ├── mod.rs
│       ├── discord.rs       # Discord webhooks
│       └── webhook.rs       # Generic webhooks
├── Cargo.toml               # Dependencies
├── Dockerfile               # Docker build
└── .env.example             # Environment template
```

## 🔒 Security

- **HTTPS**: Production uses Cloudflare Tunnel
- **CORS**: Configured to allow frontend origin
- **Environment Variables**: Secrets in `.env`, not committed
- **Input Validation**: Package IDs and module names validated
- **Timeouts**: 120s timeout for LLM requests

## 📊 Monitoring

### Logs (VPS)

```bash
# SSH into VPS
ssh -p 1443 root@n2.ckey.vn

# View logs
supervisorctl tail -f sui-monitor

# Or direct file
tail -f /var/log/sui-monitor.log
```

### Supervisor Commands

```bash
# Status
supervisorctl status

# Restart
supervisorctl restart sui-monitor

# Stop
supervisorctl stop sui-monitor

# Start
supervisorctl start sui-monitor
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- Built for: First Movers Sprint 2026

---

© 2026 Phú Nhuận Builder

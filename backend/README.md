# Sui Invariant Monitor - Backend

Rust backend service for the Sui Invariant Monitor. Provides REST API for analyzing smart contracts and monitoring protocol invariants.

## 🏗️ Architecture

```
backend/
├── src/
│   ├── main.rs                    # Entry point & service loop
│   ├── config.rs                  # Environment configuration
│   ├── error.rs                   # Error types
│   │
│   ├── api/                       # REST API layer
│   │   ├── routes.rs             # Route definitions
│   │   ├── handlers.rs           # Request handlers
│   │   └── mod.rs
│   │
│   ├── analysis/                  # AI analysis module
│   │   ├── metadata.rs           # Sui RPC metadata fetcher
│   │   ├── llm.rs                # LLM client (OpenRouter/Ollama)
│   │   └── mod.rs
│   │
│   ├── invariants/                # Invariant system
│   │   ├── engine.rs             # Evaluation engine
│   │   ├── types.rs              # Core types
│   │   ├── definitions/          # Invariant implementations
│   │   └── mod.rs
│   │
│   ├── sui_client/                # Sui blockchain integration
│   │   ├── fetcher.rs            # Direct RPC calls
│   │   └── mod.rs
│   │
│   ├── aggregator/                # State aggregation
│   │   ├── state.rs              # Protocol state builder
│   │   └── mod.rs
│   │
│   └── alerting/                  # Alert system
│       ├── discord.rs            # Discord webhooks
│       ├── webhook.rs            # Generic webhooks
│       └── mod.rs
│
├── Cargo.toml
├── .env.example
└── Dockerfile
```

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.75 or later
- **Sui RPC endpoint** (Mainnet or Testnet)
- (Optional) **Ollama** for local AI models

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd backend

# Copy environment template
cp .env.example .env

# Edit .env file
nano .env
```

### Environment Configuration

```env
# Sui Network
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Server Configuration
PORT=8080
POLLING_INTERVAL_SECS=10

# Monitoring (optional)
MONITORED_OBJECT_IDS=

# Alerting (optional)
WEBHOOK_URL=
DISCORD_WEBHOOK_URL=
```

### Running the Server

```bash
# Development
cargo run

# Production build
cargo build --release
./target/release/sui-invariant-monitor

# With auto-reload
cargo watch -x run
```

The server will start on `http://localhost:8080`

## 📡 API Reference

### Health & Status

#### GET /health
Health check endpoint
```json
{
  "status": "ok",
  "uptime_secs": 12345
}
```

#### GET /api/status
Get monitoring status
```json
{
  "last_check": "2026-01-18T00:00:00Z",
  "violations": 0,
  "total_invariants": 5,
  "all_ok": true,
  "monitored_objects": ["0x..."]
}
```

### Invariants

#### GET /api/invariants
List all monitored invariants
```json
[
  {
    "id": "INV-001",
    "name": "Balance Conservation",
    "description": "...",
    "status": "Ok",
    "evaluated_at": "2026-01-18T00:00:00Z",
    "computation": {...},
    "violation_reason": null
  }
]
```

#### GET /api/invariants/:id
Get specific invariant details

#### POST /api/invariants/add
Add suggested invariants to monitoring
```json
{
  "invariants": [...],
  "package_id": "0x2",
  "module_name": "sui"
}
```

#### POST /api/invariants/remove
Remove invariant from monitoring
```json
{
  "invariant_id": "INV-001"
}
```

### AI Analysis

#### POST /api/analyze
Analyze a package with AI
```json
{
  "package_id": "0x2",
  "module_name": "coin",
  "llm_provider": "ollama",
  "model": "llama3.2",
  "ollama_url": "http://localhost:11434"
}
```

Response:
```json
{
  "success": true,
  "message": "Analyzed 1 module(s), found 5 invariants",
  "modules": [...],
  "analysis_results": [
    {
      "package_id": "0x2",
      "module_name": "coin",
      "suggested_invariants": [...],
      "analysis_notes": "..."
    }
  ]
}
```

#### GET /api/metadata/:package_id/:module_name
Get module metadata without AI analysis

### Object Monitoring

#### POST /api/monitor
Add object ID to dynamic monitoring
```json
{
  "object_id": "0x..."
}
```

## 🧩 Key Components

### Metadata Fetcher
Fetches Move module metadata from Sui RPC:
- Struct definitions
- Function signatures
- Abilities
- Field types

### LLM Clients
Two implementations:
- `OpenRouterClient`: Cloud AI (Claude, GPT-4, Gemini)
- `OllamaClient`: Local AI (Llama, Mistral, CodeLlama)

Both implement the `LlmClient` trait:
```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn analyze_module(&self, metadata: &ModuleMetadata) -> Result<AnalysisResult>;
}
```

### Invariant Engine
- Starts with 0 invariants (no hard-coded defaults)
- Invariants added via API
- Evaluates all invariants every polling cycle
- Tracks violations and errors

### State Aggregator
Normalizes Sui object data into protocol state:
```rust
pub struct ProtocolState {
    pub total_supply: u64,
    pub total_borrowed: u64,
    pub total_reserves: u64,
    pub collateral_value: u64,
    pub on_chain_balance: u64,
}
```

## 🔧 Development

### Running Tests
```bash
cargo test
cargo test -- --nocapture  # With output
```

### Code Quality
```bash
cargo fmt              # Format code
cargo clippy           # Lint
cargo check            # Quick compile check
```

### Building for Production
```bash
cargo build --release
strip target/release/sui-invariant-monitor  # Reduce binary size
```

## 🐳 Docker

### Build
```bash
docker build -t sui-invariant-monitor .
```

### Run
```bash
docker run -p 8080:8080 \
  -e SUI_RPC_URL=https://fullnode.mainnet.sui.io:443 \
  sui-invariant-monitor
```

## ⚙️ Configuration

### Logging
Set `RUST_LOG` environment variable:
```bash
RUST_LOG=sui_invariant_monitor=debug cargo run
RUST_LOG=info,sui_invariant_monitor=trace cargo run
```

### Polling Interval
Adjust `POLLING_INTERVAL_SECS` in `.env`:
```env
POLLING_INTERVAL_SECS=10  # Check every 10 seconds
```

### Custom RPC Endpoint
```env
# Mainnet
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Testnet
SUI_RPC_URL=https://fullnode.testnet.sui.io:443

# Local
SUI_RPC_URL=http://localhost:9000
```

## 📊 Monitoring & Alerts

### Webhook Alerts
Configure generic webhook:
```env
WEBHOOK_URL=https://your-webhook-endpoint.com/alerts
```

### Discord Alerts
Configure Discord webhook:
```env
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...
```

Alerts are sent when invariants are violated.

## 🔍 Troubleshooting

### "Failed to fetch module metadata"
- Check `SUI_RPC_URL` is correct
- Verify package ID and module name exist on-chain
- Test RPC endpoint: `curl $SUI_RPC_URL`

### "OpenRouter error 404"
- Ensure `HTTP-Referer` and `X-Title` headers are set (already configured)
- Verify API key is valid
- Check model name format: `anthropic/claude-3.5-sonnet`

### "No invariants to display"
- Invariants must be added via `/api/invariants/add` endpoint
- No hard-coded invariants by default
- Use AI analysis to generate suggestions first

## 📚 Dependencies

Key crates:
- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization
- `reqwest` - HTTP client
- `tracing` - Logging
- `chrono` - Date/time
- `anyhow` - Error handling

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- Built for: First Movers Sprint 2026

---

© 2026 Phú Nhuận Builder. Built for First Movers Sprint 2026

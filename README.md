# Sui Invariant Monitor 🛡️

AI-powered smart contract safety monitoring tool for the Sui blockchain. Automatically analyze Move modules and monitor protocol invariants in real-time.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Sui](https://img.shields.io/badge/Sui-Blockchain-4da2ff.svg)

## 🎯 Overview

Sui Invariant Monitor is a comprehensive tool that combines AI analysis with real-time monitoring to ensure smart contract safety on Sui blockchain. It features:

- **AI-Powered Analysis**: Uses LLMs (OpenRouter/Ollama) to analyze Move modules and suggest safety invariants
- **Real-time Monitoring**: Continuously evaluates protocol state against defined invariants
- **Modern UI**: Clean Swiss-style interface with Sui blue branding
- **Network Switching**: Support for both Mainnet and Testnet
- **Flexible Architecture**: Backend in Rust, frontend in React + TypeScript

## ✨ Features

### AI Contract Analysis
- Fetch metadata from any Sui package on-chain
- Analyze Move modules using Claude, GPT-4, Gemini, or local Ollama models
- Generate safety-critical invariants automatically
- Suggest severity levels and formulas for each invariant

### Invariant Monitoring
- Add AI-suggested invariants to active monitoring
- Real-time protocol state evaluation
- Visual status indicators (OK/Violated/Error)
- Remove unwanted invariants with one click

### User Experience
- **Bulk Actions**: Add all suggested invariants at once
- **Manual Control**: Only monitor what you explicitly confirm
- **Network Support**: Switch between Mainnet/Testnet seamlessly
- **Responsive Design**: Clean, minimalist Swiss-style interface

## 🏗️ Architecture

```
sui-invariant-monitor/
├── backend/          # Rust + Axum API server
│   ├── src/
│   │   ├── api/           # REST endpoints
│   │   ├── analysis/      # AI + metadata fetching
│   │   ├── invariants/    # Invariant definitions & engine
│   │   ├── sui_client/    # Sui RPC integration
│   │   └── aggregator/    # State aggregation
│   └── Cargo.toml
│
└── frontend/         # React + TypeScript + Vite
    ├── src/
    │   ├── components/    # UI components
    │   ├── pages/         # Page layouts
    │   ├── api/           # API client
    │   └── context/       # React context (network)
    └── package.json
```

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.82+ (for backend)
- **Node.js** 18+ (for frontend)
- **Ollama** (optional, for local AI models)

### Backend Setup

```bash
cd backend
cp .env.example .env
# Edit .env with your Sui RPC URL
cargo run
```

Backend runs on `http://localhost:8080`

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`

## 📖 Usage Guide

### 1. Analyze a Contract

1. Enter a Sui **Package ID** (e.g., `0x2`)
2. (Optional) Specify a **Module Name**
3. Configure LLM settings:
   - **Ollama**: Local models (llama3.2, codellama, etc.)
   - **OpenRouter**: Cloud models (Claude, GPT-4, Gemini)
4. Click **Analyze Contract**

### 2. Add Invariants to Monitoring

After analysis:
- Click **"+ Add All to Monitoring"** for bulk action
- Or click **"+ Add to Monitoring"** on individual invariants

### 3. Monitor Invariant Status

- View real-time status in the **Invariant Status** grid
- **Green badge**: Invariant is satisfied
- **Red badge**: Invariant violated
- Click **−** button to remove from monitoring

## 🔧 Configuration

### Backend Environment Variables

```env
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
PORT=8080
POLLING_INTERVAL_SECS=10
```

### LLM Providers

**OpenRouter** (Cloud):
- Models: Claude 3.5, GPT-4o, Gemini Pro, Llama 3.1
- Requires API key from [openrouter.ai](https://openrouter.ai/keys)

**Ollama** (Local):
- Auto-detects installed models
- Supports llama3.2, codellama, mistral, qwen2.5-coder
- Run `ollama pull llama3.2` to install

## 🎨 Design Philosophy

**Minimalism & Swiss Style**
- 60% White (primary)
- 30% Sui Blue (#4da2ff)
- 10% Black (tertiary)
- Clean typography, geometric shapes, high contrast

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/invariants` | List all monitored invariants |
| GET | `/api/invariants/:id` | Get specific invariant details |
| GET | `/api/status` | Get monitoring status |
| POST | `/api/analyze` | Analyze package with AI |
| POST | `/api/invariants/add` | Add invariants to monitoring |
| POST | `/api/invariants/remove` | Remove invariant from monitoring |
| POST | `/api/monitor` | Add object ID to monitor |

## 🧪 Development

### Backend Development

```bash
cd backend
cargo watch -x run  # Auto-reload on changes
cargo test          # Run tests
cargo fmt           # Format code
```

### Frontend Development

```bash
cd frontend
npm run dev         # Development server
npm run typecheck   # TypeScript checks
npm run build       # Production build
```

## 📦 Deployment

### Backend (Fly.io)

```bash
cd backend
fly deploy
```

### Frontend (Vercel/Netlify)

```bash
cd frontend
npm run build
# Deploy dist/ folder
```

## 🛠️ Tech Stack

**Backend:**
- Rust
- Axum (Web framework)
- Reqwest (HTTP client)
- Serde (Serialization)
- Tokio (Async runtime)

**Frontend:**
- React 18
- TypeScript
- Vite
- React Query
- React Router

## 🤝 Contributing

This project was built for the **Sui First Movers Sprint 2026**. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- Built for: First Movers Sprint 2026

---

© 2026 Phú Nhuận Builder. Built for First Movers Sprint 2026

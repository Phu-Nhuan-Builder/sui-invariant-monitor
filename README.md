# Sui Invariant Monitor 🛡️

AI-powered smart contract safety monitoring tool for the Sui blockchain. Automatically analyze Move modules and monitor protocol invariants in real-time.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Sui](https://img.shields.io/badge/Sui-Blockchain-4da2ff.svg)
![Status](https://img.shields.io/badge/status-production-success.svg)

## 🌐 Live Demo

- **Frontend**: https://sui-invariant-monitor.vercel.app
- **Backend API**: https://combo-ripe-nat-occur.trycloudflare.com

## 🎯 Overview

Sui Invariant Monitor is a comprehensive tool that combines AI analysis with real-time monitoring to ensure smart contract safety on Sui blockchain. It features:

- **AI-Powered Analysis**: Uses frontier LLMs (GPT-5.2, Claude Opus 4.5, DeepSeek V3.2) to analyze Move modules
- **Real-time Monitoring**: Continuously evaluates protocol state against defined invariants
- **Modern UI**: Clean Swiss-style interface with Sui blue branding
- **Network Switching**: Dynamic support for both Mainnet and Testnet
- **Production Ready**: Deployed with HTTPS, auto-scaling, and 24/7 uptime

## 💡 Why This Project Matters

### 1. Mục đích chính của dự án là gì?

**Sui Invariant Monitor giải quyết vấn đề an toàn smart contract trên Sui blockchain.**

Trong DeFi và các ứng dụng blockchain, smart contracts quản lý hàng tỷ đô la tài sản. Một lỗi nhỏ trong code có thể dẫn đến:
- **Mất tiền**: Exploits, hacks, drains (ví dụ: $600M Poly Network hack)
- **Protocol failure**: Invariants bị vi phạm, hệ thống sụp đổ
- **Mất niềm tin**: Users rời bỏ protocol

**Giải pháp của chúng tôi:**
1. **AI Analysis**: Tự động phân tích Move modules, tìm ra các invariants quan trọng (ví dụ: "Total Supply = Sum of Balances")
2. **Real-time Monitoring**: Liên tục kiểm tra invariants mỗi 10 giây, phát hiện vi phạm ngay lập tức
3. **Early Warning**: Thông báo Discord khi có vấn đề, cho phép team phản ứng nhanh

**Kết quả**: Giảm thiểu rủi ro, tăng độ tin cậy của protocols trên Sui.

### 2. Logic on-chain chính của dự án là gì?

**Dự án này KHÔNG deploy smart contracts lên chain.** Thay vào đó, nó **đọc và phân tích** on-chain data:

**On-chain Interactions:**

1. **Metadata Fetching** (via Sui RPC):
   ```
   GET /sui_getObject -> Lấy object state
   GET /sui_getNormalizedMoveModule -> Lấy module metadata
   ```
   - Đọc struct definitions, function signatures
   - Không modify state, chỉ read-only

2. **State Aggregation**:
   ```
   Monitored Objects -> Fetch current state -> Aggregate values
   Example: 
   - Total Supply = sum(all coin supplies)
   - Total Borrowed = sum(all loan amounts)
   ```

3. **Invariant Evaluation** (Off-chain):
   ```
   Current State -> Check against Invariants -> Alert if violated
   Example:
   - Check: Total Supply >= Total Borrowed
   - If violated -> Send Discord webhook
   ```

**Tại sao không on-chain?**
- **Cost**: On-chain monitoring tốn gas fees liên tục
- **Flexibility**: Off-chain có thể dùng AI, complex logic
- **Speed**: Không bị giới hạn bởi block time

**Trade-off**: Phụ thuộc vào RPC node availability, nhưng đổi lại được flexibility và cost-effectiveness.

### 3. Nếu bỏ blockchain ra khỏi dự án thì sản phẩm còn ý nghĩa không?

**KHÔNG.** Dự án này **hoàn toàn phụ thuộc** vào blockchain. Đây là lý do:

**Blockchain-Specific Features:**

1. **Move Language Analysis**:
   - AI phân tích Move modules (Sui's programming language)
   - Không áp dụng cho traditional databases hay APIs
   - Move có đặc điểm riêng: object-centric, linear types

2. **On-chain State Reading**:
   - Đọc object state từ Sui blockchain
   - Không có blockchain = không có data để monitor
   - Traditional databases không có "invariants" concept như DeFi

3. **DeFi-Specific Invariants**:
   - Total Supply Conservation
   - Collateralization Ratios
   - Liquidity Constraints
   - Các invariants này chỉ tồn tại trong DeFi protocols

4. **Immutable Audit Trail**:
   - Blockchain cung cấp transparent, immutable history
   - Có thể verify violations on-chain
   - Traditional systems có thể bị tamper

**Nếu bỏ blockchain:**
- ❌ Không có Move code để analyze
- ❌ Không có on-chain state để monitor
- ❌ Không có DeFi invariants để check
- ❌ Mất tính transparent và trustless

**Kết luận**: Sui Invariant Monitor là một **blockchain-native tool**, được thiết kế đặc biệt cho Sui ecosystem. Nó không thể tồn tại độc lập khỏi blockchain.

---

## ✨ Features

### AI Contract Analysis
- Fetch metadata from any Sui package on-chain (Mainnet/Testnet)
- Analyze Move modules using latest frontier AI models:
  - **GPT-5.2** (OpenAI)
  - **Claude Opus 4.5** (Anthropic)
  - **Claude Sonnet 4.5** (Anthropic)
  - **DeepSeek V3.2** (Open weights)
  - **MiniMax M2.1** (Chinese frontier)
  - **GLM-4.7** (Zhipu AI)
  - **Mimo V2 Flash** (Free model)
  - **Local Ollama models** (llama3.2, codellama, etc.)
- Generate safety-critical invariants automatically
- Suggest severity levels and formulas for each invariant

### Invariant Monitoring
- Add AI-suggested invariants to active monitoring
- Real-time protocol state evaluation (10-second polling)
- Visual status indicators (OK/Violated/Error)
- Remove unwanted invariants with one click
- Discord webhook notifications for violations

### User Experience
- **Bulk Actions**: Add all suggested invariants at once
- **Manual Control**: Only monitor what you explicitly confirm
- **Network Support**: Switch between Mainnet/Testnet seamlessly
- **Responsive Design**: Clean, minimalist Swiss-style interface
- **No Registration**: Start analyzing immediately

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│  Frontend (Vercel - HTTPS)              │
│  https://sui-invariant-monitor.vercel.app│
└──────────────┬──────────────────────────┘
               │ HTTPS
               ▼
┌─────────────────────────────────────────┐
│  Cloudflare Tunnel (Free HTTPS)         │
│  https://combo-ripe-nat-occur.trycloudflare.com│
└──────────────┬──────────────────────────┘
               │ HTTP
               ▼
┌─────────────────────────────────────────┐
│  Backend (VPS - ckey.vn)                │
│  Rust + Axum + Supervisor               │
│  Port: 7681                             │
└──────────────┬──────────────────────────┘
               │
               ▼
         Sui RPC (Mainnet/Testnet)
```

### Tech Stack

**Backend:**
- **Language**: Rust 1.83
- **Framework**: Axum (async web framework)
- **HTTP Client**: Reqwest (with HTTP/1.1 for OpenRouter)
- **Serialization**: Serde
- **Runtime**: Tokio
- **Process Manager**: Supervisor
- **Hosting**: VPS (ckey.vn)
- **HTTPS**: Cloudflare Tunnel (free)

**Frontend:**
- **Framework**: React 18 + TypeScript
- **Build Tool**: Vite
- **State Management**: React Query (TanStack Query)
- **Routing**: React Router v6
- **Styling**: Vanilla CSS (Swiss design)
- **Hosting**: Vercel (auto-deploy on push)

## 🚀 Quick Start

### Live Demo (No Setup Required)

Visit https://sui-invariant-monitor.phunhuanbuilder.com or https://invariant-monitor-on-sui.vercel.app and start analyzing contracts immediately!

### Local Development

#### Prerequisites
- **Rust** 1.83+ (for backend)
- **Node.js** 18+ (for frontend)
- **Ollama** (optional, for local AI models)

#### Backend Setup

```bash
cd backend
cp .env.example .env
# Edit .env with your configuration
cargo run
```

Backend runs on `http://localhost:8080`

#### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`

## 📖 Usage Guide

### 1. Analyze a Contract

1. Visit https://sui-invariant-monitor.vercel.app
2. Select **Network** (Mainnet or Testnet)
3. Enter a Sui **Package ID** (e.g., `0x2` for Sui Framework)
4. (Optional) Specify a **Module Name** to analyze specific module
5. Configure LLM settings:
   - **Ollama**: Local models (free, requires Ollama running)
   - **OpenRouter**: Cloud models (requires API key)
6. Click **Analyze Contract**

### 2. Add Invariants to Monitoring

After analysis:
- Click **"+ Add All to Monitoring"** for bulk action
- Or click **"+ Add to Monitoring"** on individual invariants
- Invariants will be evaluated every 10 seconds

### 3. Monitor Invariant Status

- View real-time status in the **Invariant Status** grid
- **Green badge**: Invariant is satisfied ✅
- **Red badge**: Invariant violated ⚠️
- **Gray badge**: Evaluation error
- Click **−** button to remove from monitoring

## 🔧 Configuration

### Backend Environment Variables

```env
# Sui RPC URL (fallback, dynamic switching via API)
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Server port
PORT=8080

# Logging level
RUST_LOG=info

# Polling interval (seconds)
POLLING_INTERVAL_SECS=10

# Discord webhook for violation alerts
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/YOUR_WEBHOOK_URL
```

### LLM Providers

**OpenRouter** (Cloud - Recommended for production):
- Latest frontier models (GPT-5.2, Claude Opus 4.5, etc.)
- Requires API key from [openrouter.ai](https://openrouter.ai/keys)
- Pay-per-use pricing

**Ollama** (Local - Free):
- Auto-detects installed models
- Supports llama3.2, codellama, mistral, qwen2.5-coder
- Run `ollama pull llama3.2` to install
- Requires Ollama running on `http://localhost:11434`

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/status` | Get monitoring status |
| GET | `/api/invariants` | List all monitored invariants |
| GET | `/api/invariants/:id` | Get specific invariant details |
| POST | `/api/analyze` | Analyze package with AI |
| POST | `/api/invariants/add` | Add invariants to monitoring |
| POST | `/api/invariants/remove` | Remove invariant from monitoring |
| POST | `/api/monitor` | Add object ID to monitor |
| GET | `/api/metadata/:package/:module` | Get module metadata |

## 📦 Deployment

### Production Deployment

**Backend** (VPS + Cloudflare Tunnel):
- See [`DEPLOY_STEPS.md`](DEPLOY_STEPS.md) for VPS deployment
- See [`CLOUDFLARE_TUNNEL.md`](CLOUDFLARE_TUNNEL.md) for HTTPS setup

**Frontend** (Vercel):
```bash
cd frontend
vercel --prod
```

Auto-deploys on every push to `main` branch.

### Alternative Deployment Options

- **Render.com**: See [`RENDER_DEPLOYMENT.md`](RENDER_DEPLOYMENT.md)
- **VPS**: See [`VPS_DEPLOYMENT.md`](VPS_DEPLOYMENT.md)

## 🎨 Design Philosophy

**Minimalism & Swiss Style**
- 60% White (primary background)
- 30% Sui Blue (#4da2ff) (accents, CTAs)
- 10% Black (tertiary text)
- Clean typography, geometric shapes, high contrast
- No unnecessary decorations or gradients

## 🧪 Development

### Backend Development

```bash
cd backend
cargo watch -x run  # Auto-reload on changes
cargo test          # Run tests
cargo fmt           # Format code
cargo clippy        # Linting
```

### Frontend Development

```bash
cd frontend
npm run dev         # Development server
npm run typecheck   # TypeScript checks
npm run build       # Production build
npm run preview     # Preview production build
```

## 🔒 Security

- **HTTPS**: All production traffic encrypted via Cloudflare Tunnel
- **CORS**: Properly configured for cross-origin requests
- **No Secrets in Code**: Environment variables for sensitive data
- **Input Validation**: Package IDs and module names validated
- **Rate Limiting**: Recommended for production (not implemented)

## 📊 Monitoring & Logs

### Backend Logs (VPS)
```bash
ssh -p 1443 root@n2.ckey.vn
supervisorctl tail -f sui-monitor
```

### Frontend Logs
- Vercel Dashboard: https://vercel.com/dashboard
- Real-time logs and analytics

## 🤝 Contributing

This project was built for the **Sui First Movers Sprint 2026**. Contributions are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit with descriptive messages
5. Push to your fork
6. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- GitHub: [@phunhuanbuilder](https://github.com/phunhuanbuilder)
- Built for: **First Movers Sprint 2026**

## 🙏 Acknowledgments

- **Sui Foundation** for the First Movers Sprint
- **Cloudflare** for free HTTPS tunneling
- **Vercel** for frontend hosting
- **OpenRouter** for AI model access
- **Anthropic, OpenAI, DeepSeek** for frontier AI models

---

© 2026 Phú Nhuận Builder. Built for First Movers Sprint 2026

**Live Demo**: https://sui-invariant-monitor.vercel.app

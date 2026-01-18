# 🚀 Quick Setup Guide for Demo on Fresh macOS

This guide helps you set up the Sui Invariant Monitor for demo on a fresh macOS installation.

## ⚡ Quick Start (5 minutes)

### Option 1: Use Production (Fastest - No Setup)

Just visit the live demo:
- **Frontend**: https://sui-invariant-monitor.phunhuanbuilder.com
- **Backend**: Already running on VPS

✅ **No installation needed!**

---

### Option 2: Run Locally (Full Demo)

## 📋 Prerequisites

### 1. Install Homebrew

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 2. Install Required Tools

```bash
# Install Node.js (for frontend)
brew install node

# Install Rust (for backend)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installations
node --version  # Should be 18+
cargo --version # Should be 1.83+
```

### 3. Install Git (if not installed)

```bash
brew install git
```

---

## 🎯 Setup Steps

### Step 1: Clone Repository

```bash
cd ~/Downloads
git clone https://github.com/Phu-Nhuan-Builder/sui-invariant-monitor.git
cd sui-invariant-monitor
```

### Step 2: Setup Backend

```bash
cd backend

# Create .env file
cat > .env << 'EOF'
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
PORT=8080
RUST_LOG=info
POLLING_INTERVAL_SECS=10
EOF

# Build and run (takes ~5 minutes first time)
cargo run
```

**Wait for**: "Starting API server on 0.0.0.0:8080"

### Step 3: Setup Frontend (New Terminal)

Open a new terminal:

```bash
cd ~/Downloads/sui-invariant-monitor/frontend

# Install dependencies
npm install

# Run dev server
npm run dev
```

**Wait for**: "Local: http://localhost:5173"

---

## 🎬 Demo the Application

### 1. Open Browser

Visit: http://localhost:5173

### 2. Demo Flow

#### A. Analyze a Contract

1. **Select Network**: Choose "Mainnet"
2. **Enter Package ID**: `0x2` (Sui Framework)
3. **Select LLM Provider**: 
   - **OpenRouter** (requires API key from https://openrouter.ai/keys)
   - **Ollama** (requires local Ollama - see below)
4. **Choose Model**: 
   - OpenRouter: `anthropic/claude-sonnet-4.5`
   - Ollama: `llama3.2` (if installed)
5. Click **"Analyze Contract"**

#### B. Add Invariants to Monitoring

After analysis completes:
1. Click **"+ Add All to Monitoring"** button
2. View invariants in the status grid
3. Watch real-time status updates (every 10 seconds)

#### C. Network Switching

1. Toggle between **Mainnet** and **Testnet**
2. Analyze different packages on each network
3. Show dynamic RPC URL switching

---

## 🎨 Demo Tips

### Prepare Before Demo

1. **Get OpenRouter API Key** (if using cloud AI):
   - Visit https://openrouter.ai/keys
   - Sign up and get free credits
   - Copy API key

2. **Test Package IDs**:
   - Mainnet: `0x2` (Sui Framework)
   - Testnet: `0x2` (Sui Framework)

3. **Pre-run Build**:
   ```bash
   # Backend
   cd backend && cargo build --release
   
   # Frontend
   cd frontend && npm run build
   ```

### During Demo

1. **Show AI Analysis**:
   - Explain how AI reads Move code
   - Show suggested invariants
   - Highlight severity levels

2. **Show Real-time Monitoring**:
   - Add invariants to monitoring
   - Show status badges (OK/Violated)
   - Explain 10-second polling

3. **Show Network Switching**:
   - Toggle mainnet/testnet
   - Show same package on different networks
   - Explain dynamic RPC URLs

4. **Show Architecture**:
   - Open `README.md` → Architecture diagram
   - Explain off-chain monitoring
   - Show why blockchain is essential

---

## 🔧 Optional: Install Ollama (Local AI)

For offline demo without API keys:

```bash
# Install Ollama
brew install ollama

# Start Ollama service
ollama serve &

# Pull a model
ollama pull llama3.2

# Verify
curl http://localhost:11434/api/tags
```

Now you can use Ollama in the app without API keys!

---

## 🐛 Troubleshooting

### Backend won't start

```bash
# Check if port 8080 is in use
lsof -i :8080

# Kill process if needed
kill -9 <PID>

# Try different port
PORT=8081 cargo run
```

### Frontend won't start

```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Ollama not working

```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# Restart Ollama
pkill ollama
ollama serve &
```

---

## 📊 Demo Checklist

Before presenting:
- [ ] Backend running on http://localhost:8080
- [ ] Frontend running on http://localhost:5173
- [ ] OpenRouter API key ready (or Ollama installed)
- [ ] Test package IDs noted
- [ ] Browser open to http://localhost:5173
- [ ] README.md open for architecture diagram
- [ ] Terminal windows visible for logs

During demo:
- [ ] Show AI analysis with real package
- [ ] Add invariants to monitoring
- [ ] Show real-time status updates
- [ ] Switch between mainnet/testnet
- [ ] Explain blockchain necessity
- [ ] Show code quality (optional)

---

## 🎯 Quick Demo Script (5 minutes)

1. **Introduction** (30s)
   - "Sui Invariant Monitor - AI-powered smart contract safety"
   - Show live demo URL

2. **AI Analysis** (2 min)
   - Enter package `0x2`
   - Select Claude Sonnet 4.5
   - Click Analyze
   - Show suggested invariants

3. **Real-time Monitoring** (1.5 min)
   - Add all invariants
   - Show status grid
   - Explain 10-second polling
   - Show Discord alerts (optional)

4. **Network Switching** (30s)
   - Toggle mainnet/testnet
   - Show dynamic RPC

5. **Q&A** (30s)
   - Why blockchain is essential
   - Architecture overview

---

## 💡 Alternative: Use Production Demo

If local setup fails, use production:

1. Visit: https://sui-invariant-monitor.phunhuanbuilder.com
2. Same demo flow as local
3. Backend already running on VPS
4. No setup needed!

---

## 📝 Summary

**Fastest**: Use production demo (0 setup)
**Best**: Run locally (5 min setup)
**Offline**: Install Ollama (10 min setup)

Choose based on your demo environment and internet availability!

---

© 2026 Phú Nhuận Builder

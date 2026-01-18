# 🚀 Deploy Backend lên VPS - Hướng dẫn Step-by-Step

## 📋 Thông tin VPS của bạn
- **SSH**: `ssh -p 1443 root@n2.ckey.vn`
- **Port mapping**: 1443 → 22, 1444 → 7681
- **OS**: Ubuntu (Jammy)
- **Expires**: 20-01-2026

## ✅ Bước 1: SSH vào VPS

Mở terminal trên Mac:

```bash
ssh -p 1443 root@n2.ckey.vn
```

Nhập password khi được hỏi (từ ckey.vn panel).

## ✅ Bước 2: Update System

```bash
apt update && apt upgrade -y
```

## ✅ Bước 3: Install Rust

```bash
# Install dependencies
apt install -y build-essential pkg-config libssl-dev git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Chọn option 1 (default installation)

# Load Rust environment
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

## ✅ Bước 4: Clone Repository

```bash
cd ~
git clone https://github.com/phunhuanbuilder/sui-invariant-monitor.git
cd sui-invariant-monitor/backend
```

## ✅ Bước 5: Create .env File

```bash
nano .env
```

**Paste nội dung này:**
```env
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
PORT=8080
RUST_LOG=info
POLLING_INTERVAL_SECS=10
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR
```

**Save**: Ctrl+O → Enter → Ctrl+X

## ✅ Bước 6: Build Project

```bash
cargo build --release
```

⏳ Đợi 5-10 phút. Nếu thành công sẽ thấy: `Finished release profile`

## ✅ Bước 7: Test Run

```bash
./target/release/sui-invariant-monitor
```

Nếu thấy logs:
- "Starting Sui Invariant Monitor"
- "Starting API server on 0.0.0.0:8080"

→ **Thành công!** Press Ctrl+C để stop.

## ✅ Bước 8: Setup Systemd Service

```bash
nano /etc/systemd/system/sui-monitor.service
```

**Paste:**
```ini
[Unit]
Description=Sui Invariant Monitor
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/sui-invariant-monitor/backend
Environment="PATH=/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
EnvironmentFile=/root/sui-invariant-monitor/backend/.env
ExecStart=/root/sui-invariant-monitor/backend/target/release/sui-invariant-monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Save**: Ctrl+O → Enter → Ctrl+X

```bash
# Enable và start service
systemctl daemon-reload
systemctl enable sui-monitor
systemctl start sui-monitor

# Check status
systemctl status sui-monitor
```

Phải thấy: **"active (running)"** màu xanh

## ✅ Bước 9: Install Nginx

```bash
apt install -y nginx

nano /etc/nginx/sites-available/sui-monitor
```

**Paste:**
```nginx
server {
    listen 80;
    server_name n2.ckey.vn;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

**Save**: Ctrl+O → Enter → Ctrl+X

```bash
# Enable site
ln -s /etc/nginx/sites-available/sui-monitor /etc/nginx/sites-enabled/
nginx -t
systemctl restart nginx

# Allow firewall
ufw allow 80
ufw allow 1443
ufw enable
```

## ✅ Bước 10: Test API

Từ VPS:
```bash
curl http://localhost:8080/health
```

Từ máy local (Mac):
```bash
curl http://n2.ckey.vn/health
```

Response:
```json
{"status":"ok","uptime_secs":123}
```

## 🎯 Backend URL

**Public URL**: `http://n2.ckey.vn`

## 📊 Useful Commands

```bash
# Xem logs
journalctl -u sui-monitor -f

# Restart
systemctl restart sui-monitor

# Status
systemctl status sui-monitor

# Update code
cd ~/sui-invariant-monitor
git pull
cd backend
cargo build --release
systemctl restart sui-monitor
```

## 🔄 Update Frontend

```typescript
// frontend/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'http://n2.ckey.vn';
```

```bash
cd frontend
vercel --prod
```

---

✅ Done! Backend running 24/7 tại: **http://n2.ckey.vn**

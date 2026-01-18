# 🚀 Deploy Backend lên VPS - Hướng dẫn Step-by-Step (Supervisor)

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
# RPC URL for monitoring service (background evaluation)
# Note: AI analysis will use network selected from frontend (mainnet/testnet)
# This is only fallback for monitoring service
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

PORT=8080
RUST_LOG=info
POLLING_INTERVAL_SECS=10
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR
```

> **📝 Lưu ý về Network Switching:**
> - **AI Analysis**: Frontend tự động pass network (mainnet/testnet) → backend sẽ dùng đúng RPC URL
> - **Monitoring Service**: Dùng `SUI_RPC_URL` từ .env (nếu bạn add invariants để monitor)
> - Không cần config 2 RPC URLs, network switching hoạt động tự động!

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

## ✅ Bước 8: Install Supervisor

```bash
apt install -y supervisor
```

Verify supervisor installed:
```bash
supervisorctl version
```

## ✅ Bước 9: Create Supervisor Config

```bash
nano /etc/supervisor/conf.d/sui-monitor.conf
```

**Paste:**
```ini
[program:sui-monitor]
command=/root/sui-invariant-monitor/backend/target/release/sui-invariant-monitor
directory=/root/sui-invariant-monitor/backend
user=root
autostart=true
autorestart=true
redirect_stderr=true
stdout_logfile=/var/log/sui-monitor.log
stdout_logfile_maxbytes=10MB
stdout_logfile_backups=3
# Note: SUI_RPC_URL is fallback for monitoring service
# AI analysis uses network from frontend (mainnet/testnet dynamic switching)
environment=PATH="/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin",RUST_LOG="info",SUI_RPC_URL="https://fullnode.mainnet.sui.io:443",PORT="8080",POLLING_INTERVAL_SECS="10",DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR"
```

**Save**: Ctrl+O → Enter → Ctrl+X

## ✅ Bước 10: Start Service với Supervisor

```bash
# Reload supervisor config
supervisorctl reread
supervisorctl update

# Start service
supervisorctl start sui-monitor

# Check status (phải thấy RUNNING)
supervisorctl status sui-monitor
```

Expected output:
```
sui-monitor                      RUNNING   pid 12345, uptime 0:00:05
```

## ✅ Bước 11: View Logs

```bash
# Xem logs real-time
tail -f /var/log/sui-monitor.log

# Hoặc dùng supervisor
supervisorctl tail -f sui-monitor
```

## ✅ Bước 12: Install Nginx (Optional - for reverse proxy)

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
systemctl restart nginx || service nginx restart

# Allow firewall
ufw allow 80
ufw allow 1443
ufw enable
```

## ✅ Bước 13: Test API

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

## 📊 Supervisor Commands

```bash
# Xem status
supervisorctl status

# Start service
supervisorctl start sui-monitor

# Stop service
supervisorctl stop sui-monitor

# Restart service
supervisorctl restart sui-monitor

# Xem logs
supervisorctl tail sui-monitor
supervisorctl tail -f sui-monitor  # Follow logs

# Reload config sau khi sửa
supervisorctl reread
supervisorctl update
```

## 🔄 Update Code

Khi có code mới:

```bash
cd ~/sui-invariant-monitor
git pull
cd backend
cargo build --release
supervisorctl restart sui-monitor

# Check logs
supervisorctl tail -f sui-monitor
```

## 🔧 Troubleshooting

### Service không start:
```bash
# Check supervisor status
supervisorctl status sui-monitor

# Check logs
tail -50 /var/log/sui-monitor.log

# Manual test
cd ~/sui-invariant-monitor/backend
./target/release/sui-invariant-monitor
```

### Port 8080 đã được dùng:
```bash
# Find process
lsof -i :8080

# Stop via supervisor
supervisorctl stop sui-monitor

# Or kill process
kill -9 PID
```

### Supervisor không nhận config:
```bash
# Reload config
supervisorctl reread
supervisorctl update

# Restart supervisor service
service supervisor restart
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

✅ Done! Backend running 24/7 với Supervisor tại: **http://n2.ckey.vn**

## 💡 Ưu điểm Supervisor

- ✅ **Auto-restart**: Tự động restart khi app crash
- ✅ **Log management**: Tự động rotate logs
- ✅ **Process monitoring**: Theo dõi process status
- ✅ **Easy management**: Commands đơn giản và rõ ràng
- ✅ **Resource control**: Giới hạn resources nếu cần

# VPS Deployment Guide - Ckey.vn

## 🏷️ VPS Specs Recommended

**Package đề xuất**: 200-300k VNĐ/tháng
- **RAM**: 1GB minimum (2GB better)
- **CPU**: 1 core
- **Storage**: 20GB SSD
- **OS**: Ubuntu 22.04 LTS
- **Location**: Vietnam datacenter

## 🚀 Step-by-Step Setup

### 1️⃣ Khởi tạo VPS và SSH

```bash
# SSH vào server (IP và password từ Ckey.vn panel)
ssh root@YOUR_VPS_IP

# Update system
apt update && apt upgrade -y

# Create new user for security
adduser monitor
usermod -aG sudo monitor

# Switch to monitor user
su - monitor
```

### 2️⃣ Install Dependencies

```bash
# Install build tools
sudo apt install -y build-essential pkg-config libssl-dev git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 3️⃣ Clone & Build Project

```bash
# Clone repository
cd ~
git clone https://github.com/phunhuanbuilder/sui-invariant-monitor.git
cd sui-invariant-monitor/backend

# Create .env file
nano .env
```

**Add to `.env`:**
```env
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
PORT=8080
RUST_LOG=info
POLLING_INTERVAL_SECS=10
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR
```

**Save**: Ctrl+O, Enter, Ctrl+X

```bash
# Build release version
cargo build --release

# Test run (Ctrl+C to stop)
./target/release/sui-invariant-monitor
```

### 4️⃣ Setup Systemd Service (Auto-start)

```bash
# Create service file
sudo nano /etc/systemd/system/sui-monitor.service
```

**Add this content:**
```ini
[Unit]
Description=Sui Invariant Monitor
After=network.target

[Service]
Type=simple
User=monitor
WorkingDirectory=/home/monitor/sui-invariant-monitor/backend
Environment="PATH=/home/monitor/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
EnvironmentFile=/home/monitor/sui-invariant-monitor/backend/.env
ExecStart=/home/monitor/sui-invariant-monitor/backend/target/release/sui-invariant-monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Save**: Ctrl+O, Enter, Ctrl+X

```bash
# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable sui-monitor
sudo systemctl start sui-monitor

# Check status
sudo systemctl status sui-monitor

# View logs
sudo journalctl -u sui-monitor -f
```

### 5️⃣ Setup Nginx Reverse Proxy

```bash
# Install nginx
sudo apt install -y nginx

# Create nginx config
sudo nano /etc/nginx/sites-available/sui-monitor
```

**Add this content:**
```nginx
server {
    listen 80;
    server_name YOUR_VPS_IP;  # Or your domain

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Save**: Ctrl+O, Enter, Ctrl+X

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/sui-monitor /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx

# Allow firewall
sudo ufw allow 80
sudo ufw allow 443
sudo ufw allow 22
sudo ufw enable
```

### 6️⃣ Setup SSL Certificate (Optional but Recommended)

```bash
# Install Certbot
sudo apt install -y certbot python3-certbot-nginx

# Get SSL certificate (nếu có domain)
sudo certbot --nginx -d your-domain.com

# Certbot sẽ tự động renew
sudo certbot renew --dry-run
```

**Nếu không có domain**, dùng HTTP qua IP: `http://YOUR_VPS_IP`

## 🔧 Common Commands

```bash
# Start service
sudo systemctl start sui-monitor

# Stop service
sudo systemctl stop sui-monitor

# Restart service
sudo systemctl restart sui-monitor

# View logs
sudo journalctl -u sui-monitor -f

# Check status
sudo systemctl status sui-monitor

# Update code
cd ~/sui-invariant-monitor
git pull
cd backend
cargo build --release
sudo systemctl restart sui-monitor
```

## 📊 Monitoring

### Check if service is running:
```bash
# Check process
ps aux | grep sui-invariant-monitor

# Check port
sudo netstat -tulpn | grep 8080

# Test API
curl http://localhost:8080/health
curl http://YOUR_VPS_IP/health
```

### View resource usage:
```bash
# CPU and Memory
htop

# Or
top
```

## 🔒 Security Best Practices

```bash
# 1. Change SSH port (optional)
sudo nano /etc/ssh/sshd_config
# Change Port 22 to Port 2222
sudo systemctl restart sshd

# 2. Disable root login
sudo nano /etc/ssh/sshd_config
# Set PermitRootLogin no
sudo systemctl restart sshd

# 3. Setup fail2ban
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

## 🌐 Update Frontend URL

Sau khi backend chạy thành công:

```typescript
// frontend/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'http://YOUR_VPS_IP';
```

```env
# frontend/.env.production
VITE_API_URL=http://YOUR_VPS_IP
```

Redeploy frontend:
```bash
cd frontend
vercel --prod
```

## 💰 Cost Estimate (Ckey.vn)

**VPS 1GB RAM**:
- ~200-300k VNĐ/tháng
- ~7-10k VNĐ/ngày
- ~300-400 VNĐ/giờ

**Bandwidth**: Usually unlimited trong gói

**Total**: ~250k VNĐ/tháng cho backend running 24/7

## 🆘 Troubleshooting

### Service không start:
```bash
# Check logs
sudo journalctl -u sui-monitor -n 50

# Check binary exists
ls -la ~/sui-invariant-monitor/backend/target/release/

# Test run manually
cd ~/sui-invariant-monitor/backend
./target/release/sui-invariant-monitor
```

### Port 8080 đã được dùng:
```bash
# Find process using port
sudo lsof -i :8080

# Kill process
sudo kill -9 PID
```

### Out of memory:
```bash
# Add swap
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

## 📝 Checklist

- [ ] VPS đã mua và có IP
- [ ] SSH vào được VPS
- [ ] Installed Rust và dependencies
- [ ] Cloned repository
- [ ] Created `.env` file
- [ ] Built release binary
- [ ] Tested manual run
- [ ] Created systemd service
- [ ] Service running và auto-start
- [ ] Nginx installed và configured
- [ ] Firewall configured
- [ ] API accessible từ public
- [ ] Updated frontend URL
- [ ] (Optional) SSL certificate setup

## 🎯 Backend sẽ available tại:

- **Local**: http://localhost:8080
- **Public**: http://YOUR_VPS_IP
- **With Domain**: https://your-domain.com

---

**Author**: Phú Nhuận Builder
**Email**: phunhuanbuilder@gmail.com

**Support**: Nếu gặp vấn đề, check logs:
```bash
sudo journalctl -u sui-monitor -f
```

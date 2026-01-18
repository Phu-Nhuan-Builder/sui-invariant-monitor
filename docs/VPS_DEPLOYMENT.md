# VPS Deployment Guide - ThueGPU.vn

## 🏷️ VPS Specs Recommended

**Package đề xuất**: ThueGPU.vn - VPS CPU
- **RAM**: 1GB minimum (2GB better)
- **CPU**: 1 core
- **Storage**: 20GB SSD
- **OS**: Ubuntu 22.04 LTS
- **Location**: Vietnam datacenter
- **Price**: ~200-300k VNĐ/tháng

## 🚀 Step-by-Step Setup

### 1️⃣ Đăng ký và Khởi tạo VPS trên ThueGPU.vn

1. Truy cập https://thuegpu.vn
2. Đăng ký tài khoản
3. Chọn gói **VPS CPU** (không cần GPU cho backend này)
4. Chọn hệ điều hành: **Ubuntu 22.04 LTS**
5. Chọn datacenter: **Vietnam**
6. Thanh toán và đợi VPS được khởi tạo
7. Lấy thông tin SSH từ panel: IP, username, password

### 2️⃣ SSH vào VPS và Setup User

```bash
# SSH vào server (thông tin từ ThueGPU.vn panel)
ssh root@YOUR_VPS_IP
# Nhập password khi được yêu cầu

# Update system
apt update && apt upgrade -y

# Create new user for security
adduser monitor
usermod -aG sudo monitor

# Switch to monitor user
su - monitor
```

### 3️⃣ Install Dependencies

```bash
# Install build tools
sudo apt install -y build-essential pkg-config libssl-dev git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Chọn option 1 (default)
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 4️⃣ Clone & Build Project

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
# Build release version (mất 5-10 phút)
cargo build --release

# Test run (Ctrl+C to stop)
./target/release/sui-invariant-monitor
```

Nếu thấy log "Starting Sui Invariant Monitor" và "Starting API server" → thành công!

### 5️⃣ Setup Systemd Service (Auto-start khi reboot)

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

# Check status (phải thấy "active (running)")
sudo systemctl status sui-monitor

# View logs real-time
sudo journalctl -u sui-monitor -f
```

### 6️⃣ Setup Nginx Reverse Proxy

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
    server_name YOUR_VPS_IP;  # Hoặc domain của bạn

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

# Setup firewall
sudo ufw allow 80
sudo ufw allow 443
sudo ufw allow 22
sudo ufw enable
```

### 7️⃣ Test API từ Public

```bash
# Test từ VPS
curl http://localhost:8080/health

# Test từ máy local của bạn
curl http://YOUR_VPS_IP/health
```

Response thành công:
```json
{"status":"ok","uptime_secs":123}
```

### 8️⃣ (Optional) Setup SSL Certificate với Domain

**Nếu có domain** (ví dụ: `api.yoursite.com`):

```bash
# Point domain A record to VPS IP trước

# Install Certbot
sudo apt install -y certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d api.yoursite.com

# Test auto-renewal
sudo certbot renew --dry-run
```

**Nếu KHÔNG có domain**, dùng HTTP qua IP: `http://YOUR_VPS_IP`

## 🔧 Common Commands

```bash
# Xem logs
sudo journalctl -u sui-monitor -f

# Restart service
sudo systemctl restart sui-monitor

# Stop service
sudo systemctl stop sui-monitor

# Start service
sudo systemctl start sui-monitor

# Check status
sudo systemctl status sui-monitor

# Update code từ GitHub
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

# Test health endpoint
curl http://localhost:8080/health
curl http://localhost:8080/api/status
```

### View resource usage:
```bash
# Install htop nếu chưa có
sudo apt install htop

# Monitor resources
htop

# Check memory
free -h

# Check disk
df -h
```

## 🔒 Security Best Practices

```bash
# 1. Change SSH port (tùy chọn)
sudo nano /etc/ssh/sshd_config
# Đổi Port 22 thành Port 2222
sudo systemctl restart sshd

# 2. Disable root login
sudo nano /etc/ssh/sshd_config
# Set PermitRootLogin no
sudo systemctl restart sshd

# 3. Setup fail2ban (chống brute force)
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

# 4. Auto security updates
sudo apt install unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades
```

## 🌐 Update Frontend URL

Sau khi backend chạy thành công trên VPS:

**Update hardcoded fallback:**
```typescript
// frontend/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'http://YOUR_VPS_IP';
```

**Update production env:**
```env
# frontend/.env.production
VITE_API_URL=http://YOUR_VPS_IP
```

**Redeploy frontend:**
```bash
cd frontend
vercel --prod
```

## 💰 Cost Estimate (ThueGPU.vn)

**VPS CPU 1GB RAM**:
- ~200-300k VNĐ/tháng
- Thanh toán theo tháng
- Bandwidth không giới hạn (fair use)

**Ưu điểm**:
- ✅ Datacenter tại VN → latency thấp
- ✅ Support tiếng Việt
- ✅ Thanh toán VNĐ
- ✅ GPU options nếu cần sau này

## 🆘 Troubleshooting

### Service không start:
```bash
# Check detailed logs
sudo journalctl -u sui-monitor -n 100 --no-pager

# Check if binary exists
ls -la ~/sui-invariant-monitor/backend/target/release/sui-invariant-monitor

# Test run manually
cd ~/sui-invariant-monitor/backend
./target/release/sui-invariant-monitor
```

### Port 8080 already in use:
```bash
# Find process using port
sudo lsof -i :8080

# Kill process
sudo kill -9 PID
```

### Out of memory (RAM < 1GB):
```bash
# Add swap space
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Verify
free -h
```

### Build fails (cargo build error):
```bash
# Check Rust version
rustc --version  # Should be 1.83+

# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Cannot access from public:
```bash
# Check if nginx running
sudo systemctl status nginx

# Check if service running
sudo systemctl status sui-monitor

# Check firewall
sudo ufw status

# Check if port open
curl http://localhost:8080/health
```

## 📝 Deployment Checklist

- [ ] Đăng ký ThueGPU.vn và mua VPS
- [ ] SSH vào được VPS
- [ ] Created user `monitor`
- [ ] Installed Rust và dependencies
- [ ] Cloned repository từ GitHub
- [ ] Created `.env` file với đầy đủ config
- [ ] Built release binary successfully
- [ ] Tested manual run (./target/release/sui-invariant-monitor)
- [ ] Created systemd service
- [ ] Service đang running (`systemctl status sui-monitor`)
- [ ] Nginx installed và configured
- [ ] Firewall configured (port 80, 443, 22)
- [ ] API accessible từ public (`curl http://VPS_IP/health`)
- [ ] Updated frontend với VPS IP
- [ ] Frontend redeployed
- [ ] (Optional) Domain configured
- [ ] (Optional) SSL certificate installed

## 🎯 Backend URLs

Sau khi setup thành công:

- **Internal**: http://localhost:8080
- **Public (HTTP)**: http://YOUR_VPS_IP
- **Public (HTTPS với domain)**: https://api.yoursite.com

### Test Endpoints:

```bash
# Health check
curl http://YOUR_VPS_IP/health

# Status
curl http://YOUR_VPS_IP/api/status

# List invariants
curl http://YOUR_VPS_IP/api/invariants
```

## 🔄 Auto-deploy từ GitHub (Optional)

Setup webhook để auto-deploy khi push code:

```bash
# Create deploy script
nano ~/deploy.sh
```

```bash
#!/bin/bash
cd ~/sui-invariant-monitor
git pull
cd backend
cargo build --release
sudo systemctl restart sui-monitor
```

```bash
# Make executable
chmod +x ~/deploy.sh

# Setup GitHub webhook hoặc cron job
crontab -e
# Add: */5 * * * * ~/deploy.sh >> ~/deploy.log 2>&1
```

## 📞 Support

**ThueGPU.vn Support**:
- Website: https://thuegpu.vn
- Email: support@thuegpu.vn
- Chat: Có trên website

**Project Issues**:
- GitHub: https://github.com/phunhuanbuilder/sui-invariant-monitor
- Email: phunhuanbuilder@gmail.com

---

**Author**: Phú Nhuận Builder  
**Email**: phunhuanbuilder@gmail.com  
**Built for**: First Movers Sprint 2026

**Pro tip**: Dùng `screen` hoặc `tmux` để keep terminal sessions khi disconnect SSH:
```bash
# Install screen
sudo apt install screen

# Start screen session
screen -S monitor

# Run app
./target/release/sui-invariant-monitor

# Detach: Ctrl+A then D
# Reattach: screen -r monitor
```

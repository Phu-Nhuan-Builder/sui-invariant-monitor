# 🚀 Hướng Dẫn Cấu Hình Lại Từ Đầu

## 📋 Tổng Quan

Sau khi đã push code lên repo mới, đây là các bước để cấu hình lại toàn bộ hệ thống.

**Repository mới**: https://github.com/Phu-Nhuan-Builder/sui-invariant-monitor

---

## 1️⃣ Backend - VPS Deployment

### Bước 1: SSH vào VPS

```bash
ssh -p 1443 root@n2.ckey.vn
```

### Bước 2: Xóa code cũ (nếu có)

```bash
cd ~
rm -rf sui-invariant-monitor
```

### Bước 3: Clone repo mới

```bash
git clone https://github.com/Phu-Nhuan-Builder/sui-invariant-monitor.git
cd sui-invariant-monitor/backend
```

### Bước 4: Tạo file .env

```bash
nano .env
```

**Paste:**
```env
# RPC URL for monitoring service (fallback)
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

PORT=7681
RUST_LOG=info
POLLING_INTERVAL_SECS=10
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR
```

**Save**: Ctrl+O → Enter → Ctrl+X

### Bước 5: Build project

```bash
cargo build --release
```

⏳ Đợi ~5-10 phút

### Bước 6: Test run

```bash
./target/release/sui-invariant-monitor
```

Nếu thấy "Starting API server on 0.0.0.0:7681" → OK!
Press Ctrl+C để stop.

### Bước 7: Update Supervisor config

```bash
nano /etc/supervisor/conf.d/sui-monitor.conf
```

**Paste (xóa hết nội dung cũ):**
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
environment=PATH="/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin",RUST_LOG="info",SUI_RPC_URL="https://fullnode.mainnet.sui.io:443",PORT="7681",POLLING_INTERVAL_SECS="10",DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR"
```

**Save**: Ctrl+O → Enter → Ctrl+X

### Bước 8: Restart Supervisor

```bash
supervisorctl reread
supervisorctl update
supervisorctl restart sui-monitor
supervisorctl status sui-monitor
```

Phải thấy: `RUNNING`

### Bước 9: Test backend

```bash
curl http://localhost:7681/health
```

Response: `{"status":"ok","uptime_secs":...}`

---

## 2️⃣ Cloudflare Tunnel - HTTPS

### Bước 1: Stop tunnel cũ (nếu đang chạy)

```bash
# Tìm process
ps aux | grep cloudflared

# Kill nếu có
pkill cloudflared
```

### Bước 2: Start tunnel mới

```bash
cloudflared tunnel --url http://localhost:7681
```

**Lưu URL mới** (dạng `https://xxx.trycloudflare.com`)

### Bước 3: Test từ Mac

```bash
# Thay xxx bằng URL tunnel của bạn
curl https://xxx.trycloudflare.com/health
```

---

## 3️⃣ Frontend - Vercel

### Bước 1: Update API URL trong code

Trên Mac:

```bash
cd /Users/admin/Downloads/sui-invariant-monitor/frontend
```

**Edit `src/api/client.ts`:**
```typescript
const API_BASE = import.meta.env.VITE_API_URL || 
  'https://YOUR_NEW_TUNNEL_URL.trycloudflare.com';
```

**Edit `.env.production`:**
```env
VITE_API_URL=https://YOUR_NEW_TUNNEL_URL.trycloudflare.com
```

### Bước 2: Commit changes

```bash
cd /Users/admin/Downloads/sui-invariant-monitor
git add frontend/src/api/client.ts frontend/.env.production
git commit -m "Update API URL to new Cloudflare Tunnel"
git push origin main
```

### Bước 3: Disconnect Vercel từ repo cũ

1. Go to https://vercel.com/dashboard
2. Click vào project `sui-invariant-monitor`
3. Settings → Git → Disconnect

### Bước 4: Connect Vercel với repo mới

1. Vercel Dashboard → Add New → Project
2. Import Git Repository
3. Connect GitHub account (nếu chưa)
4. Select: `Phu-Nhuan-Builder/sui-invariant-monitor`
5. Configure:
   - **Framework Preset**: Vite
   - **Root Directory**: `frontend`
   - **Build Command**: `npm run build`
   - **Output Directory**: `dist`
6. Environment Variables:
   - Key: `VITE_API_URL`
   - Value: `https://YOUR_TUNNEL_URL.trycloudflare.com`
7. Click **Deploy**

### Bước 5: Wait for deployment

⏳ ~2-3 phút

Vercel sẽ cho bạn URL mới (hoặc giữ URL cũ nếu same project name).

---

## 4️⃣ Update READMEs (Optional)

Nếu Vercel URL thay đổi, update trong code:

```bash
cd /Users/admin/Downloads/sui-invariant-monitor

# Edit README.md
# Thay đổi Live Demo URLs

git add README.md
git commit -m "Update production URLs"
git push origin main
```

---

## ✅ Verification Checklist

### Backend (VPS)
- [ ] Code pulled from new repo
- [ ] `.env` file created
- [ ] Build successful
- [ ] Supervisor running
- [ ] `curl http://localhost:7681/health` works

### Cloudflare Tunnel
- [ ] Tunnel running
- [ ] New HTTPS URL obtained
- [ ] `curl https://xxx.trycloudflare.com/health` works

### Frontend (Vercel)
- [ ] Disconnected from old repo
- [ ] Connected to new repo
- [ ] Deployment successful
- [ ] Environment variables set
- [ ] No mixed content errors

### Integration
- [ ] Frontend can call backend API
- [ ] Network switching works (mainnet/testnet)
- [ ] AI analysis works
- [ ] Add/remove invariants works

---

## 🔧 Troubleshooting

### Backend không start
```bash
# Check logs
supervisorctl tail -f sui-monitor

# Manual test
cd ~/sui-invariant-monitor/backend
./target/release/sui-invariant-monitor
```

### Tunnel không connect
```bash
# Restart tunnel
pkill cloudflared
cloudflared tunnel --url http://localhost:7681
```

### Frontend lỗi Mixed Content
- Check API URL trong `client.ts` có đúng HTTPS không
- Redeploy Vercel với `--force`

### Vercel deployment failed
- Check build logs
- Verify `frontend/` directory structure
- Check environment variables

---

## 📝 Summary

**Thời gian ước tính**: ~30 phút

**Kết quả**:
- ✅ Backend running on VPS with new code
- ✅ HTTPS via Cloudflare Tunnel
- ✅ Frontend deployed on Vercel from new repo
- ✅ Full integration working

**Production URLs** (sau khi hoàn tất):
- Frontend: `https://sui-invariant-monitor.vercel.app` (hoặc URL mới)
- Backend: `https://YOUR_TUNNEL_URL.trycloudflare.com`

---

© 2026 Phú Nhuận Builder

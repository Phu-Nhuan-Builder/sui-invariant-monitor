# Cloudflare Tunnel Setup - Free HTTPS for VPS Backend

## 🎯 Goal
Get free HTTPS URL for VPS backend: `https://sui-monitor.your-domain.workers.dev`

## 📋 Prerequisites
- VPS backend running on port 7681 ✅
- Cloudflare account (free)

## 🚀 Setup Steps

### 1. SSH into VPS
```bash
ssh -p 1443 root@n2.ckey.vn
```

### 2. Install cloudflared
```bash
# Download cloudflared
wget https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb

# Install
dpkg -i cloudflared-linux-amd64.deb

# Verify
cloudflared --version
```

### 3. Login to Cloudflare
```bash
cloudflared tunnel login
```

This will:
1. Open a browser URL
2. Login to Cloudflare
3. Select domain (or create free `.workers.dev` subdomain)
4. Authorize tunnel

**Copy the URL** and open in browser on your Mac to authenticate.

### 4. Create Tunnel
```bash
# Create tunnel
cloudflared tunnel create sui-monitor

# This creates:
# - Tunnel ID
# - Credentials file: ~/.cloudflared/<TUNNEL-ID>.json
```

**Save the Tunnel ID** shown in output!

### 5. Create Config File
```bash
mkdir -p ~/.cloudflared
nano ~/.cloudflared/config.yml
```

**Paste (replace TUNNEL-ID with your actual ID):**
```yaml
tunnel: TUNNEL-ID
credentials-file: /root/.cloudflared/TUNNEL-ID.json

ingress:
  - hostname: sui-monitor.YOUR-DOMAIN.workers.dev
    service: http://localhost:7681
  - service: http_status:404
```

**Save**: Ctrl+O, Enter, Ctrl+X

### 6. Route DNS
```bash
# Replace with your tunnel ID and hostname
cloudflared tunnel route dns sui-monitor sui-monitor.YOUR-DOMAIN.workers.dev
```

### 7. Run Tunnel (Test)
```bash
cloudflared tunnel run sui-monitor
```

You should see:
```
Connection registered
```

**Test from Mac:**
```bash
curl https://sui-monitor.YOUR-DOMAIN.workers.dev/health
```

### 8. Setup as Service (Auto-start)
```bash
# Stop test run (Ctrl+C)

# Install as service
cloudflared service install

# Start service
systemctl start cloudflared
systemctl enable cloudflared

# Check status
systemctl status cloudflared
```

## 🎉 Done!

Your backend is now available at:
```
https://sui-monitor.YOUR-DOMAIN.workers.dev
```

## 📝 Update Frontend

```typescript
// frontend/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'https://sui-monitor.YOUR-DOMAIN.workers.dev';
```

```env
# frontend/.env.production
VITE_API_URL=https://sui-monitor.YOUR-DOMAIN.workers.dev
```

```bash
cd frontend
vercel --prod
```

## 🔧 Manage Tunnel

```bash
# View logs
journalctl -u cloudflared -f

# Restart
systemctl restart cloudflared

# Stop
systemctl stop cloudflared

# List tunnels
cloudflared tunnel list

# Delete tunnel
cloudflared tunnel delete sui-monitor
```

## 💡 Alternative: Quick Tunnel (No Domain)

If you don't want to setup domain:

```bash
# Quick tunnel (temporary URL)
cloudflared tunnel --url http://localhost:7681
```

This gives you a temporary HTTPS URL like:
```
https://random-name.trycloudflare.com
```

**Note**: This URL changes every restart!

## 🆓 Cost
- **100% FREE** - No credit card needed
- Unlimited bandwidth
- Automatic HTTPS

---

**Next**: After tunnel is running, update frontend and redeploy!

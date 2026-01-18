# Deploy Backend to Render.com - Quick Guide

## ✅ Prerequisites
- GitHub repo: https://github.com/phunhuanbuilder/sui-invariant-monitor
- Render account: https://render.com (sign up with GitHub)

## 🚀 Deploy Steps

### 1. Login to Render
Go to https://dashboard.render.com

### 2. Create New Web Service
- Click **"New +"** → **"Web Service"**
- Click **"Build and deploy from a Git repository"**
- Click **"Connect account"** (if not connected)

### 3. Select Repository
- Find: `phunhuanbuilder/sui-invariant-monitor`
- Click **"Connect"**

### 4. Configure Service
Render will auto-detect `render.yaml`. Verify settings:

- **Name**: `sui-invariant-monitor`
- **Region**: Singapore
- **Branch**: `main`
- **Root Directory**: Leave empty (render.yaml specifies `./backend`)
- **Environment**: Docker
- **Docker Build Context**: `./backend`
- **Dockerfile Path**: `./backend/Dockerfile`

### 5. Environment Variables
Already configured in `render.yaml`:
- `SUI_RPC_URL`: https://fullnode.mainnet.sui.io:443
- `PORT`: 8080
- `RUST_LOG`: info
- `POLLING_INTERVAL_SECS`: 10

**Add manually** (not in yaml):
- `DISCORD_WEBHOOK_URL`: `https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR`

### 6. Plan Selection
- **Free**: Good for testing (spins down after inactivity)
- **Starter ($7/month)**: Always on, better performance ⭐ **Recommended**

### 7. Deploy
- Click **"Create Web Service"**
- Wait ~5-10 minutes for build

### 8. Get HTTPS URL
After deployment:
```
https://sui-invariant-monitor.onrender.com
```

## 📝 Update Frontend

### Update API URL
```typescript
// frontend/src/api/client.ts
const API_BASE = import.meta.env.VITE_API_URL || 'https://sui-invariant-monitor.onrender.com';
```

```env
# frontend/.env.production
VITE_API_URL=https://sui-invariant-monitor.onrender.com
```

### Redeploy Frontend
```bash
cd frontend
vercel --prod
```

## ✅ Test
```bash
curl https://sui-invariant-monitor.onrender.com/health
```

## 🔄 Auto-Deploy
Render auto-deploys on every push to `main` branch!

## 📊 Monitor
- **Dashboard**: https://dashboard.render.com
- **Logs**: Click service → "Logs" tab
- **Metrics**: Click service → "Metrics" tab

## 💰 Cost
- **Free**: $0 (with limitations)
- **Starter**: $7/month (recommended for production)

---

**Note**: First deploy takes ~10 minutes (building Rust). Subsequent deploys use cache (~2-3 minutes).

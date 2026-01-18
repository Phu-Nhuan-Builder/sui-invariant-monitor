# Render.com Deployment Guide

## 🚀 Deploying Backend to Render

### Option 1: Via Dashboard (Recommended)

1. **Login to Render**: Go to https://render.com and sign in with GitHub

2. **New Web Service**:
   - Click "New +" → "Web Service"
   - Connect your GitHub repository: `phunhuanbuilder/sui-invariant-monitor`
   - Or use: https://github.com/phunhuanbuilder/sui-invariant-monitor

3. **Configure Service**:
   - **Name**: `sui-invariant-monitor`
   - **Region**: Singapore
   - **Branch**: `main`
   - **Root Directory**: `backend`
   - **Environment**: Docker
   - **Plan**: Starter ($7/month) or Free

4. **Environment Variables**:
   Click "Advanced" and add:
   ```
   SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
   PORT=8080
   RUST_LOG=info
   POLLING_INTERVAL_SECS=10
   DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/1462214251841192099/Na5kWHbCHDQrA3-mZ8BtSw6WoXV2c4ayagK-LNxBvd4KqU_2N9jCbNAxjEy6zYnf_9JR
   ```

5. **Health Check**:
   - **Health Check Path**: `/health`

6. **Deploy**:
   - Click "Create Web Service"
   - Wait ~5-10 minutes for build

### Option 2: Via render.yaml (GitOps)

1. **Commit render.yaml** to your repo (already created)

2. **Blueprint**:
   - Dashboard → "New +" → "Blueprint"
   - Select your repo
   - Render will auto-detect `render.yaml`

3. **Apply**:
   - Review settings
   - Click "Apply"

## 📡 After Deployment

Your backend will be available at:
```
https://sui-invariant-monitor.onrender.com
```

### Update Frontend

Update `frontend/.env.production`:
```env
VITE_API_URL=https://sui-invariant-monitor.onrender.com
```

Update hardcoded fallback in `frontend/src/api/client.ts`:
```typescript
const API_BASE = import.meta.env.VITE_API_URL || 'https://sui-invariant-monitor.onrender.com';
```

### Redeploy Frontend on Vercel

```bash
cd frontend
vercel --prod
```

## 🔧 Advantages of Render

✅ **Easier than Fly.io**: No CLI needed, just use dashboard
✅ **Auto-deploy**: Commits to main branch auto-deploy
✅ **Free SSL**: Automatic HTTPS
✅ **Simple logs**: Easy to view in dashboard
✅ **Health checks**: Built-in monitoring
✅ **No machine management**: Just works

## 💰 Pricing

- **Free Plan**: 
  - ✅ Good for testing
  - ⚠️ Spins down after inactivity (cold starts ~30s)
  - ⚠️ 750 hours/month limit
  
- **Starter Plan ($7/month)**:
  - ✅ Always on
  - ✅ No spin down
  - ✅ Better performance
  - **RECOMMENDED for production**

## 🐛 Troubleshooting

### Build fails
- Check Dockerfile path in dashboard
- Ensure Root Directory = `backend`
- View build logs in dashboard

### Health check fails
- Verify app listens on `0.0.0.0:8080`
- Check `/health` endpoint returns 200
- Increase Health Check Grace Period to 300s

### Cannot connect
- Verify service is "Live" (green)
- Check logs for startup errors
- Test health endpoint: `https://your-app.onrender.com/health`

## 📚 Render Docs

- Dashboard: https://dashboard.render.com
- Docs: https://render.com/docs
- Status: https://status.render.com

---

**Author**: Phú Nhuận Builder
**Email**: phunhuanbuilder@gmail.com

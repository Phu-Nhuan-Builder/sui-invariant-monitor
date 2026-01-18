# Sui Invariant Monitor - Frontend ⚛️

React + TypeScript frontend for the Sui Invariant Monitor. Clean Swiss-style interface for AI-powered contract analysis and real-time monitoring.

## 🌐 Production

- **Live App**: https://sui-invariant-monitor.vercel.app
- **Hosting**: Vercel (auto-deploy on push)
- **Status**: ✅ Production Ready

## ✨ Features

- **AI Contract Analysis**: Analyze Sui packages with frontier AI models
- **Network Switching**: Toggle between Mainnet and Testnet
- **Real-time Monitoring**: Live invariant status updates
- **Bulk Actions**: Add all suggested invariants at once
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Swiss Style**: Minimalist design with Sui blue branding

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
# Clone repository
git clone https://github.com/phunhuanbuilder/sui-invariant-monitor.git
cd sui-invariant-monitor/frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

App runs on `http://localhost:5173`

### Environment Variables

Create `.env.local` for local development:

```env
# Backend API URL (optional, has fallback)
VITE_API_URL=http://localhost:8080
```

Production uses `.env.production`:

```env
# Production backend (Cloudflare Tunnel)
VITE_API_URL=https://combo-ripe-nat-occur.trycloudflare.com
```

## 📁 Project Structure

```
frontend/
├── src/
│   ├── main.tsx             # Entry point
│   ├── App.tsx              # Root component
│   ├── components/
│   │   ├── Layout.tsx       # Main layout with header/footer
│   │   ├── NetworkSwitcher.tsx  # Mainnet/Testnet toggle
│   │   ├── AnalyzeContractForm.tsx  # AI analysis form
│   │   ├── InvariantCard.tsx    # Invariant display card
│   │   ├── StatusBadge.tsx      # Status indicator
│   │   └── ComputationDetails.tsx  # Computation display
│   ├── pages/
│   │   ├── Overview.tsx     # Main dashboard
│   │   └── InvariantDetail.tsx  # Invariant details page
│   ├── api/
│   │   └── client.ts        # API client functions
│   ├── context/
│   │   └── NetworkContext.tsx   # Network state management
│   ├── hooks/
│   │   └── useInvariants.ts     # React Query hooks
│   ├── types/
│   │   └── invariant.ts     # TypeScript types
│   └── styles/
│       └── index.css        # Global styles
├── public/
│   └── logo.svg             # Phú Nhuận Builder logo
├── index.html               # HTML template
├── vite.config.ts           # Vite configuration
├── tsconfig.json            # TypeScript config
└── package.json             # Dependencies
```

## 🎨 Design System

### Color Palette

```css
/* Primary - White */
--color-white: #ffffff;

/* Secondary - Sui Blue */
--color-sui-blue: #4da2ff;
--color-sui-blue-dark: #3d8ae6;
--color-sui-blue-light: #e6f3ff;

/* Tertiary - Black */
--color-black: #000000;
--color-gray-dark: #333333;
--color-gray-medium: #666666;
--color-gray-light: #f5f5f5;

/* Status Colors */
--color-success: #10b981;
--color-error: #ef4444;
--color-warning: #f59e0b;
```

### Typography

- **Font Family**: System fonts (San Francisco, Segoe UI, Roboto)
- **Headings**: Bold, large size
- **Body**: Regular weight, readable size
- **Code**: Monospace for package IDs

### Components

- **Cards**: White background, subtle shadow, rounded corners
- **Buttons**: Sui blue primary, white secondary
- **Badges**: Color-coded status indicators
- **Forms**: Clean inputs with focus states

## 🧩 Key Components

### NetworkSwitcher

Toggle between Mainnet and Testnet.

```tsx
import { NetworkSwitcher } from './components/NetworkSwitcher';

<NetworkSwitcher />
```

### AnalyzeContractForm

AI-powered contract analysis form.

```tsx
import { AnalyzeContractForm } from './components/AnalyzeContractForm';

<AnalyzeContractForm 
  onAnalysisComplete={(result) => console.log(result)} 
/>
```

### InvariantCard

Display invariant with status and actions.

```tsx
import { InvariantCard } from './components/InvariantCard';

<InvariantCard invariant={invariant} />
```

## 🔧 Development

### Commands

```bash
# Development server
npm run dev

# Type checking
npm run typecheck

# Build for production
npm run build

# Preview production build
npm run preview

# Lint
npm run lint
```

### API Client

All API calls are in `src/api/client.ts`:

```typescript
import { analyzePackage, fetchInvariants } from './api/client';

// Analyze package
const result = await analyzePackage({
  package_id: '0x2',
  llm_provider: 'openrouter',
  model: 'anthropic/claude-opus-4.5',
  network: 'mainnet'
});

// Fetch invariants
const invariants = await fetchInvariants();
```

### React Query

Using TanStack Query for data fetching:

```typescript
import { useInvariants } from './hooks/useInvariants';

const { data: invariants, isLoading } = useInvariants();
```

### Network Context

Access network state anywhere:

```typescript
import { useNetwork } from './context/NetworkContext';

const { network, setNetwork } = useNetwork();
```

## 📦 Deployment

### Vercel (Production)

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy to production
vercel --prod
```

Auto-deploys on every push to `main` branch.

### Build Locally

```bash
# Build for production
npm run build

# Output in dist/ folder
# Deploy dist/ to any static hosting
```

### Environment Variables (Vercel)

Set in Vercel Dashboard:
1. Go to Project Settings → Environment Variables
2. Add `VITE_API_URL` = `https://combo-ripe-nat-occur.trycloudflare.com`
3. Redeploy

## 🧪 Testing

### Manual Testing Checklist

- [ ] Network switching works
- [ ] Package analysis completes
- [ ] Invariants display correctly
- [ ] Add to monitoring works
- [ ] Remove from monitoring works
- [ ] Status badges update
- [ ] Responsive on mobile
- [ ] No console errors

### Browser Compatibility

- ✅ Chrome 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ✅ Edge 90+

## 🎯 Performance

- **Bundle Size**: ~200KB (gzipped)
- **First Load**: <2s on 3G
- **Lighthouse Score**: 95+ (Performance)
- **Code Splitting**: Automatic via Vite
- **Image Optimization**: SVG logos only

## 🔒 Security

- **HTTPS Only**: Production enforces HTTPS
- **No Secrets**: API keys entered by user, not stored
- **CORS**: Backend properly configured
- **Input Validation**: Package IDs validated before API calls

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### Code Style

- Use TypeScript for type safety
- Follow React best practices
- Use functional components and hooks
- Keep components small and focused
- Add comments for complex logic

## 📄 License

MIT License - see LICENSE file for details

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- Built for: First Movers Sprint 2026

---

© 2026 Phú Nhuận Builder

**Live App**: https://sui-invariant-monitor.vercel.app

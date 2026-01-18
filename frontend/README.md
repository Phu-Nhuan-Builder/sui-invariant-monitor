# Sui Invariant Monitor - Frontend

Modern React + TypeScript frontend for the Sui Invariant Monitor. Features a clean Swiss-style design with AI-powered contract analysis and real-time invariant monitoring.

## 🎨 Design System

### Color Palette
Based on Minimalism & Swiss Style principles:

- **Primary (60%)**: White `#FFFFFF`
- **Secondary (30%)**: Sui Blue `#4DA2FF`
- **Tertiary (10%)**: Black `#000000`

### Typography
- **Font**: Inter (sans-serif)
- **Sizes**: 12px - 28px
- **Weights**: 400 (regular), 500 (medium), 600 (semibold)

### Principles
- Clean & simple
- Generous whitespace
- High contrast
- Geometric shapes
- Function over form

## 🏗️ Architecture

```
frontend/
├── src/
│   ├── main.tsx                  # App entry point
│   ├── App.tsx                   # Root component + routing
│   │
│   ├── components/               # Reusable components
│   │   ├── Layout.tsx           # Main layout with header/footer
│   │   ├── NetworkSwitcher.tsx  # Mainnet/Testnet switcher
│   │   ├── AnalyzeContractForm.tsx  # AI analysis form
│   │   ├── InvariantCard.tsx    # Invariant display card
│   │   ├── StatusBadge.tsx      # OK/Violated/Error badge
│   │   └── ComputationDetails.tsx
│   │
│   ├── pages/                    # Page components
│   │   ├── Overview.tsx         # Main dashboard
│   │   └── InvariantDetail.tsx  # Detail view
│   │
│   ├── context/                  # React Context
│   │   └── NetworkContext.tsx   # Network state management
│   │
│   ├── hooks/                    # Custom hooks
│   │   └── useInvariants.ts     # React Query hooks
│   │
│   ├── api/                      # API client
│   │   └── client.ts            # Backend API calls
│   │
│   ├── types/                    # TypeScript types
│   │   └── invariant.ts
│   │
│   └── styles/                   # Global styles
│       └── index.css            # Swiss-style design system
│
├── public/
│   ├── favicon.svg
│   └── phunhuanbuilder-logo-zoom.png
│
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 🚀 Getting Started

### Prerequisites
- **Node.js** 18+ or 20+
- **npm** or **pnpm**

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

The app will be available at `http://localhost:5173`

### Environment Variables

Create `.env.local`:
```env
VITE_API_URL=http://localhost:8080
```

For production:
```env
VITE_API_URL=https://your-backend-api.com
```

## 📦 Scripts

```bash
# Development
npm run dev          # Start dev server with HMR

# Production
npm run build        # Build for production
npm run preview      # Preview production build

# Quality
npm run typecheck    # TypeScript type checking
npm run lint         # ESLint (if configured)
```

## 🧩 Key Components

### AnalyzeContractForm
AI-powered contract analysis interface:
- Package ID input
- Module name (optional)
- LLM provider selection (OpenRouter/Ollama)
- Model selection with auto-detection for Ollama
- Bulk "Add All" or individual "Add to Monitoring" buttons

```tsx
<AnalyzeContractForm />
```

### NetworkSwitcher
Dropdown for switching between networks:
- Mainnet
- Testnet
- Updates NetworkContext for all API calls

```tsx
<NetworkSwitcher />
```

### InvariantCard
Display card for each invariant:
- ID and name
- Status badge (OK/Violated/Error)
- Description
- Last evaluation time
- Remove button (−)

```tsx
<InvariantCard invariant={invariant} />
```

### Layout
Main layout wrapper with:
- Header (logo, status, network switcher)
- Main content area
- Footer (Phú Nhuận Builder branding)

```tsx
<Layout>
  <Outlet />
</Layout>
```

## 🔌 API Integration

### React Query Setup

```tsx
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});
```

### Custom Hooks

```tsx
// Fetch all invariants
const { data: invariants, isLoading, error } = useInvariants();

// Fetch monitor status
const { data: status } = useMonitorStatus();

// Fetch specific invariant
const { data: invariant } = useQuery({
  queryKey: ['invariant', id],
  queryFn: () => fetchInvariant(id),
});
```

### API Functions

```typescript
// Analysis
analyzePackage(request: AnalyzeRequest): Promise<AnalyzeResponse>

// Invariants
fetchInvariants(): Promise<InvariantResult[]>
fetchInvariant(id: string): Promise<InvariantResult>
addSuggestedInvariants(request: AddInvariantsRequest)
removeInvariant(request: RemoveInvariantRequest)

// Status
fetchStatus(): Promise<MonitorStatus>
```

## 🎯 Features

### 1. AI Contract Analysis
- Enter any Sui package ID
- Select LLM provider (OpenRouter or Ollama)
- Choose AI model from dropdown
- Ollama: Auto-detect installed local models
- View suggested invariants with severity levels

### 2. Invariant Management
- **Bulk Add**: Add all suggested invariants at once
- **Individual Add**: Select specific invariants
- **Remove**: Click − button to remove from monitoring
- **Confirmation**: Prompt before removing

### 3. Real-time Monitoring
- Auto-refresh every 30 seconds
- Visual status indicators
- Click card for detailed view
- Computation breakdown with formula and inputs

### 4. Network Switching
- Switch between Mainnet and Testnet
- URL updates automatically
- Persists in React Context

## 🎨 Styling

### CSS Architecture
- **Variables**: CSS custom properties for colors, spacing, typography
- **Utilities**: Reusable utility classes
- **Components**: Component-specific styles
- **Responsive**: Mobile-first breakpoints

### Key CSS Variables

```css
:root {
  /* Colors */
  --white: #ffffff;
  --black: #000000;
  --sui-blue: #4da2ff;
  --gray: #e5e5e5;
  
  /* Spacing */
  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 12px;
  --space-lg: 16px;
  --space-xl: 24px;
  
  /* Typography */
  --font-sans: 'Inter', system-ui, sans-serif;
  --font-mono: 'JetBrains Mono', monospace;
}
```

### Component Classes

```css
.btn                    /* Primary button */
.btn-secondary          /* Secondary button */
.btn-icon-remove        /* Remove icon button */
.invariant-card         /* Invariant display card */
.status-badge           /* Status indicator */
.suggested-invariant    /* AI suggestion card */
```

## 🔧 Configuration

### Vite Config
```typescript
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:8080'  // Backend proxy
    }
  }
})
```

### TypeScript Config
- Strict mode enabled
- Path aliases configured
- React JSX runtime

## 🚀 Deployment

### Build for Production

```bash
npm run build
```

Output in `dist/` directory.

### Deploy to Vercel

```bash
npm install -g vercel
vercel --prod
```

### Deploy to Netlify

```bash
npm install -g netlify-cli
netlify deploy --prod --dir=dist
```

### Environment Variables
Set in deployment platform:
```
VITE_API_URL=https://your-backend-api.com
```

## 🧪 Development Tips

### Hot Module Replacement
Vite provides instant HMR. Changes reflect immediately without full reload.

### React Query DevTools
Install for debugging:
```bash
npm install @tanstack/react-query-devtools
```

Add to App.tsx:
```tsx
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

<ReactQueryDevtools initialIsOpen={false} />
```

### Type Safety
Always run typecheck before committing:
```bash
npm run typecheck
```

## 🎨 Design Guidelines

### Adding New Components
1. Follow Swiss style principles (minimal, geometric)
2. Use CSS variables for colors/spacing
3. Add proper TypeScript interfaces
4. Keep components focused and reusable

### Color Usage
- **White background** for main areas
- **Sui Blue** for interactive elements and accents
- **Black** for primary text
- **Gray** for borders and secondary text

### Typography
- Use `var(--font-sans)` for UI text
- Use `var(--font-mono)` for code/formulas
- Keep font sizes between 12px-28px

## 📚 Dependencies

### Core
- `react` - UI library
- `react-dom` - React DOM renderer
- `react-router-dom` - Routing
- `@tanstack/react-query` - Data fetching & caching

### Build Tools
- `vite` - Build tool & dev server
- `typescript` - Type safety
- `@vitejs/plugin-react` - React plugin for Vite

## 🐛 Troubleshooting

### "Cannot connect to backend"
- Check `VITE_API_URL` in `.env.local`
- Ensure backend is running on correct port
- Check CORS configuration in backend

### "Ollama models not appearing"
- Verify Ollama is running: `ollama list`
- Check Ollama URL in settings (default: `http://localhost:11434`)
- Click refresh button (↻) to re-fetch models

### "Invariants disappear after adding"
- This is expected! Analysis results persist until page refresh
- Invariant Status section only shows confirmed invariants
- Use "Invariant Status" grid to view monitored invariants

## 👨‍💻 Author

**Phú Nhuận Builder**
- Email: phunhuanbuilder@gmail.com
- Built for: First Movers Sprint 2026

---

© 2026 Phú Nhuận Builder. Built for First Movers Sprint 2026

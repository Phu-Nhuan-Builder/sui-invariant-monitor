# Sui Invariant Monitor - Frontend

React + TypeScript frontend for the Sui Invariant Monitor.

## Prerequisites

- Node.js 18+
- npm or yarn

## Quick Start

```bash
# Install dependencies
npm install

# Run development server (assumes backend on localhost:8080)
npm run dev

# Build for production
npm run build
```

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_API_URL` | Backend API URL | (uses proxy in dev) |

For production, set `VITE_API_URL` to your deployed backend URL.

## Pages

- **`/`** - Overview of all invariants with status badges
- **`/invariant/:id`** - Detailed view of specific invariant

## Development

The dev server proxies `/api` and `/health` requests to `localhost:8080`. 
Make sure the backend is running before starting the frontend.

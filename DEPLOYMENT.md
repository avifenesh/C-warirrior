# Code Warrior Deployment Guide

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│     Vercel      │────▶│    Railway      │────▶│      Neon       │
│  (Frontend)     │     │  (Rust API)     │     │  (PostgreSQL)   │
│                 │     │                 │     │                 │
│  Static SvelteKit    │  code-warrior-api│     │  User accounts  │
│  + API_URL env  │     │  + DATABASE_URL │     │  Save slots     │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Prerequisites

- [Vercel CLI](https://vercel.com/docs/cli): `npm i -g vercel`
- [Railway CLI](https://docs.railway.app/develop/cli): `npm i -g @railway/cli`
- GitHub account (for integrations)

## Step 1: Set Up Neon Database

1. Go to [neon.tech](https://neon.tech) and create a new project
2. Name it `code-warrior`
3. Copy the connection string (looks like `postgres://user:pass@host/db?sslmode=require`)

### Run Migrations

```bash
# Set your connection string
export DATABASE_URL="postgres://user:pass@host/db?sslmode=require"

# Run migrations
psql $DATABASE_URL -f migrations/001_create_users.sql
psql $DATABASE_URL -f migrations/002_create_save_slots.sql
psql $DATABASE_URL -f migrations/003_create_user_devices.sql
```

## Step 2: Deploy Backend to Railway

1. Go to [railway.app](https://railway.app) and create a new project
2. Connect your GitHub repo
3. Railway will auto-detect the Dockerfile in `src-api/`

### Add Environment Variables in Railway Dashboard:

```
DATABASE_URL=<your-neon-connection-string>
JWT_SECRET=<generate-a-random-secret>
RUST_LOG=info
CORS_ORIGIN=https://your-app.vercel.app
```

### Or Use Railway CLI:

```bash
# Login to Railway
railway login

# Link to project
railway link

# Deploy
railway up

# Get your API URL
railway domain
```

Your API will be at: `https://code-warrior-api-production.up.railway.app`

## Step 3: Deploy Frontend to Vercel

1. Go to [vercel.com](https://vercel.com) and import your GitHub repo
2. Set root directory to `src-ui`
3. Framework preset: SvelteKit

### Add Environment Variable:

```
API_URL=https://code-warrior-api-production.up.railway.app
```

### Or Use Vercel CLI:

```bash
# Login to Vercel
vercel login

# Deploy (from project root)
vercel --prod

# Set environment variable
vercel env add API_URL production
# Enter: https://your-railway-url.up.railway.app
```

## Step 4: Connect Integrations (Optional)

### Railway + Neon Integration

Railway has native Neon integration:
1. In Railway dashboard → Add Plugin → Neon
2. This auto-provisions DATABASE_URL

### Vercel + Railway Integration

1. In Vercel dashboard → Integrations → Railway
2. Auto-sync deployment URLs

## Environment Variables Summary

### Railway (Backend)
| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | Neon PostgreSQL connection string |
| `JWT_SECRET` | Secret for signing JWT tokens |
| `RUST_LOG` | Log level (info, debug, trace) |
| `CORS_ORIGIN` | Frontend URL for CORS |
| `PORT` | Auto-set by Railway |

### Vercel (Frontend)
| Variable | Description |
|----------|-------------|
| `API_URL` | Railway backend URL |

## Updating CORS After Deployment

Once you have your Vercel URL, update `CORS_ORIGIN` in Railway:

```bash
railway variables set CORS_ORIGIN=https://code-warrior.vercel.app
```

## Testing Deployment

```bash
# Test API health
curl https://your-api.up.railway.app/health

# Test frontend
open https://code-warrior.vercel.app
```

## Local Development

```bash
# Start frontend (web dev)
cd src-ui && npm run dev

# Start API locally
cd src-api && cargo run
```

## Troubleshooting

### CORS Errors
- Ensure `CORS_ORIGIN` matches your Vercel domain exactly
- Check Railway logs: `railway logs`

### Database Connection
- Verify `DATABASE_URL` is set correctly
- Check Neon dashboard for connection limits

### Build Failures
- Railway: Check Dockerfile builds locally with `docker build -f src-api/Dockerfile .`
- Vercel: Run `npm run build:web` locally first

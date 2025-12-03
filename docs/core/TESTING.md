# Code Warrior: Testing Protocol

**Every code change MUST be tested on web (HTTP/WASM).**

## Production URLs

| Service | URL |
|---------|-----|
| **Frontend (Vercel)** | https://code-warrior-seven.vercel.app |
| **API (Railway)** | https://code-warrior-api-production.up.railway.app |

## Local Development Setup

**Web (HTTP Backend) - Local:**
```bash
cd src-api && cargo run                              # Terminal 1: API server
cd src-ui && API_URL=http://localhost:3000 npm run dev  # Terminal 2: Frontend
```

## Testing Scope

- **Web (HTTP/WASM)** - Primary and only supported path
  - Production: https://code-warrior-seven.vercel.app
  - Local: http://localhost:1420
  - Uses HTTP API to backend; WASM handles local projection when available

## Testing Checklist

**Before any PR or deployment:**

- [ ] **No Regressions**: Existing features still work
- [ ] **No UI Breaks**: All components render correctly
- [ ] **Full Flow**: Start → Play → Submit code → Complete level
- [ ] **Movement**: WASD navigation works
- [ ] **Interactions**: E key triggers terminals/NPCs
- [ ] **Code Submission**: C code compiles and validates
- [ ] **State Sync**: Game state persists between actions

## Backend Communication

**When modifying `src-ui/src/lib/backend/`:**

1. **Keep HTTP API stable** - Test web after changes
2. **Keep interfaces identical** - WASM and HTTP implement the same `Backend` interface

## UI/UX Testing

- **Visual inspection**: Screenshots at key states
- **Console cleanliness**: No unexpected errors (404s, exceptions)
- **Performance**: No lag during movement/state updates
- **Error handling**: Graceful failures with user feedback

## Automated Deployment & Validation

**Local E2E only (run after every change):**

```bash
./tools/test-local-e2e.sh
```

This script:
1. Assumes local API (`cd src-api && cargo run`) and frontend (`cd src-ui && API_URL=http://localhost:3000 npm run dev`) are running
2. Runs full Playwright E2E tests against `localhost` (same flow as prod)

**Local E2E then deploy (one-shot flow):**

```bash
./tools/test-local-and-deploy.sh
```

This script:
1. Calls `test-local-e2e.sh` (aborts if local tests fail)
2. Then calls `./tools/deploy-and-validate.sh` to deploy and validate production

**Production-only deploy & validation:**

```bash
./tools/deploy-and-validate.sh
```

This script:
1. Deploys API to Railway
2. Deploys frontend to Vercel
3. Waits for propagation
4. Runs Playwright validation (API health, levels, frontend, game flow, movement)
5. Reports success/failure with production URLs

**Rule of thumb:** run `test-local-e2e.sh` after every code change; use `test-local-and-deploy.sh` or `deploy-and-validate.sh` only when you actually want to deploy.


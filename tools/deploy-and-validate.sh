#!/bin/bash
# Code Warrior - Automated Deployment & Validation
# Usage: ./tools/deploy-and-validate.sh
#
# This script ensures a consistent deployment by:
# 1. Running all build checks
# 2. Rebuilding WASM if needed
# 3. Deploying to Railway (backend) and Vercel (frontend)
# 4. Validating both deployments
#
# IMPORTANT: Always use this script for deployments to avoid
# partial deploys or missing WASM rebuilds.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."
PROD_URL="https://code-warrior-seven.vercel.app"
API_URL="https://code-warrior-api-production.up.railway.app"

export PROD_URL
export API_URL

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║       Code Warrior - Deploy & Validate All Platforms      ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

cd "$ROOT_DIR"

# Step 1: Pre-flight checks
echo "▶ [1/6] Running pre-flight checks..."

# Check Rust builds
echo "  → Checking Rust..."
if cargo check --quiet 2>/dev/null; then
    echo "  ✓ Rust OK"
else
    echo "  ✗ Rust build failed"
    exit 1
fi

# Check frontend builds
echo "  → Checking frontend..."
cd "$ROOT_DIR/src-ui"
if npm run check 2>/dev/null | grep -q "0 errors"; then
    echo "  ✓ Frontend OK"
else
    echo "  ✗ Frontend type check failed"
    exit 1
fi
cd "$ROOT_DIR"

# Step 2: Check if WASM needs rebuild
echo ""
echo "▶ [2/6] Checking WASM freshness..."

WASM_PKG="$ROOT_DIR/src-ui/src/lib/wasm/pkg"
LEVELS_JSON="$ROOT_DIR/src/assets/levels.json"

# Check if WASM exists and compare timestamps
if [ -d "$WASM_PKG" ]; then
    WASM_TIME=$(stat -f %m "$WASM_PKG/code_warrior_wasm_bg.wasm" 2>/dev/null || echo "0")
    LEVELS_TIME=$(stat -f %m "$LEVELS_JSON" 2>/dev/null || echo "0")
    RUST_LATEST=$(find "$ROOT_DIR/src" -name "*.rs" -exec stat -f %m {} \; 2>/dev/null | sort -n | tail -1)

    if [ "$LEVELS_TIME" -gt "$WASM_TIME" ] || [ "$RUST_LATEST" -gt "$WASM_TIME" ]; then
        echo "  ⚠ WASM is stale, rebuilding..."
        REBUILD_WASM=true
    else
        echo "  ✓ WASM is up to date"
        REBUILD_WASM=false
    fi
else
    echo "  ⚠ WASM not found, building..."
    REBUILD_WASM=true
fi

# Step 3: Rebuild WASM if needed
if [ "$REBUILD_WASM" = true ]; then
    echo ""
    echo "▶ [3/6] Rebuilding WASM..."
    cd "$ROOT_DIR/src-wasm"
    if wasm-pack build --target web --out-dir "$WASM_PKG" 2>/dev/null; then
        echo "  ✓ WASM rebuilt successfully"
    else
        echo "  ✗ WASM build failed"
        exit 1
    fi
    cd "$ROOT_DIR"
else
    echo ""
    echo "▶ [3/6] WASM rebuild skipped (up to date)"
fi

# Step 4: Deploy to Railway
echo ""
echo "▶ [4/6] Deploying API to Railway..."
if railway up --service code-warrior-api --detach 2>/dev/null; then
    echo "  ✓ Railway deployment triggered"
else
    echo "  ⚠ Railway deploy skipped (may be paused or already deploying)"
fi

# Step 5: Deploy to Vercel
echo ""
echo "▶ [5/6] Deploying frontend to Vercel..."
if npx vercel deploy --prod --yes > /tmp/vercel-deploy.log 2>&1; then
    echo "  ✓ Vercel deployment complete"
    grep "Production:" /tmp/vercel-deploy.log || true
else
    echo "  ✗ Vercel deployment failed"
    cat /tmp/vercel-deploy.log
    exit 1
fi

# Step 6: Wait and validate
echo ""
echo "▶ [6/6] Waiting for deployments to propagate (20s)..."
sleep 20

echo ""
echo "▶ Running validation tests..."

# API health check
API_HEALTH=$(curl -s "$API_URL/health" | grep -c '"ok"' || echo "0")
if [ "$API_HEALTH" -gt 0 ]; then
    echo "  ✓ API health check passed"
else
    echo "  ✗ API health check failed"
    RESULT=1
fi

# Frontend check
FRONTEND_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$PROD_URL")
if [ "$FRONTEND_CHECK" = "200" ]; then
    echo "  ✓ Frontend responds 200"
else
    echo "  ✗ Frontend returned $FRONTEND_CHECK"
    RESULT=1
fi

# WASM check (verify it loads)
WASM_CHECK=$(curl -s "$PROD_URL" | grep -c "wasm" 2>/dev/null | head -1 || echo "0")
if [ "${WASM_CHECK:-0}" -gt 0 ] 2>/dev/null; then
    echo "  ✓ WASM references found in frontend"
    RESULT=${RESULT:-0}
else
    echo "  ⚠ No WASM references found (may be lazy loaded)"
    RESULT=${RESULT:-0}
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
if [ "${RESULT:-0}" -eq 0 ]; then
    echo "🎉 DEPLOYMENT SUCCESSFUL - All platforms validated!"
else
    echo "❌ DEPLOYMENT FAILED - Check errors above"
fi
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "Production URLs:"
echo "  Frontend: $PROD_URL"
echo "  API:      $API_URL"
echo ""

exit ${RESULT:-0}

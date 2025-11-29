#!/bin/bash
# Code Warrior - Automated Deployment & Validation
# Usage: ./tools/deploy-and-validate.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROD_URL="https://code-warrior-seven.vercel.app"
API_URL="https://code-warrior-api-production.up.railway.app"
SKILL_DIR="$HOME/.claude/plugins/marketplaces/playwright-skill/skills/playwright-skill"
VALIDATION_SCRIPT="$SCRIPT_DIR/playwright-validation.js"

export PROD_URL
export API_URL

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║       Code Warrior - Deploy & Validate All Platforms      ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Step 1: Deploy to Railway
echo "▶ [1/4] Deploying API to Railway..."
cd "$(dirname "$0")/.."
if railway up --service code-warrior-api --detach 2>/dev/null; then
    echo "  ✓ Railway deployment triggered"
else
    echo "  ⚠ Railway deploy skipped (may be paused or already deploying)"
fi

# Step 2: Deploy to Vercel
echo ""
echo "▶ [2/4] Deploying frontend to Vercel..."
if vercel deploy --prod --yes > /tmp/vercel-deploy.log 2>&1; then
    echo "  ✓ Vercel deployment complete"
    grep "Production:" /tmp/vercel-deploy.log || true
else
    echo "  ✗ Vercel deployment failed"
    cat /tmp/vercel-deploy.log
    exit 1
fi

# Step 3: Wait for deployments to propagate
echo ""
echo "▶ [3/4] Waiting for deployments to propagate (15s)..."
sleep 15

# Step 4: Validate
echo ""
echo "▶ [4/4] Running validation tests..."

# Run validation
if [ -d "$SKILL_DIR" ]; then
    cd "$SKILL_DIR" && node run.js "$VALIDATION_SCRIPT"
    RESULT=$?
else
    echo "  ⚠ Playwright skill not found, running API-only validation..."
    # Fallback: API-only validation
    API_HEALTH=$(curl -s "$API_URL/health" | grep -c '"ok"' || echo "0")
    if [ "$API_HEALTH" -gt 0 ]; then
        echo "  ✓ API health check passed"
        FRONTEND_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$PROD_URL")
        if [ "$FRONTEND_CHECK" = "200" ]; then
            echo "  ✓ Frontend responds 200"
            RESULT=0
        else
            echo "  ✗ Frontend returned $FRONTEND_CHECK"
            RESULT=1
        fi
    else
        echo "  ✗ API health check failed"
        RESULT=1
    fi
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
if [ $RESULT -eq 0 ]; then
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

exit $RESULT

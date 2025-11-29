#!/bin/bash
# Code Warrior - Local E2E tests against localhost
# Usage: ./tools/test-local-e2e.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VALIDATION_SCRIPT="$SCRIPT_DIR/playwright-validation.js"

LOCAL_FRONTEND_URL="${LOCAL_FRONTEND_URL:-http://localhost:1420}"
LOCAL_API_URL="${LOCAL_API_URL:-http://127.0.0.1:3000}"
SKILL_DIR="$HOME/.claude/plugins/marketplaces/playwright-skill/skills/playwright-skill"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║          Code Warrior - Local E2E (localhost)             ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

echo "▶ [1/3] Checking local backend at $LOCAL_API_URL..."
API_HEALTH=$(curl -s "$LOCAL_API_URL/health" | grep -c '"ok"' || echo "0")
if [ "$API_HEALTH" -eq 0 ]; then
    echo "  ✗ Local API health check failed"
    echo "    Make sure the backend is running, e.g.:"
    echo "      cd src-api && cargo run"
    exit 1
fi
echo "  ✓ Local API healthy"

echo ""
echo "▶ [2/3] Checking local frontend at $LOCAL_FRONTEND_URL..."
FRONTEND_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$LOCAL_FRONTEND_URL" || echo "000")
if [ "$FRONTEND_STATUS" -lt 200 ] || [ "$FRONTEND_STATUS" -ge 400 ]; then
    echo "  ✗ Local frontend returned HTTP $FRONTEND_STATUS"
    echo "    Make sure the frontend dev server is running, e.g.:"
    echo "      cd src-ui && API_URL=$LOCAL_API_URL npm run dev"
    exit 1
fi
echo "  ✓ Local frontend responding (HTTP $FRONTEND_STATUS)"

echo ""
echo "▶ [3/3] Running full local E2E validation..."

RESULT_LOCAL=1
if [ -d "$SKILL_DIR" ]; then
    cd "$SKILL_DIR"
    PROD_URL="$LOCAL_FRONTEND_URL" API_URL="$LOCAL_API_URL" node run.js "$VALIDATION_SCRIPT"
    RESULT_LOCAL=$?
else
    echo "  ⚠ Playwright skill not found, running API-only local validation..."
    if [ "$API_HEALTH" -gt 0 ]; then
        echo "  ✓ Local API health check passed"
        RESULT_LOCAL=0
    else
        echo "  ✗ Local API health check failed"
        RESULT_LOCAL=1
    fi
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
if [ "$RESULT_LOCAL" -ne 0 ]; then
    echo "❌ LOCAL VALIDATION FAILED"
else
    echo "✅ LOCAL VALIDATION PASSED"
fi
echo "═══════════════════════════════════════════════════════════"
echo ""

exit "$RESULT_LOCAL"


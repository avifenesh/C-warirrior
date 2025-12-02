#!/bin/bash
# Code Warrior - Local E2E tests against localhost
#
# Usage:
#   ./tools/test-local-e2e.sh           # Smoke tests (~30s)
#   ./tools/test-local-e2e.sh --full    # Full L01-L05 tests (~2min)
#   ./tools/test-local-e2e.sh --api     # API tests only
#   ./tools/test-local-e2e.sh --legacy  # Run old playwright-validation.js

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEST_RUNNER="$SCRIPT_DIR/run-tests.js"
LEGACY_SCRIPT="$SCRIPT_DIR/playwright-validation.js"

LOCAL_FRONTEND_URL="${LOCAL_FRONTEND_URL:-http://localhost:1420}"
LOCAL_API_URL="${LOCAL_API_URL:-http://127.0.0.1:3000}"
SKILL_DIR="$HOME/.claude/plugins/marketplaces/playwright-skill/skills/playwright-skill"

# Parse arguments
USE_LEGACY=0
TEST_MODE=""
for arg in "$@"; do
    case $arg in
        --legacy) USE_LEGACY=1 ;;
        --full) TEST_MODE="--full" ;;
        --api) TEST_MODE="--api" ;;
        --e2e) TEST_MODE="--e2e" ;;
        -v|--verbose) TEST_MODE="$TEST_MODE --verbose" ;;
    esac
done

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║          Code Warrior - Local E2E (localhost)             ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

echo "▶ [1/3] Checking local backend at $LOCAL_API_URL..."
API_RESPONSE=$(curl -s --max-time 5 "$LOCAL_API_URL/health" 2>/dev/null || echo "")
if echo "$API_RESPONSE" | grep -q '"ok"'; then
    echo "  ✓ Local API healthy"
else
    echo "  ✗ Local API health check failed"
    echo "    Make sure the backend is running, e.g.:"
    echo "      cd src-api && cargo run"
    exit 1
fi

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
echo "▶ [3/3] Running E2E validation..."

RESULT_LOCAL=1

# Use new test runner by default
if [ "$USE_LEGACY" -eq 0 ] && [ -f "$TEST_RUNNER" ]; then
    echo "  Using new test runner (run-tests.js)..."
    cd "$SCRIPT_DIR"
    PROD_URL="$LOCAL_FRONTEND_URL" API_URL="$LOCAL_API_URL" node run-tests.js $TEST_MODE
    RESULT_LOCAL=$?
# Fall back to legacy script via playwright skill
elif [ -d "$SKILL_DIR" ]; then
    echo "  Using legacy playwright-validation.js..."
    cd "$SKILL_DIR"
    PROD_URL="$LOCAL_FRONTEND_URL" API_URL="$LOCAL_API_URL" node run.js "$LEGACY_SCRIPT"
    RESULT_LOCAL=$?
else
    echo "  ⚠ No test runner available, running API-only validation..."
    node "$SCRIPT_DIR/tests/api/health.test.js" 2>/dev/null || true
    RESULT_LOCAL=$?
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


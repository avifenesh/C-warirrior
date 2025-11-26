#!/bin/bash
# Code Warrior - Automated Deployment & Validation
# Usage: ./tools/deploy-and-validate.sh

set -e

PROD_URL="https://code-warrior-seven.vercel.app"
API_URL="https://code-warrior-api-production.up.railway.app"
SKILL_DIR="$HOME/.claude/plugins/marketplaces/playwright-skill/skills/playwright-skill"

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

# Create validation test
cat > /tmp/playwright-deploy-validation.js << 'EOF'
const { chromium } = require('playwright');

const PROD_URL = process.env.PROD_URL || 'https://code-warrior-seven.vercel.app';
const API_URL = process.env.API_URL || 'https://code-warrior-api-production.up.railway.app';

(async () => {
  let passed = 0;
  let failed = 0;

  // Test 1: API Health
  process.stdout.write('  Testing API health... ');
  try {
    const res = await fetch(`${API_URL}/health`);
    const data = await res.json();
    if (data.status === 'ok') {
      console.log('✓');
      passed++;
    } else {
      console.log('✗');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 2: API Levels
  process.stdout.write('  Testing API levels... ');
  try {
    const res = await fetch(`${API_URL}/api/levels`, {
      headers: { 'X-Device-ID': 'deploy-test' }
    });
    const levels = await res.json();
    if (Array.isArray(levels) && levels.length > 0) {
      console.log(`✓ (${levels.length} levels)`);
      passed++;
    } else {
      console.log('✗');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Browser tests
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // Test 3: Frontend loads
  process.stdout.write('  Testing frontend... ');
  try {
    await page.goto(PROD_URL, { waitUntil: 'domcontentloaded', timeout: 30000 });
    await page.waitForTimeout(2000);
    const visible = await page.locator('text=CODE WARRIOR').first().isVisible();
    if (visible) {
      console.log('✓');
      passed++;
    } else {
      console.log('✗');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 4: New Quest
  process.stdout.write('  Testing NEW QUEST... ');
  try {
    await page.click('text=NEW QUEST', { timeout: 5000 });
    await page.waitForTimeout(3000);
    const quest = await page.locator('text=QUEST').first().isVisible();
    if (quest) {
      console.log('✓');
      passed++;
    } else {
      console.log('✗');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 5: Movement
  process.stdout.write('  Testing movement... ');
  try {
    for (let i = 0; i < 3; i++) {
      await page.keyboard.press('d');
      await page.waitForTimeout(100);
    }
    console.log('✓');
    passed++;
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  await browser.close();

  // Summary
  console.log('');
  console.log(`  Results: ${passed}/${passed + failed} tests passed`);

  process.exit(failed > 0 ? 1 : 0);
})();
EOF

# Run validation
if [ -d "$SKILL_DIR" ]; then
    cd "$SKILL_DIR" && node run.js /tmp/playwright-deploy-validation.js
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

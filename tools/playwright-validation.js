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


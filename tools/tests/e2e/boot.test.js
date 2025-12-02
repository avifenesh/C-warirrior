/**
 * E2E Boot and World Map Tests
 *
 * Tests the initial boot sequence and world map functionality.
 */

const { chromium } = require('playwright');

const PROD_URL = process.env.PROD_URL || 'http://localhost:1420';

async function test(name, fn) {
  const start = Date.now();
  try {
    await fn();
    console.log(`  ✓ ${name} (${Date.now() - start}ms)`);
    return true;
  } catch (e) {
    console.log(`  ✗ ${name}: ${e.message}`);
    return false;
  }
}

async function runBootTests() {
  console.log('\n▶ E2E Boot Tests\n');
  let passed = 0;
  let failed = 0;

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // Track console errors
  const errors = [];
  page.on('console', msg => {
    if (msg.type() === 'error') errors.push(msg.text());
  });

  try {
    // Boot sequence
    if (await test('Page loads without crash', async () => {
      await page.goto(PROD_URL, { waitUntil: 'domcontentloaded', timeout: 30000 });
    })) passed++; else failed++;

    // Boot screen shows
    if (await test('Boot screen appears', async () => {
      // Either boot screen or world map should show
      const bootOrMap = await page.waitForSelector('.world-map-container, text=CODE WARRIOR', { timeout: 20000 });
      if (!bootOrMap) throw new Error('Neither boot nor map visible');
    })) passed++; else failed++;

    // World map loads
    if (await test('World map loads after boot', async () => {
      await page.waitForSelector('.world-map-container', { timeout: 30000 });
    })) passed++; else failed++;

    // Level markers present
    if (await test('Level markers are rendered', async () => {
      const markers = await page.locator('.level-marker').count();
      if (markers === 0) throw new Error('No markers found');
      if (markers < 10) throw new Error(`Only ${markers} markers, expected more`);
    })) passed++; else failed++;

    // At least one available level
    if (await test('At least one level is available', async () => {
      const available = await page.locator('.level-marker.available').count();
      if (available === 0) throw new Error('No available levels');
    })) passed++; else failed++;

    // Title displayed
    if (await test('Map title "THE REALM OF C" displayed', async () => {
      const title = await page.locator('.map-title, text=THE REALM OF C').isVisible();
      if (!title) throw new Error('Title not visible');
    })) passed++; else failed++;

    // No console errors
    if (await test('No critical console errors', async () => {
      const criticalErrors = errors.filter(e =>
        e.includes('Error') && !e.includes('favicon') && !e.includes('404')
      );
      if (criticalErrors.length > 0) {
        throw new Error(`Console errors: ${criticalErrors.join('; ').slice(0, 100)}`);
      }
    })) passed++; else failed++;

  } finally {
    await browser.close();
  }

  console.log(`\n  Results: ${passed}/${passed + failed} passed\n`);
  return failed === 0;
}

// Run if called directly
if (require.main === module) {
  runBootTests().then(success => process.exit(success ? 0 : 1));
}

module.exports = { runBootTests };

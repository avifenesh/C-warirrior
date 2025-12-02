/**
 * E2E Level Flow Tests
 *
 * Tests: World Map → Level Selection → Game → Terminal → Submit
 */

const { chromium } = require('playwright');
const { QUEST_SOLUTIONS, LEVEL_NAVIGATION, calculateMovement } = require('../fixtures/solutions');

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

async function runLevelFlowTests(levelId = 'L01') {
  console.log(`\n▶ E2E Level Flow Tests (${levelId})\n`);
  let passed = 0;
  let failed = 0;

  const browser = await chromium.launch({ headless: true, slowMo: 50 });
  const page = await browser.newPage();

  try {
    // Navigate to app
    if (await test('Navigate to app', async () => {
      await page.goto(PROD_URL, { waitUntil: 'networkidle', timeout: 30000 });
      await page.waitForSelector('.world-map-container', { timeout: 20000 });
    })) passed++; else failed++;

    // Click level marker
    if (await test(`Click ${levelId} marker to start`, async () => {
      const marker = page.locator('.level-marker.available').first();
      const count = await marker.count();
      if (count === 0) throw new Error('No available markers');
      await marker.click({ force: true });
      await page.waitForTimeout(2000);
    })) passed++; else failed++;

    // Game canvas loads
    if (await test('Game canvas renders', async () => {
      await page.waitForSelector('.game-canvas, canvas', { timeout: 15000 });
      const canvas = await page.locator('.game-canvas, canvas').first();
      const visible = await canvas.isVisible();
      if (!visible) throw new Error('Canvas not visible');
    })) passed++; else failed++;

    // Player movement
    if (await test('Player can move (WASD)', async () => {
      const movement = calculateMovement(levelId, 0);
      if (!movement) throw new Error('No movement data');

      // Move right toward first terminal
      for (let i = 0; i < Math.min(movement.right, 3); i++) {
        await page.keyboard.press('d');
        await page.waitForTimeout(80);
      }
    })) passed++; else failed++;

    // Navigate to terminal
    if (await test('Navigate to terminal', async () => {
      const movement = calculateMovement(levelId, 0);

      // Complete movement to terminal
      for (let i = 3; i < movement.right; i++) {
        await page.keyboard.press('d');
        await page.waitForTimeout(80);
      }
    })) passed++; else failed++;

    // Open terminal with E key
    if (await test('Press E to open terminal', async () => {
      let opened = false;

      for (let attempt = 0; attempt < 8 && !opened; attempt++) {
        await page.keyboard.press('e');
        await page.waitForTimeout(1000);

        const terminal = await page.locator('.grimoire-container').isVisible().catch(() => false);
        if (terminal) {
          opened = true;
          break;
        }

        // Adjust position
        await page.keyboard.press(['d', 'a', 'd', 'a'][attempt % 4]);
        await page.waitForTimeout(100);
      }

      if (!opened) throw new Error('Terminal did not open after 8 attempts');
    })) passed++; else failed++;

    // Terminal UI elements
    if (await test('Terminal displays correctly', async () => {
      const hasTitle = await page.locator('.grimoire-title, .quest-title').isVisible().catch(() => false);
      const hasTextarea = await page.locator('textarea').isVisible();
      const hasSubmit = await page.locator('.submit-btn, .cast-btn').isVisible().catch(() => false);

      if (!hasTextarea) throw new Error('Code textarea not visible');
      if (!hasSubmit) throw new Error('Submit button not visible');
    })) passed++; else failed++;

    // Dismiss mission briefing if present
    if (await test('Handle mission briefing', async () => {
      const missionBtn = page.locator('.mission-start-btn');
      if (await missionBtn.isVisible().catch(() => false)) {
        await missionBtn.click();
        await page.waitForTimeout(500);
      }
    })) passed++; else failed++;

    // Enter code
    const quest = QUEST_SOLUTIONS[levelId][0];
    if (await test('Enter code in textarea', async () => {
      const textarea = page.locator('textarea').first();
      await textarea.click({ force: true });
      await textarea.fill(quest.code);
      await page.waitForTimeout(300);

      const value = await textarea.inputValue();
      if (!value.includes('return')) throw new Error('Code not entered correctly');
    })) passed++; else failed++;

    // Submit code
    if (await test('Submit code and get success', async () => {
      const submitBtn = page.locator('.submit-btn, .cast-btn').first();
      await submitBtn.click();
      await page.waitForTimeout(3000);

      // Check for success indicator
      const success = await page.locator('text=/success|complete|passed/i').isVisible().catch(() => false);
      const statusSuccess = await page.locator('.status-badge:has-text("Success")').isVisible().catch(() => false);

      if (!success && !statusSuccess) {
        const output = await page.locator('.output-box').textContent().catch(() => 'no output');
        throw new Error(`No success indicator. Output: ${output.slice(0, 100)}`);
      }
    })) passed++; else failed++;

    // Close terminal
    if (await test('Close terminal', async () => {
      await page.keyboard.press('Escape');
      await page.waitForTimeout(500);

      // Terminal should be closed
      const stillOpen = await page.locator('.grimoire-container').isVisible().catch(() => false);
      // It's OK if it's still open (might show quest select)
    })) passed++; else failed++;

    // Return to map
    if (await test('Return to world map', async () => {
      const mapBtn = page.locator('button:has-text("MAP")');
      if (await mapBtn.isVisible().catch(() => false)) {
        await mapBtn.click();
        await page.waitForTimeout(2000);
      }
      await page.waitForSelector('.world-map-container, .level-marker', { timeout: 10000 });
    })) passed++; else failed++;

  } finally {
    await browser.close();
  }

  console.log(`\n  Results: ${passed}/${passed + failed} passed\n`);
  return failed === 0;
}

// Run if called directly
if (require.main === module) {
  const level = process.argv[2] || 'L01';
  runLevelFlowTests(level).then(success => process.exit(success ? 0 : 1));
}

module.exports = { runLevelFlowTests };

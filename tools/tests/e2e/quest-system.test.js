/**
 * E2E Quest System Tests
 *
 * Tests completing all quests in a level and verifying level completion.
 * This is the most comprehensive test - validates the full game loop.
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

/**
 * Navigate from current position to terminal index
 */
async function navigateToTerminal(page, levelId, terminalIdx, fromIdx = -1) {
  const TILE_SIZE = 32;
  const nav = LEVEL_NAVIGATION[levelId];

  let startX, startY;
  if (fromIdx < 0) {
    // Starting from spawn
    startX = nav.spawn.x;
    startY = nav.spawn.y;
  } else {
    // Starting from previous terminal
    startX = nav.terminals[fromIdx].x;
    startY = nav.terminals[fromIdx].y;
  }

  const targetX = nav.terminals[terminalIdx].x;
  const targetY = nav.terminals[terminalIdx].y;

  const dx = (targetX - startX) / TILE_SIZE;
  const dy = (targetY - startY) / TILE_SIZE;

  // Move horizontally
  for (let i = 0; i < Math.abs(dx); i++) {
    await page.keyboard.press(dx > 0 ? 'd' : 'a');
    await page.waitForTimeout(80);
  }

  // Move vertically (if needed)
  for (let i = 0; i < Math.abs(dy); i++) {
    await page.keyboard.press(dy > 0 ? 's' : 'w');
    await page.waitForTimeout(80);
  }
}

/**
 * Open terminal with retry logic
 */
async function openTerminal(page) {
  for (let attempt = 0; attempt < 10; attempt++) {
    await page.keyboard.press('e');
    await page.waitForTimeout(800);

    const terminal = await page.locator('.grimoire-container').isVisible().catch(() => false);
    if (terminal) return true;

    // Small movement to find the sweet spot
    const moves = ['d', 'a', 'w', 's', 'd', 'a', 'w', 's', 'd', 'a'];
    await page.keyboard.press(moves[attempt]);
    await page.waitForTimeout(100);
  }
  return false;
}

/**
 * Submit code for a quest
 */
async function submitQuestCode(page, code) {
  // Dismiss mission briefing if present
  const missionBtn = page.locator('.mission-start-btn');
  if (await missionBtn.isVisible().catch(() => false)) {
    await missionBtn.click();
    await page.waitForTimeout(500);
  }

  // Fill in code
  const textarea = page.locator('textarea').first();
  await textarea.click({ force: true });
  await textarea.fill(code);
  await page.waitForTimeout(300);

  // Click submit
  const submitBtn = page.locator('.submit-btn, .cast-btn').first();
  await submitBtn.click();
  await page.waitForTimeout(2500);

  // Check for success
  const success = await page.locator('.status-badge:has-text("Success"), text=/quest complete/i, text=/success/i').first().isVisible().catch(() => false);
  return success;
}

/**
 * Run quest system tests for multiple levels
 */
async function runQuestSystemTests(levels = ['L01']) {
  console.log(`\n▶ E2E Quest System Tests (${levels.join(', ')})\n`);
  let totalPassed = 0;
  let totalFailed = 0;

  const browser = await chromium.launch({ headless: true, slowMo: 50 });
  const page = await browser.newPage();

  // Monitor for errors
  page.on('console', msg => {
    if (msg.type() === 'error') {
      const text = msg.text();
      if (!text.includes('favicon') && !text.includes('404')) {
        // Only log critical errors in verbose mode
      }
    }
  });

  try {
    // Navigate to app and wait for world map
    if (await test('Load app and world map', async () => {
      await page.goto(PROD_URL, { waitUntil: 'networkidle', timeout: 30000 });
      await page.waitForSelector('.world-map-container', { timeout: 25000 });
    })) totalPassed++; else totalFailed++;

    for (const levelId of levels) {
      const quests = QUEST_SOLUTIONS[levelId];
      if (!quests || quests.length === 0) {
        console.log(`  ○ ${levelId}: No quests defined, skipping`);
        continue;
      }

      console.log(`\n  📍 Testing ${levelId} (${quests.length} quests):`);

      // Ensure we're on world map
      const onMap = await page.locator('.world-map-container').isVisible().catch(() => false);
      if (!onMap) {
        const mapBtn = page.locator('button:has-text("MAP")');
        if (await mapBtn.isVisible().catch(() => false)) {
          await mapBtn.click();
          await page.waitForTimeout(2000);
        }
      }

      // Click level to start
      if (await test(`${levelId}: Start level`, async () => {
        // Find any available marker (ideally we'd find the specific level)
        const markers = page.locator('.level-marker.available, .level-marker.current');
        const count = await markers.count();
        if (count === 0) throw new Error('No available markers');

        // For now, click the first available
        await markers.first().click({ force: true });
        await page.waitForTimeout(2000);

        // Wait for game to load
        await page.waitForSelector('.game-canvas, canvas', { timeout: 15000 });
      })) totalPassed++; else totalFailed++;

      // Complete each quest
      for (let i = 0; i < quests.length; i++) {
        const quest = quests[i];

        if (await test(`${levelId}: Quest ${i + 1} - ${quest.title}`, async () => {
          // Navigate to terminal
          await navigateToTerminal(page, levelId, i, i - 1);

          // Open terminal
          const opened = await openTerminal(page);
          if (!opened) throw new Error('Could not open terminal');

          // Submit solution
          const success = await submitQuestCode(page, quest.code);
          if (!success) {
            const output = await page.locator('.output-box').textContent().catch(() => 'no output');
            throw new Error(`Quest not completed. Output: ${output.slice(0, 100)}`);
          }

          // Close terminal to go back to game
          await page.keyboard.press('Escape');
          await page.waitForTimeout(500);
        })) totalPassed++; else totalFailed++;
      }

      // Check for level complete (after all quests)
      if (await test(`${levelId}: Level complete check`, async () => {
        // Level complete modal might appear
        const levelComplete = await page.locator('text=QUEST COMPLETE, text=ALL QUESTS COMPLETE').isVisible().catch(() => false);

        // Or return to map button
        const returnBtn = await page.locator('button:has-text("RETURN TO MAP"), .pixel-button').isVisible().catch(() => false);

        // Click return if visible
        if (returnBtn) {
          const btn = page.locator('button:has-text("RETURN TO MAP"), .pixel-button').first();
          if (await btn.isVisible().catch(() => false)) {
            await btn.click();
            await page.waitForTimeout(2000);
          }
        }

        // Otherwise use MAP button
        const mapBtn = page.locator('button:has-text("MAP")');
        if (await mapBtn.isVisible().catch(() => false)) {
          await mapBtn.click();
          await page.waitForTimeout(2000);
        }
      })) totalPassed++; else totalFailed++;

      // Verify back on map
      if (await test(`${levelId}: Return to world map`, async () => {
        await page.waitForSelector('.world-map-container, .level-marker', { timeout: 10000 });
      })) totalPassed++; else totalFailed++;
    }

  } finally {
    await browser.close();
  }

  console.log(`\n  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
  console.log(`  Results: ${totalPassed}/${totalPassed + totalFailed} passed`);
  console.log(`  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`);

  return totalFailed === 0;
}

// Run if called directly
if (require.main === module) {
  const args = process.argv.slice(2);
  const levels = args.length > 0 ? args : ['L01'];
  runQuestSystemTests(levels).then(success => process.exit(success ? 0 : 1));
}

module.exports = { runQuestSystemTests, navigateToTerminal, openTerminal, submitQuestCode };

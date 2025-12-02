#!/usr/bin/env node
/**
 * Code Warrior Test Runner
 *
 * Usage:
 *   node run-tests.js              # Run smoke tests (~30s)
 *   node run-tests.js --full       # Run full test suite (~2min)
 *   node run-tests.js --api        # Run API tests only
 *   node run-tests.js --e2e        # Run E2E tests only
 *   node run-tests.js --level L01  # Run tests for specific level
 */

const { chromium } = require('playwright');
const path = require('path');

// Configuration
const PROD_URL = process.env.PROD_URL || 'http://localhost:1420';
const API_URL = process.env.API_URL || 'http://127.0.0.1:3000';
const DEVICE_ID = process.env.DEVICE_ID || `test-${Date.now()}`;

// Parse CLI arguments
const args = process.argv.slice(2);
const levelArg = args.find((a, i) => args[i - 1] === '--level') || null;
const flags = {
  full: args.includes('--full'),
  api: args.includes('--api'),
  e2e: args.includes('--e2e') || !!levelArg, // --level implies E2E mode
  smoke: !args.includes('--full') && !args.includes('--api') && !args.includes('--e2e') && !levelArg,
  level: levelArg,
  headless: !args.includes('--headed'),
  verbose: args.includes('--verbose') || args.includes('-v')
};

// Test results tracking
const results = {
  passed: 0,
  failed: 0,
  skipped: 0,
  tests: []
};

// Logging helpers
function log(msg) {
  console.log(msg);
}

function logTest(name, status, duration = null, error = null) {
  const icon = status === 'pass' ? '\x1b[32m✓\x1b[0m' : status === 'fail' ? '\x1b[31m✗\x1b[0m' : '\x1b[33m○\x1b[0m';
  const time = duration ? ` \x1b[90m(${duration}ms)\x1b[0m` : '';
  log(`  ${icon} ${name}${time}`);
  if (error && flags.verbose) {
    log(`    \x1b[31m${error}\x1b[0m`);
  }
  results.tests.push({ name, status, duration, error });
  if (status === 'pass') results.passed++;
  else if (status === 'fail') results.failed++;
  else results.skipped++;
}

// Test wrapper
async function runTest(name, fn) {
  const start = Date.now();
  try {
    await fn();
    logTest(name, 'pass', Date.now() - start);
    return true;
  } catch (e) {
    logTest(name, 'fail', Date.now() - start, e.message);
    return false;
  }
}

// ============================================================================
// API TESTS
// ============================================================================

async function runApiTests() {
  log('\n\x1b[36m▶ API Tests\x1b[0m');

  // Health check
  await runTest('API health check', async () => {
    const res = await fetch(`${API_URL}/health`);
    const data = await res.json();
    if (data.status !== 'ok') throw new Error(`Expected status 'ok', got '${data.status}'`);
  });

  // Levels list
  await runTest('API levels list', async () => {
    const res = await fetch(`${API_URL}/api/levels`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    const levels = await res.json();
    if (!Array.isArray(levels)) throw new Error('Expected array of levels');
    if (levels.length < 5) throw new Error(`Expected at least 5 levels, got ${levels.length}`);
  });

  // Game init
  await runTest('API game init', async () => {
    const res = await fetch(`${API_URL}/api/game/init`, {
      method: 'POST',
      headers: {
        'X-Device-ID': DEVICE_ID,
        'Content-Type': 'application/json'
      },
      body: '{}'
    });
    if (!res.ok) throw new Error(`Init failed: ${res.status}`);
  });

  // Level load
  await runTest('API level load', async () => {
    const res = await fetch(`${API_URL}/api/levels/L01/load`, {
      method: 'POST',
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    if (!res.ok) throw new Error(`Load failed: ${res.status}`);
    const data = await res.json();
    if (!data.level_data) throw new Error('Missing level_data in response');
  });
}

// ============================================================================
// E2E TESTS
// ============================================================================

async function runE2ETests(page, testLevels = ['L01']) {
  const { QUEST_SOLUTIONS, LEVEL_NAVIGATION, calculateMovement } = require('./tests/fixtures/solutions');

  log('\n\x1b[36m▶ E2E Tests\x1b[0m');

  // Boot test
  await runTest('Boot → World Map', async () => {
    await page.goto(PROD_URL, { waitUntil: 'networkidle', timeout: 30000 });
    await page.waitForSelector('.world-map-container', { timeout: 20000 });
    const markers = await page.locator('.level-marker').count();
    if (markers === 0) throw new Error('No level markers found');
  });

  // Level markers
  await runTest('World map has available levels', async () => {
    const available = await page.locator('.level-marker.available').count();
    if (available === 0) throw new Error('No available levels');
  });

  // Test each level
  for (const levelId of testLevels) {
    const quests = QUEST_SOLUTIONS[levelId];
    if (!quests) continue;

    // Click level to start
    await runTest(`${levelId}: Select level`, async () => {
      // If not on world map, navigate back
      const onMap = await page.locator('.world-map-container').isVisible().catch(() => false);
      if (!onMap) {
        // Try to return to map
        const mapBtn = page.locator('button:has-text("MAP")');
        if (await mapBtn.isVisible().catch(() => false)) {
          await mapBtn.click();
          await page.waitForSelector('.world-map-container', { timeout: 10000 });
        }
      }

      // Find and click the level marker (can be available, in_progress, or current - anything not locked)
      // LevelMarker uses classes like .level-marker.available, .level-marker.in_progress, etc.
      const marker = page.locator('.level-marker:not(.locked):not(.completed)').first();
      await marker.waitFor({ state: 'visible', timeout: 10000 });
      await marker.click({ force: true });
      await page.waitForTimeout(2000);

      // Wait for game canvas
      await page.waitForSelector('.game-canvas, canvas', { timeout: 15000 });
      // Wait for assets to load (game needs time to initialize)
      await page.waitForTimeout(3000);
    });

    // Complete all quests in this level
    for (let questIdx = 0; questIdx < quests.length; questIdx++) {
      const quest = quests[questIdx];

      await runTest(`${levelId}: Quest ${questIdx + 1} - ${quest.title}`, async () => {
        // Ensure focus on game container by clicking the canvas area
        const canvas = page.locator('canvas').first();
        await canvas.click({ force: true, position: { x: 400, y: 300 } });
        await page.waitForTimeout(500);

        // Also click the body to ensure window has focus
        await page.evaluate(() => document.body.focus());
        await page.waitForTimeout(200);

        // Screenshot before navigation for debugging
        if (flags.verbose) {
          const fs = require('fs');
          const before = await page.screenshot().catch(() => null);
          if (before) fs.writeFileSync(`/tmp/quest${questIdx + 1}-before.png`, before);
        }

        // Navigate to terminal - each terminal has ABSOLUTE position from spawn
        // (player always starts at spawn after re-entering level)
        const terminalNav = calculateMovement(levelId, questIdx);
        const totalUp = terminalNav?.up || 0;
        const totalDown = terminalNav?.down || 0;
        const totalRight = terminalNav?.right || 0;
        const totalLeft = terminalNav?.left || 0;

        if (flags.verbose) log(`    [DEBUG] Navigating to terminal ${questIdx + 1}: up=${totalUp} down=${totalDown} right=${totalRight} left=${totalLeft}`);

        // Move UP first, then RIGHT (for levels with terminals above spawn)
        for (let i = 0; i < totalUp; i++) {
          await page.keyboard.press('w');
          await page.waitForTimeout(100);
        }
        for (let i = 0; i < totalDown; i++) {
          await page.keyboard.press('s');
          await page.waitForTimeout(100);
        }
        for (let i = 0; i < totalRight; i++) {
          await page.keyboard.press('d');
          await page.waitForTimeout(100);
        }
        for (let i = 0; i < totalLeft; i++) {
          await page.keyboard.press('a');
          await page.waitForTimeout(100);
        }
        await page.waitForTimeout(300);

        // Look for interact prompt and try to open terminal
        let terminalOpened = false;

        // First check if terminal is already open (shouldn't be, but just in case)
        terminalOpened = await page.locator('.grimoire-container').isVisible().catch(() => false);
        if (terminalOpened) {
          if (flags.verbose) log('    [DEBUG] Terminal already open');
        }

        for (let attempt = 0; attempt < 12 && !terminalOpened; attempt++) {
          // Check if interact prompt is visible (means we're near terminal)
          const interactVisible = await page.locator('.interact-prompt').isVisible().catch(() => false);

          if (flags.verbose && attempt === 0) {
            log(`    [DEBUG] After navigation - interact prompt: ${interactVisible}`);
          }

          if (interactVisible) {
            await page.keyboard.press('e');
            await page.waitForTimeout(800);
            terminalOpened = await page.locator('.grimoire-container').isVisible().catch(() => false);
            if (terminalOpened) break;
          }

          // Make small adjustments to find terminal (search pattern)
          // For quest 1: search all directions
          // For quest 2+: only search RIGHT and UP/DOWN (never left, to avoid previous terminals)
          const standardPattern = ['d', 'd', 'a', 'a', 's', 's', 'w', 'w', 'd', 'd', 'a', 'a'];
          const forwardPattern = ['d', 'd', 'd', 'd', 'w', 'w', 's', 's', 'd', 'd', 'd', 'd'];  // Only right and vertical
          const adjustments = questIdx > 0 ? forwardPattern : standardPattern;
          await page.keyboard.press(adjustments[attempt] || 'd');
          await page.waitForTimeout(120);
        }

        if (!terminalOpened) {
          // Last resort: try pressing E repeatedly while moving
          if (flags.verbose) log('    [DEBUG] Terminal not found, trying E repeatedly');
          for (let i = 0; i < 8 && !terminalOpened; i++) {
            await page.keyboard.press('e');
            await page.waitForTimeout(400);
            terminalOpened = await page.locator('.grimoire-container').isVisible().catch(() => false);
            if (!terminalOpened) {
              // For quest 2+: only move RIGHT (never left to avoid previous terminals)
              // For quest 1: alternate right/left
              const moveDir = questIdx > 0 ? 'd' : (i % 2 === 0 ? 'd' : 'a');
              await page.keyboard.press(moveDir);
              await page.waitForTimeout(100);
            }
          }
        }

        if (!terminalOpened) {
          // Take screenshot for debugging
          const screenshot = await page.screenshot().catch(() => null);
          if (screenshot && flags.verbose) {
            const fs = require('fs');
            fs.writeFileSync(`/tmp/quest${questIdx + 1}-failed.png`, screenshot);
            log(`    [DEBUG] Screenshot saved to /tmp/quest${questIdx + 1}-failed.png`);
          }
          throw new Error('Terminal did not open after navigation');
        }

        // Dismiss mission briefing if present (click "BEGIN QUEST")
        const missionBtn = page.locator('.mission-start-btn');
        await page.waitForTimeout(500);
        if (await missionBtn.isVisible().catch(() => false)) {
          await missionBtn.click();
          await page.waitForTimeout(500);
        }

        // Wait for textarea to be ready and fill in solution
        const textarea = page.locator('textarea').first();
        await textarea.waitFor({ state: 'visible', timeout: 5000 });
        await textarea.click({ force: true });
        await page.waitForTimeout(200);

        // Clear existing content and fill with solution
        await textarea.fill(quest.code);
        await page.waitForTimeout(300);

        // Click submit button
        const submitBtn = page.locator('.submit-btn');
        await submitBtn.waitFor({ state: 'visible', timeout: 5000 });
        await submitBtn.click();

        // Poll for success indicators (may take a few seconds)
        // Success can appear as: success badge, passed tests, toast, or quest complete modal
        let questSuccess = false;
        let pollAttempts = 0;
        const maxPollAttempts = 10;

        while (!questSuccess && pollAttempts < maxPollAttempts) {
          await page.waitForTimeout(500);
          pollAttempts++;

          const successBadge = await page.locator('.status-badge.success').isVisible().catch(() => false);
          const testsPassed = await page.locator('.test-count.passed').isVisible().catch(() => false);
          const successToast = await page.locator('.pixel-toast.success').isVisible().catch(() => false);
          // Level complete modal (only appears after ALL quests done)
          const levelCompleteModal = await page.locator('h2.pixel-title').filter({ hasText: 'QUEST COMPLETE' }).isVisible().catch(() => false);

          questSuccess = successBadge || testsPassed || successToast || levelCompleteModal;

          if (flags.verbose && pollAttempts <= 3) {
            log(`    [DEBUG] Poll ${pollAttempts}: badge=${successBadge} tests=${testsPassed} toast=${successToast} levelModal=${levelCompleteModal}`);
          }

          if (questSuccess) {
            if (flags.verbose) log(`    [DEBUG] Success detected at poll ${pollAttempts}`);
            break;
          }
        }

        // Poll for quest complete modal (can appear several seconds after toast)
        // The WASM/frontend processing can be slow - poll for up to 15 seconds
        let levelCompleteModal = false;
        const modalPollStart = Date.now();
        const maxModalWait = 15000; // 15 seconds

        while (!levelCompleteModal && (Date.now() - modalPollStart) < maxModalWait) {
          await page.waitForTimeout(500);
          levelCompleteModal = await page.locator('h2.pixel-title').filter({ hasText: 'QUEST COMPLETE' }).isVisible().catch(() => false);

          if (flags.verbose && !levelCompleteModal && (Date.now() - modalPollStart) % 3000 < 600) {
            log(`    [DEBUG] Waiting for modal... ${Math.floor((Date.now() - modalPollStart) / 1000)}s`);
          }
        }

        if (levelCompleteModal) {
          if (flags.verbose) log(`    [DEBUG] Quest complete modal appeared after ${Math.floor((Date.now() - modalPollStart) / 1000)}s`);
          // Click return button
          const returnBtn = page.locator('button').filter({ hasText: /RETURN|MAP/ }).first();
          if (await returnBtn.isVisible().catch(() => false)) {
            await returnBtn.click();
            if (flags.verbose) log('    [DEBUG] Clicked return to map');
            await page.waitForTimeout(2000);
            // Wait for world map
            await page.waitForSelector('.world-map-container', { timeout: 10000 });
            // Re-enter level if more quests
            if (questIdx < quests.length - 1) {
              if (flags.verbose) log('    [DEBUG] Re-entering level for remaining quests');
              const marker = page.locator('.level-marker:not(.locked):not(.completed)').first();
              await marker.waitFor({ state: 'visible', timeout: 10000 });
              await marker.click({ force: true });
              await page.waitForTimeout(2000);
              await page.waitForSelector('.game-canvas, canvas', { timeout: 15000 });
              // Extra wait for assets to load after re-entry
              await page.waitForTimeout(2000);
            }
          }
        }

        // Handle submission result - may need to search for correct terminal
        let searchAttempts = 0;
        const maxSearchAttempts = 5;

        while (!questSuccess && searchAttempts < maxSearchAttempts) {
          const compileError = await page.locator('.output-box.error').textContent().catch(() => '');
          const feedback = await page.locator('.output-feedback').textContent().catch(() => '');
          const errorMsg = compileError || feedback || '';

          // For quests after first, check if we hit wrong terminal
          const isWrongTerminal = questIdx > 0 && errorMsg.includes('implicit declaration');
          if (!isWrongTerminal) {
            throw new Error(`Quest failed. Error: ${errorMsg || 'Unknown'}`);
          }

          searchAttempts++;
          if (flags.verbose) log(`    [DEBUG] Wrong terminal, searching right (attempt ${searchAttempts})`);

          // Close and move right
          await page.keyboard.press('Escape');
          await page.waitForTimeout(500);
          for (let i = 0; i < 6; i++) {
            await page.keyboard.press('d');
            await page.waitForTimeout(100);
          }

          // Find next terminal
          let opened = false;
          for (let i = 0; i < 12 && !opened; i++) {
            await page.keyboard.press('e');
            await page.waitForTimeout(400);
            opened = await page.locator('.grimoire-container').isVisible().catch(() => false);
            if (!opened) {
              await page.keyboard.press('d');
              await page.waitForTimeout(100);
            }
          }

          if (!opened) {
            throw new Error(`Could not find terminal for quest ${questIdx + 1}`);
          }

          // Submit to new terminal
          const btn = page.locator('.mission-start-btn');
          if (await btn.isVisible().catch(() => false)) {
            await btn.click();
            await page.waitForTimeout(500);
          }
          const ta = page.locator('textarea').first();
          await ta.waitFor({ state: 'visible', timeout: 5000 });
          await ta.fill(quest.code);
          await page.waitForTimeout(300);
          await page.locator('.submit-btn').click();
          await page.waitForTimeout(3000);

          questSuccess = await page.locator('.status-badge.success').isVisible().catch(() => false)
            || await page.locator('.test-count.passed').isVisible().catch(() => false)
            || await page.locator('.pixel-title:has-text("QUEST COMPLETE")').isVisible().catch(() => false);
        }

        if (!questSuccess) {
          throw new Error(`Quest ${questIdx + 1} failed after ${maxSearchAttempts} searches`);
        }

        // If modal didn't appear earlier, close Grimoire if still open
        if (!levelCompleteModal) {
          const grimoireOpen = await page.locator('.grimoire-container').isVisible().catch(() => false);
          if (grimoireOpen) {
            if (flags.verbose) log('    [DEBUG] Closing Grimoire to continue');
            const closeBtn = page.locator('.close-btn').first();
            if (await closeBtn.isVisible().catch(() => false)) {
              await closeBtn.click();
            } else {
              await page.keyboard.press('Escape');
            }
            await page.waitForTimeout(800);
          }

          // Re-focus game container for next quest navigation
          const container = page.locator('[role="application"]').first();
          if (await container.isVisible().catch(() => false)) {
            await container.click({ force: true });
            await page.waitForTimeout(300);
          }
        }
      });
    }

    // Return to map after level
    await runTest(`${levelId}: Return to world map`, async () => {
      // Check if already on map
      const onMap = await page.locator('.world-map-container').isVisible().catch(() => false);
      if (onMap) return;

      // Check for level complete modal with return button
      const returnBtn = page.locator('.pixel-button:has-text("RETURN TO MAP")');
      if (await returnBtn.isVisible().catch(() => false)) {
        await returnBtn.click();
        await page.waitForTimeout(2000);
      }

      // Try MAP button in HUD if still not on map
      const stillNotOnMap = !(await page.locator('.world-map-container').isVisible().catch(() => false));
      if (stillNotOnMap) {
        const mapBtn = page.locator('button:has-text("MAP")');
        if (await mapBtn.isVisible().catch(() => false)) {
          await mapBtn.click();
          await page.waitForTimeout(2000);
        }
      }

      await page.waitForSelector('.world-map-container', { timeout: 10000 });
    });
  }
}

// ============================================================================
// SMOKE TESTS (Quick validation)
// ============================================================================

async function runSmokeTests(page) {
  log('\n\x1b[36m▶ Smoke Tests\x1b[0m');

  // Boot
  await runTest('App boots successfully', async () => {
    await page.goto(PROD_URL, { waitUntil: 'networkidle', timeout: 30000 });
    await page.waitForSelector('.world-map-container, .game-canvas', { timeout: 20000 });
  });

  // World map
  await runTest('World map displays levels', async () => {
    const onMap = await page.locator('.world-map-container').isVisible().catch(() => false);
    if (!onMap) {
      // Navigate to map
      await page.goto(PROD_URL, { waitUntil: 'networkidle' });
      await page.waitForTimeout(3000);
    }
    const markers = await page.locator('.level-marker').count();
    if (markers === 0) throw new Error('No markers');
  });

  // Can start level
  await runTest('Can start L01', async () => {
    // Click any non-locked level (could be available, in_progress, or current)
    const marker = page.locator('.level-marker:not(.locked):not(.completed)').first();
    await marker.waitFor({ state: 'visible', timeout: 10000 });
    await marker.click({ force: true });
    await page.waitForTimeout(3000);
    const canvas = await page.locator('.game-canvas, canvas').isVisible().catch(() => false);
    if (!canvas) {
      // Check for loading state
      const loading = await page.locator('.status-banner').textContent().catch(() => '');
      throw new Error(`Canvas not visible. Status: ${loading}`);
    }
  });
}

// ============================================================================
// MAIN
// ============================================================================

async function main() {
  log('\x1b[1m\x1b[35m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m');
  log('\x1b[1m\x1b[35m  Code Warrior Test Suite\x1b[0m');
  log('\x1b[1m\x1b[35m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m');
  log(`\n  Mode: ${flags.full ? 'Full' : flags.api ? 'API' : flags.e2e ? 'E2E' : 'Smoke'}`);
  log(`  API: ${API_URL}`);
  log(`  Frontend: ${PROD_URL}`);

  const startTime = Date.now();
  let browser = null;

  try {
    // Run API tests
    if (flags.api || flags.full || flags.smoke) {
      await runApiTests();
    }

    // Run browser tests
    if (flags.e2e || flags.full || flags.smoke) {
      browser = await chromium.launch({ headless: flags.headless, slowMo: 50 });
      const page = await browser.newPage();

      // Monitor console for errors
      page.on('console', msg => {
        if (msg.type() === 'error' && flags.verbose) {
          log(`  \x1b[90m[console] ${msg.text()}\x1b[0m`);
        }
      });

      if (flags.smoke) {
        await runSmokeTests(page);
      } else {
        // Full mode tests L01-L05 in sequence (completing each level unlocks the next)
        const levels = flags.level ? [flags.level] : (flags.full ? ['L01', 'L02', 'L03', 'L04', 'L05'] : ['L01']);
        await runE2ETests(page, levels);
      }

      await browser.close();
    }
  } catch (e) {
    log(`\n\x1b[31mFatal error: ${e.message}\x1b[0m`);
    if (browser) await browser.close();
    process.exit(1);
  }

  // Summary
  const duration = ((Date.now() - startTime) / 1000).toFixed(1);
  log('\n\x1b[1m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m');
  log(`\x1b[1m  Results: ${results.passed}/${results.passed + results.failed} passed\x1b[0m`);
  log(`\x1b[1m  Duration: ${duration}s\x1b[0m`);
  log('\x1b[1m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m\n');

  process.exit(results.failed > 0 ? 1 : 0);
}

main();

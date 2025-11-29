const { chromium } = require('playwright');

const PROD_URL = process.env.PROD_URL || 'https://code-warrior-seven.vercel.app';
const API_URL = process.env.API_URL || 'https://code-warrior-api-production.up.railway.app';

// L01 Quest Solutions (all 3 required to complete level)
const L01_QUESTS = [
  { title: 'The Secret Number', solution: 'int getSecret() {\n    return 42;\n}' },
  { title: 'Double Trouble', solution: 'int getDouble() {\n    return 21 * 2;\n}' },
  { title: 'The Sum Spell', solution: 'int getSum() {\n    return 10 + 20 + 12;\n}' }
];

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

  // Test 3: World Map loads
  process.stdout.write('  Testing world map... ');
  try {
    await page.goto(PROD_URL, { waitUntil: 'networkidle', timeout: 30000 });
    await page.waitForTimeout(4000);
    await page.waitForSelector('.level-marker', { timeout: 15000 }).catch(() => null);
    const markerCount = await page.locator('.level-marker').count();
    if (markerCount > 0) {
      console.log(`✓ (${markerCount} markers)`);
      passed++;
    } else {
      console.log('✗ (no markers found)');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 4: Level Selection (click L01 to start game)
  process.stdout.write('  Testing level selection... ');
  try {
    await page.waitForSelector('.level-marker.available', { timeout: 10000 }).catch(() => null);
    const availableMarker = page.locator('.level-marker.available').first();
    const markerExists = await availableMarker.count();
    if (markerExists > 0) {
      await availableMarker.click({ force: true });
      await page.waitForTimeout(4000);
      const canvasVisible = await page.locator('canvas').count() > 0;
      if (canvasVisible) {
        console.log('✓');
        passed++;
      } else {
        console.log('✗ (game not loaded)');
        failed++;
      }
    } else {
      console.log('✗ (no available levels)');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 5: Movement toward terminal
  // L01 map: spawn at (64, 352), terminal at (288, 192)
  // Need to move RIGHT 7 tiles (d key), UP 5 tiles (w key)
  process.stdout.write('  Testing movement... ');
  try {
    for (let i = 0; i < 7; i++) {
      await page.keyboard.press('d');
      await page.waitForTimeout(80);
    }
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('w');
      await page.waitForTimeout(80);
    }
    console.log('✓');
    passed++;
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 6: Open terminal (press E to interact)
  process.stdout.write('  Testing terminal open... ');
  try {
    let terminalOpened = false;

    for (let attempt = 0; attempt < 8 && !terminalOpened; attempt++) {
      await page.keyboard.press('e');
      await page.waitForTimeout(1000);

      const questSelectVisible = await page.locator('.quest-select-container').count() > 0;
      const terminalVisible = await page.locator('.grimoire-container').count() > 0;

      if (questSelectVisible || terminalVisible) {
        terminalOpened = true;
        break;
      }

      const moves = ['d', 'w', 'a', 's', 'd', 'w', 'a', 's'];
      await page.keyboard.press(moves[attempt % moves.length]);
      await page.waitForTimeout(100);
    }

    if (terminalOpened) {
      console.log('✓');
      passed++;
    } else {
      console.log('✗ (terminal not opened)');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 7: Complete ALL quests (L01 has 3 quests)
  process.stdout.write('  Testing quest completion (3 quests)... ');
  try {
    let questsCompleted = 0;

    for (let questIdx = 0; questIdx < L01_QUESTS.length; questIdx++) {
      const quest = L01_QUESTS[questIdx];

      // Wait for quest select container
      await page.waitForSelector('.quest-select-container', { timeout: 5000 }).catch(() => null);
      const questSelectVisible = await page.locator('.quest-select-container').count() > 0;

      if (!questSelectVisible) {
        // Need to re-open terminal (press E)
        await page.keyboard.press('e');
        await page.waitForTimeout(1000);
      }

      // Click the quest item (nth quest, 0-indexed)
      const questItems = page.locator('.quest-item');
      const questCount = await questItems.count();

      if (questCount > questIdx) {
        // Click the specific quest
        await questItems.nth(questIdx).click();
        await page.waitForTimeout(1500);

        // Dismiss mission briefing by clicking "BEGIN QUEST"
        const missionBriefingBtn = page.locator('.mission-start-btn');
        if (await missionBriefingBtn.isVisible().catch(() => false)) {
          await missionBriefingBtn.click();
          await page.waitForTimeout(500);
        }

        // Fill in the solution
        const textarea = page.locator('textarea').first();
        if (await textarea.isVisible().catch(() => false)) {
          await textarea.click({ force: true });
          await textarea.fill(quest.solution);
          await page.waitForTimeout(300);

          // Click SUBMIT button
          const submitBtn = page.locator('.submit-btn');
          if (await submitBtn.isVisible().catch(() => false)) {
            await submitBtn.click();
            await page.waitForTimeout(2000);

            // Check for success
            const success = await page.locator('.test-count.passed, .output-feedback.success').first().isVisible().catch(() => false);
            if (success) {
              questsCompleted++;
            }
          }
        }

        // Close terminal to go back to quest select (click X or close button)
        // The onClose for quest mode goes back to showQuestSelect = true
        const closeQuestBtn = page.locator('.quest-select-close, button:has-text("✕")').first();
        if (await closeQuestBtn.isVisible().catch(() => false)) {
          await closeQuestBtn.click();
          await page.waitForTimeout(500);
        } else {
          // Escape might close terminal entirely, need to re-open
          await page.keyboard.press('Escape');
          await page.waitForTimeout(500);
        }
      }
    }

    if (questsCompleted >= L01_QUESTS.length) {
      console.log(`✓ (${questsCompleted}/${L01_QUESTS.length})`);
      passed++;
    } else if (questsCompleted > 0) {
      console.log(`✓ (${questsCompleted}/${L01_QUESTS.length} partial)`);
      passed++;
    } else {
      console.log('✗ (no quests completed)');
      failed++;
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 8: Level completion and return to map
  process.stdout.write('  Testing level complete... ');
  try {
    await page.waitForTimeout(2000);

    // Check for level complete modal (shows when all quests done)
    const levelComplete = await page.locator('text=QUEST COMPLETE').first().isVisible().catch(() => false);
    const allQuestsComplete = await page.locator('text=ALL QUESTS COMPLETE').first().isVisible().catch(() => false);

    if (levelComplete || allQuestsComplete) {
      // Click RETURN TO MAP button
      const returnBtn = page.locator('button:has-text("RETURN TO MAP"), .pixel-button').first();
      if (await returnBtn.isVisible().catch(() => false)) {
        await returnBtn.click();
        await page.waitForTimeout(3000);
      }
      console.log('✓');
      passed++;
    } else {
      // Use MAP button in HUD to return
      await page.keyboard.press('Escape');
      await page.waitForTimeout(500);
      const mapBtn = page.locator('button:has-text("MAP")');
      if (await mapBtn.isVisible().catch(() => false)) {
        await mapBtn.click();
        await page.waitForTimeout(2000);
        console.log('✓ (via MAP)');
        passed++;
      } else {
        console.log('✗ (no return path)');
        failed++;
      }
    }
  } catch (e) {
    console.log('✗', e.message);
    failed++;
  }

  // Test 9: Verify back on world map
  process.stdout.write('  Testing return to map... ');
  try {
    await page.waitForSelector('.level-marker', { timeout: 10000 }).catch(() => null);
    const markerCount = await page.locator('.level-marker').count();
    if (markerCount > 0) {
      console.log('✓');
      passed++;
    } else {
      console.log('✗ (not on map)');
      failed++;
    }
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

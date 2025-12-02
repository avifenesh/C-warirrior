/**
 * API Health and Endpoint Tests
 *
 * Run standalone: node tests/api/health.test.js
 */

const API_URL = process.env.API_URL || 'http://127.0.0.1:3000';
const DEVICE_ID = process.env.DEVICE_ID || `test-${Date.now()}`;

async function test(name, fn) {
  try {
    await fn();
    console.log(`  ✓ ${name}`);
    return true;
  } catch (e) {
    console.log(`  ✗ ${name}: ${e.message}`);
    return false;
  }
}

async function runApiTests() {
  console.log('\n▶ API Health Tests\n');
  let passed = 0;
  let failed = 0;

  // Health check
  if (await test('GET /health returns ok', async () => {
    const res = await fetch(`${API_URL}/health`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    if (data.status !== 'ok') throw new Error(`Expected 'ok', got '${data.status}'`);
  })) passed++; else failed++;

  // Levels list
  if (await test('GET /api/levels returns array', async () => {
    const res = await fetch(`${API_URL}/api/levels`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const levels = await res.json();
    if (!Array.isArray(levels)) throw new Error('Not an array');
  })) passed++; else failed++;

  // Levels count
  if (await test('GET /api/levels returns 25 levels', async () => {
    const res = await fetch(`${API_URL}/api/levels`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    const levels = await res.json();
    if (levels.length !== 25) throw new Error(`Expected 25, got ${levels.length}`);
  })) passed++; else failed++;

  // L01 is unlocked
  if (await test('L01 is unlocked by default', async () => {
    const res = await fetch(`${API_URL}/api/levels`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    const levels = await res.json();
    const l01 = levels.find(l => l.id === 'L01');
    if (!l01) throw new Error('L01 not found');
    if (!l01.unlocked) throw new Error('L01 is locked');
  })) passed++; else failed++;

  // Game init
  if (await test('POST /api/game/init initializes game', async () => {
    const res = await fetch(`${API_URL}/api/game/init`, {
      method: 'POST',
      headers: {
        'X-Device-ID': DEVICE_ID,
        'Content-Type': 'application/json'
      },
      body: '{}'
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    if (!data.game_phase) throw new Error('Missing game_phase');
  })) passed++; else failed++;

  // Level load
  if (await test('POST /api/levels/L01/load returns level data', async () => {
    // First init game
    await fetch(`${API_URL}/api/game/init`, {
      method: 'POST',
      headers: { 'X-Device-ID': DEVICE_ID }
    });

    const res = await fetch(`${API_URL}/api/levels/L01/load`, {
      method: 'POST',
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    if (!data.level_data) throw new Error('Missing level_data');
    if (!data.render_state) throw new Error('Missing render_state');
  })) passed++; else failed++;

  // Quests endpoint
  if (await test('GET /api/levels/current/quests returns quest list', async () => {
    const res = await fetch(`${API_URL}/api/levels/current/quests`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const quests = await res.json();
    if (!Array.isArray(quests)) throw new Error('Not an array');
    if (quests.length < 3) throw new Error(`Expected 3+ quests, got ${quests.length}`);
  })) passed++; else failed++;

  // Render state
  if (await test('GET /api/game/render-state returns state', async () => {
    const res = await fetch(`${API_URL}/api/game/render-state`, {
      headers: { 'X-Device-ID': DEVICE_ID }
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const state = await res.json();
    if (!state.player) throw new Error('Missing player data');
  })) passed++; else failed++;

  console.log(`\n  Results: ${passed}/${passed + failed} passed\n`);
  return failed === 0;
}

// Run if called directly
if (require.main === module) {
  runApiTests().then(success => process.exit(success ? 0 : 1));
}

module.exports = { runApiTests };

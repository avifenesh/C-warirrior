/**
 * Quest Solutions Database for L01-L05
 * Extracted from src/assets/levels.json
 *
 * Usage: const { QUEST_SOLUTIONS, LEVEL_NAVIGATION } = require('./solutions');
 */

const QUEST_SOLUTIONS = {
  L01: [
    { id: 'L01_Q1', title: 'The Secret Number', code: 'int getSecret() {\n    return 42;\n}' },
    { id: 'L01_Q2', title: 'Double Trouble', code: 'int getDouble() {\n    return 50 * 2;\n}' },
    { id: 'L01_Q3', title: 'The Sum Spell', code: 'int getSum() {\n    return 25 + 35 + 40;\n}' }
  ],
  L02: [
    { id: 'L02_Q1', title: 'Adding Weights', code: 'int addWeight(int item1, int item2) {\n    return item1 + item2;\n}' },
    { id: 'L02_Q2', title: 'Calculate Area', code: 'int calculateArea(int width, int height) {\n    return width * height;\n}' },
    { id: 'L02_Q3', title: 'Triple Sum', code: 'int tripleSum(int a, int b, int c) {\n    return a + b + c;\n}' }
  ],
  L03: [
    { id: 'L03_Q1', title: 'Maximum Value', code: 'int maxValue(int a, int b) {\n    if (a > b) return a;\n    return b;\n}' },
    { id: 'L03_Q2', title: 'Even or Odd', code: 'int isEven(int n) {\n    if (n % 2 == 0) return 1;\n    return 0;\n}' },
    { id: 'L03_Q3', title: 'Clamp Value', code: 'int clamp(int value, int min, int max) {\n    if (value < min) return min;\n    if (value > max) return max;\n    return value;\n}' }
  ],
  L04: [
    { id: 'L04_Q1', title: 'Sum of Strikes', code: 'int sumStrikes(int n) {\n    int sum = 0;\n    for (int i = 1; i <= n; i++) {\n        sum += i;\n    }\n    return sum;\n}' },
    { id: 'L04_Q2', title: 'Factorial', code: 'int factorial(int n) {\n    int result = 1;\n    for (int i = 1; i <= n; i++) {\n        result *= i;\n    }\n    return result;\n}' },
    { id: 'L04_Q3', title: 'Count Divisible', code: 'int countDivisible(int n, int d) {\n    int count = 0;\n    for (int i = 1; i <= n; i++) {\n        if (i % d == 0) count++;\n    }\n    return count;\n}' }
  ],
  L05: [
    { id: 'L05_Q1', title: 'Room Access', code: 'int getRoom(int index) {\n    int rooms[5] = {10, 20, 30, 40, 50};\n    return rooms[index];\n}' },
    { id: 'L05_Q2', title: 'Sum All Rooms', code: 'int sumRooms() {\n    int rooms[5] = {10, 20, 30, 40, 50};\n    int sum = 0;\n    for (int i = 0; i < 5; i++) {\n        sum += rooms[i];\n    }\n    return sum;\n}' },
    { id: 'L05_Q3', title: 'Find Maximum', code: 'int findMax() {\n    int values[5] = {15, 42, 8, 23, 31};\n    int max = values[0];\n    for (int i = 1; i < 5; i++) {\n        if (values[i] > max) max = values[i];\n    }\n    return max;\n}' }
  ]
};

// Navigation data: tile movements from spawn to each terminal
// Calculated from levels.json world_config coordinates:
//   - All positions in pixels, divide by 32 to get tile coordinates
//   - Movement: up/down/left/right = (terminal - spawn) / 32
// ABSOLUTE navigation: each terminal's position from spawn (not incremental)
// Generated via: node tools/extract-nav-coords.js
const LEVEL_NAVIGATION = {
  L01: {
    // Spawn: (32, 224), terminals at y=192 (1 row up)
    // T1: (160,192) → right=(160-32)/32=4, up=(224-192)/32=1
    // T2: (352,192) → right=(352-32)/32=10, up=1
    // T3: (544,192) → right=(544-32)/32=16, up=1
    terminals: [
      { up: 1, right: 4, quest_id: 'L01_Q1' },
      { up: 1, right: 10, quest_id: 'L01_Q2' },
      { up: 1, right: 16, quest_id: 'L01_Q3' }
    ]
  },
  L02: {
    // Spawn: (32, 224), terminals at y=192 (1 row up)
    // T1: (128,192), T2: (320,192), T3: (512,192)
    terminals: [
      { up: 1, right: 3, quest_id: 'L02_Q1' },
      { up: 1, right: 9, quest_id: 'L02_Q2' },
      { up: 1, right: 15, quest_id: 'L02_Q3' }
    ]
  },
  L03: {
    // Spawn: (32, 352), terminals in dungeon layout
    // T1: (512,64), T2: (320,224), T3: (192,320)
    terminals: [
      { up: 9, right: 15, quest_id: 'L03_Q1' },
      { up: 4, right: 9, quest_id: 'L03_Q2' },
      { up: 1, right: 5, quest_id: 'L03_Q3' }
    ]
  },
  L04: {
    // Spawn: (32, 352), terminals in dungeon layout
    // T1: (192,64), T2: (416,160), T3: (128,288)
    terminals: [
      { up: 9, right: 5, quest_id: 'L04_Q1' },
      { up: 6, right: 12, quest_id: 'L04_Q2' },
      { up: 2, right: 3, quest_id: 'L04_Q3' }
    ]
  },
  L05: {
    // Spawn: (32, 320), terminals in dungeon layout
    // T1: (64,64), T2: (288,160), T3: (416,256)
    terminals: [
      { up: 8, right: 1, quest_id: 'L05_Q1' },
      { up: 5, right: 8, quest_id: 'L05_Q2' },
      { up: 2, right: 12, quest_id: 'L05_Q3' }
    ]
  }
};

// World map level marker positions (percentage-based for responsive UI)
const WORLD_MAP_MARKERS = {
  L01: { x: 23, y: 77 },
  L02: { x: 28, y: 66 },
  L03: { x: 37, y: 55 },
  L04: { x: 28, y: 43 },
  L05: { x: 35, y: 32 }
};

// Helper: Get navigation data for a terminal (relative movements)
function getTerminalNav(levelId, terminalIndex = 0) {
  const nav = LEVEL_NAVIGATION[levelId];
  if (!nav || !nav.terminals[terminalIndex]) return null;
  return nav.terminals[terminalIndex];
}

// Legacy helper for backwards compatibility
function calculateMovement(levelId, terminalIndex = 0) {
  const terminal = getTerminalNav(levelId, terminalIndex);
  if (!terminal) return null;
  return {
    right: terminal.right || 0,
    left: terminal.left || 0,
    down: terminal.down || 0,
    up: terminal.up || 0
  };
}

// Helper: Get all quests for a level
function getQuestsForLevel(levelId) {
  return QUEST_SOLUTIONS[levelId] || [];
}

// Helper: Get quest solution by ID
function getQuestSolution(questId) {
  for (const level of Object.values(QUEST_SOLUTIONS)) {
    const quest = level.find(q => q.id === questId);
    if (quest) return quest;
  }
  return null;
}

module.exports = {
  QUEST_SOLUTIONS,
  LEVEL_NAVIGATION,
  WORLD_MAP_MARKERS,
  getTerminalNav,
  calculateMovement,
  getQuestsForLevel,
  getQuestSolution
};

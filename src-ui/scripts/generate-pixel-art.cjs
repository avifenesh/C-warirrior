#!/usr/bin/env node
/**
 * Generate placeholder pixel art assets for Code Warrior
 * Run with: node scripts/generate-pixel-art.cjs
 */

const { createCanvas } = require('canvas');
const fs = require('fs');
const path = require('path');

const TILE_SIZE = 32;

// Color palettes (16-bit style)
const PALETTE = {
    // Terrain
    grass: ['#2d5a27', '#3d7a37', '#4d9a47', '#2d5a27'],
    stone: ['#4a4a4a', '#5a5a5a', '#6a6a6a', '#3a3a3a'],
    wood: ['#8b5a2b', '#9b6a3b', '#ab7a4b', '#7b4a1b'],
    water: ['#1e6091', '#2980b9', '#3498db', '#1e6091'],

    // Walls
    stoneWall: ['#3a3a3a', '#4a4a4a', '#5a5a5a', '#2a2a2a'],
    brick: ['#8b4513', '#a0522d', '#cd853f', '#6b3513'],

    // Interactive
    terminal: ['#1a1a2e', '#16213e', '#0f3460', '#22d3ee'],
    doorWood: ['#5a3a1a', '#6a4a2a', '#7a5a3a', '#4a2a0a'],
    doorIron: ['#4a4a5a', '#5a5a6a', '#6a6a7a', '#3a3a4a'],

    // Player (knight)
    knight: ['#708090', '#a0a0b0', '#d0d0e0', '#506070'], // armor
    knightAccent: ['#daa520', '#ffd700', '#ffec8b', '#b8860b'], // gold trim
    skin: ['#deb887', '#f5deb3', '#ffe4c4', '#d2b48c'],

    // NPC (mentor/wizard)
    wizard: ['#4a0080', '#6a00a0', '#8a00c0', '#3a0060'], // robes
    wizardAccent: ['#c0c0c0', '#d0d0d0', '#e0e0e0', '#a0a0a0'], // silver

    // UI
    uiBg: ['#1a1a2e', '#16213e', '#0f3460', '#0a0a1e'],
    uiAccent: ['#daa520', '#ffd700', '#ffec8b', '#b8860b'],
};

function setPixel(ctx, x, y, color) {
    ctx.fillStyle = color;
    ctx.fillRect(x, y, 1, 1);
}

function drawRect(ctx, x, y, w, h, color) {
    ctx.fillStyle = color;
    ctx.fillRect(x, y, w, h);
}

// ============ TERRAIN TILES ============

function generateGrassTile() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Base grass
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.grass[1]);

    // Add some variation
    for (let i = 0; i < 20; i++) {
        const x = Math.floor(Math.random() * TILE_SIZE);
        const y = Math.floor(Math.random() * TILE_SIZE);
        const colorIdx = Math.floor(Math.random() * PALETTE.grass.length);
        setPixel(ctx, x, y, PALETTE.grass[colorIdx]);
    }

    // Add grass blades
    for (let i = 0; i < 8; i++) {
        const x = Math.floor(Math.random() * (TILE_SIZE - 2)) + 1;
        const y = Math.floor(Math.random() * (TILE_SIZE - 4)) + 2;
        ctx.fillStyle = PALETTE.grass[2];
        ctx.fillRect(x, y, 1, 3);
    }

    return canvas;
}

function generateStoneTile() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Base stone
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.stone[1]);

    // Add cracks/variation
    for (let i = 0; i < 30; i++) {
        const x = Math.floor(Math.random() * TILE_SIZE);
        const y = Math.floor(Math.random() * TILE_SIZE);
        const colorIdx = Math.floor(Math.random() * PALETTE.stone.length);
        setPixel(ctx, x, y, PALETTE.stone[colorIdx]);
    }

    // Stone grid lines
    ctx.fillStyle = PALETTE.stone[3];
    ctx.fillRect(0, 15, TILE_SIZE, 1);
    ctx.fillRect(15, 0, 1, TILE_SIZE);

    return canvas;
}

function generateWaterTile(frame = 0) {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Base water
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.water[1]);

    // Animated wave pattern
    const offset = frame * 4;
    for (let y = 0; y < TILE_SIZE; y += 8) {
        for (let x = 0; x < TILE_SIZE; x++) {
            const waveY = y + Math.floor(Math.sin((x + offset) / 4) * 2);
            if (waveY >= 0 && waveY < TILE_SIZE) {
                setPixel(ctx, x, waveY, PALETTE.water[2]);
            }
        }
    }

    // Add sparkles
    for (let i = 0; i < 3; i++) {
        const x = (frame * 7 + i * 11) % TILE_SIZE;
        const y = (frame * 5 + i * 13) % TILE_SIZE;
        setPixel(ctx, x, y, '#ffffff');
    }

    return canvas;
}

// ============ WALL TILES ============

function generateStoneWall() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Base wall
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.stoneWall[1]);

    // Brick pattern
    for (let row = 0; row < 4; row++) {
        const offset = row % 2 === 0 ? 0 : 8;
        for (let col = 0; col < 3; col++) {
            const x = offset + col * 16;
            const y = row * 8;

            // Brick face
            drawRect(ctx, x + 1, y + 1, 14, 6, PALETTE.stoneWall[2]);

            // Brick highlight (top)
            drawRect(ctx, x + 1, y + 1, 14, 1, PALETTE.stoneWall[2]);

            // Brick shadow (bottom)
            drawRect(ctx, x + 1, y + 6, 14, 1, PALETTE.stoneWall[3]);
        }
    }

    return canvas;
}

function generateWallTop() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Dark void for top of wall (looks like ceiling)
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, '#0a0a0a');

    // Add subtle stone texture at bottom edge
    drawRect(ctx, 0, TILE_SIZE - 4, TILE_SIZE, 4, PALETTE.stoneWall[3]);

    return canvas;
}

// ============ INTERACTIVE TILES ============

function generateTerminal() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Stone base
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.stone[1]);

    // Terminal pedestal
    drawRect(ctx, 6, 20, 20, 12, PALETTE.terminal[0]);
    drawRect(ctx, 8, 22, 16, 8, PALETTE.terminal[1]);

    // Screen
    drawRect(ctx, 8, 4, 16, 14, PALETTE.terminal[1]);
    drawRect(ctx, 10, 6, 12, 10, '#000000');

    // Screen glow
    drawRect(ctx, 11, 7, 10, 8, PALETTE.terminal[2]);

    // Cyan accent (glowing runes)
    ctx.fillStyle = PALETTE.terminal[3];
    ctx.fillRect(12, 8, 2, 2);
    ctx.fillRect(16, 8, 2, 2);
    ctx.fillRect(14, 11, 4, 2);

    return canvas;
}

function generateDoorLocked() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Floor background
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, PALETTE.stone[1]);

    // Door frame
    drawRect(ctx, 4, 2, 24, 28, PALETTE.doorWood[3]);

    // Door surface
    drawRect(ctx, 6, 4, 20, 24, PALETTE.doorWood[1]);

    // Door planks
    drawRect(ctx, 15, 4, 2, 24, PALETTE.doorWood[3]);

    // Lock (red = locked)
    drawRect(ctx, 20, 14, 4, 4, '#8b0000');
    setPixel(ctx, 21, 15, '#ff4444');

    // Handle
    drawRect(ctx, 8, 14, 3, 4, '#4a4a4a');

    return canvas;
}

function generateDoorOpen() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Dark passage behind
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, '#0a0a0a');

    // Door frame
    drawRect(ctx, 0, 0, 6, TILE_SIZE, PALETTE.doorWood[3]);
    drawRect(ctx, TILE_SIZE - 6, 0, 6, TILE_SIZE, PALETTE.doorWood[3]);
    drawRect(ctx, 0, 0, TILE_SIZE, 4, PALETTE.doorWood[3]);

    // Green glow indicating open/unlocked
    ctx.fillStyle = '#22c55e';
    ctx.fillRect(14, 14, 4, 4);

    return canvas;
}

function generateVoid() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Pure black void
    drawRect(ctx, 0, 0, TILE_SIZE, TILE_SIZE, '#020617');

    return canvas;
}

// ============ PLAYER SPRITES ============

function generateKnightSprite(direction = 'down', frame = 0) {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    const bobOffset = frame % 2 === 0 ? 0 : 1;
    const armSwing = frame % 2 === 0 ? 0 : 1;

    // Knight facing directions
    if (direction === 'down') {
        // Helmet
        drawRect(ctx, 11, 4 + bobOffset, 10, 10, PALETTE.knight[1]);
        drawRect(ctx, 13, 6 + bobOffset, 6, 4, PALETTE.knight[3]); // visor

        // Armor body
        drawRect(ctx, 10, 14 + bobOffset, 12, 10, PALETTE.knight[0]);
        drawRect(ctx, 14, 14 + bobOffset, 4, 2, PALETTE.knightAccent[1]); // gold trim

        // Arms
        drawRect(ctx, 6 + armSwing, 16 + bobOffset, 4, 6, PALETTE.knight[0]);
        drawRect(ctx, 22 - armSwing, 16 + bobOffset, 4, 6, PALETTE.knight[0]);

        // Legs
        drawRect(ctx, 12, 24 + bobOffset, 4, 6, PALETTE.knight[2]);
        drawRect(ctx, 17, 24 + bobOffset, 4, 6, PALETTE.knight[2]);

        // Sword (right hand)
        drawRect(ctx, 24, 14 + bobOffset, 2, 12, '#a0a0a0');
        drawRect(ctx, 23, 12 + bobOffset, 4, 2, PALETTE.knightAccent[1]); // crossguard

    } else if (direction === 'up') {
        // Helmet back
        drawRect(ctx, 11, 4 + bobOffset, 10, 10, PALETTE.knight[0]);

        // Cape
        drawRect(ctx, 10, 14 + bobOffset, 12, 12, '#8b0000');
        drawRect(ctx, 12, 16 + bobOffset, 8, 8, '#6b0000');

        // Arms
        drawRect(ctx, 6 + armSwing, 16 + bobOffset, 4, 6, PALETTE.knight[0]);
        drawRect(ctx, 22 - armSwing, 16 + bobOffset, 4, 6, PALETTE.knight[0]);

        // Legs
        drawRect(ctx, 12, 24 + bobOffset, 4, 6, PALETTE.knight[2]);
        drawRect(ctx, 17, 24 + bobOffset, 4, 6, PALETTE.knight[2]);

    } else if (direction === 'left') {
        // Helmet side
        drawRect(ctx, 12, 4 + bobOffset, 8, 10, PALETTE.knight[1]);
        drawRect(ctx, 10, 6 + bobOffset, 4, 4, PALETTE.knight[3]); // visor

        // Body side
        drawRect(ctx, 12, 14 + bobOffset, 8, 10, PALETTE.knight[0]);

        // Arm with sword
        drawRect(ctx, 6, 14 + bobOffset, 6, 6, PALETTE.knight[0]);
        drawRect(ctx, 4, 10 + bobOffset, 2, 14, '#a0a0a0'); // sword
        drawRect(ctx, 2, 8 + bobOffset, 6, 2, PALETTE.knightAccent[1]); // crossguard

        // Legs
        drawRect(ctx, 14, 24 + bobOffset, 4, 6, PALETTE.knight[2]);

    } else if (direction === 'right') {
        // Mirror of left
        drawRect(ctx, 12, 4 + bobOffset, 8, 10, PALETTE.knight[1]);
        drawRect(ctx, 18, 6 + bobOffset, 4, 4, PALETTE.knight[3]); // visor

        // Body side
        drawRect(ctx, 12, 14 + bobOffset, 8, 10, PALETTE.knight[0]);

        // Arm with sword
        drawRect(ctx, 20, 14 + bobOffset, 6, 6, PALETTE.knight[0]);
        drawRect(ctx, 26, 10 + bobOffset, 2, 14, '#a0a0a0'); // sword
        drawRect(ctx, 24, 8 + bobOffset, 6, 2, PALETTE.knightAccent[1]); // crossguard

        // Legs
        drawRect(ctx, 14, 24 + bobOffset, 4, 6, PALETTE.knight[2]);
    }

    return canvas;
}

// ============ NPC SPRITES ============

function generateMentorSprite() {
    const canvas = createCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');

    // Wizard hat
    drawRect(ctx, 10, 2, 12, 4, PALETTE.wizard[0]);
    drawRect(ctx, 12, 0, 8, 4, PALETTE.wizard[0]);
    drawRect(ctx, 14, -2, 4, 4, PALETTE.wizard[1]);
    setPixel(ctx, 15, 0, PALETTE.wizardAccent[2]); // star on hat

    // Face
    drawRect(ctx, 12, 6, 8, 8, PALETTE.skin[1]);

    // Beard
    drawRect(ctx, 11, 12, 10, 6, PALETTE.wizardAccent[0]);
    drawRect(ctx, 13, 16, 6, 4, PALETTE.wizardAccent[1]);

    // Robes
    drawRect(ctx, 8, 14, 16, 14, PALETTE.wizard[1]);
    drawRect(ctx, 14, 14, 4, 14, PALETTE.wizard[3]); // center stripe

    // Staff
    drawRect(ctx, 24, 4, 2, 24, '#8b4513');
    drawRect(ctx, 22, 2, 6, 4, PALETTE.wizardAccent[2]); // crystal

    // Eyes
    setPixel(ctx, 13, 9, '#000000');
    setPixel(ctx, 18, 9, '#000000');

    return canvas;
}

// ============ UI ELEMENTS ============

function generateHealthBarBg() {
    const canvas = createCanvas(64, 16);
    const ctx = canvas.getContext('2d');

    // Dark background
    drawRect(ctx, 0, 0, 64, 16, PALETTE.uiBg[0]);

    // Border
    ctx.strokeStyle = PALETTE.uiAccent[1];
    ctx.lineWidth = 2;
    ctx.strokeRect(1, 1, 62, 14);

    return canvas;
}

function generateHealthBarFill() {
    const canvas = createCanvas(60, 12);
    const ctx = canvas.getContext('2d');

    // Red health gradient
    const gradient = ctx.createLinearGradient(0, 0, 60, 0);
    gradient.addColorStop(0, '#dc2626');
    gradient.addColorStop(0.5, '#ef4444');
    gradient.addColorStop(1, '#f87171');

    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, 60, 12);

    return canvas;
}

function generateXpBarBg() {
    const canvas = createCanvas(64, 16);
    const ctx = canvas.getContext('2d');

    // Dark background
    drawRect(ctx, 0, 0, 64, 16, PALETTE.uiBg[0]);

    // Border
    ctx.strokeStyle = PALETTE.uiAccent[1];
    ctx.lineWidth = 2;
    ctx.strokeRect(1, 1, 62, 14);

    return canvas;
}

function generateXpBarFill() {
    const canvas = createCanvas(60, 12);
    const ctx = canvas.getContext('2d');

    // Gold XP gradient
    const gradient = ctx.createLinearGradient(0, 0, 60, 0);
    gradient.addColorStop(0, PALETTE.uiAccent[3]);
    gradient.addColorStop(0.5, PALETTE.uiAccent[1]);
    gradient.addColorStop(1, PALETTE.uiAccent[2]);

    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, 60, 12);

    return canvas;
}

// ============ MAIN ============

function saveCanvas(canvas, filepath) {
    const buffer = canvas.toBuffer('image/png');
    const dir = path.dirname(filepath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(filepath, buffer);
    console.log(`Generated: ${filepath}`);
}

async function main() {
    const staticDir = path.join(__dirname, '..', 'static');

    console.log('Generating pixel art assets...\n');

    // Terrain tiles
    saveCanvas(generateGrassTile(), path.join(staticDir, 'tiles/terrain/grass.png'));
    saveCanvas(generateStoneTile(), path.join(staticDir, 'tiles/terrain/stone.png'));
    saveCanvas(generateWaterTile(0), path.join(staticDir, 'tiles/terrain/water.png'));

    // Also save as the default floor tiles for compatibility
    saveCanvas(generateStoneTile(), path.join(staticDir, 'tiles/floor_stone.png'));
    saveCanvas(generateGrassTile(), path.join(staticDir, 'tiles/floor.png'));
    saveCanvas(generateWaterTile(0), path.join(staticDir, 'tiles/floor_tech.png'));

    // Wall tiles
    saveCanvas(generateStoneWall(), path.join(staticDir, 'tiles/walls/stone_wall.png'));
    saveCanvas(generateWallTop(), path.join(staticDir, 'tiles/walls/wall_top.png'));
    saveCanvas(generateStoneWall(), path.join(staticDir, 'tiles/wall.png'));
    saveCanvas(generateWallTop(), path.join(staticDir, 'tiles/wall_top.png'));

    // Interactive tiles
    saveCanvas(generateTerminal(), path.join(staticDir, 'tiles/interactive/terminal.png'));
    saveCanvas(generateDoorLocked(), path.join(staticDir, 'tiles/interactive/door_locked.png'));
    saveCanvas(generateDoorOpen(), path.join(staticDir, 'tiles/interactive/door_open.png'));
    saveCanvas(generateVoid(), path.join(staticDir, 'tiles/interactive/void.png'));

    // Copy to root tiles for compatibility
    saveCanvas(generateTerminal(), path.join(staticDir, 'tiles/terminal.png'));
    saveCanvas(generateDoorLocked(), path.join(staticDir, 'tiles/door_locked.png'));
    saveCanvas(generateDoorOpen(), path.join(staticDir, 'tiles/door_open.png'));
    saveCanvas(generateVoid(), path.join(staticDir, 'tiles/void.png'));

    // Player sprites (4 directions)
    for (const dir of ['down', 'up', 'left', 'right']) {
        saveCanvas(generateKnightSprite(dir, 0), path.join(staticDir, `sprites/player_${dir}.png`));
        // Also save in subdirectory
        saveCanvas(generateKnightSprite(dir, 0), path.join(staticDir, `sprites/player/walk_${dir}.png`));
    }

    // Legacy player sprite
    saveCanvas(generateKnightSprite('down', 0), path.join(staticDir, 'sprites/player.png'));

    // NPC sprites
    saveCanvas(generateMentorSprite(), path.join(staticDir, 'sprites/npcs/mentor.png'));
    saveCanvas(generateMentorSprite(), path.join(staticDir, 'sprites/npc_mentor.png'));

    // UI elements
    saveCanvas(generateHealthBarBg(), path.join(staticDir, 'ui/health_bar_bg.png'));
    saveCanvas(generateHealthBarFill(), path.join(staticDir, 'ui/health_bar_fill.png'));
    saveCanvas(generateXpBarBg(), path.join(staticDir, 'ui/xp_bar_bg.png'));
    saveCanvas(generateXpBarFill(), path.join(staticDir, 'ui/xp_bar_fill.png'));

    console.log('\nDone! All pixel art assets generated.');
}

main().catch(console.error);

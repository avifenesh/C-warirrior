#!/usr/bin/env node
/**
 * Fix sprite transparency - removes white/light backgrounds
 */

const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const STATIC_DIR = path.join(__dirname, '..', 'src-ui', 'static');

// Directories to process
const SPRITE_DIRS = [
    'sprites',
    'tiles/terrain',
    'tiles/walls',
    'tiles/interactive',
    'tiles',
    'ui'
];

// Threshold for what counts as "white" (0-255, higher = stricter)
const WHITE_THRESHOLD = 240;
const TRANSPARENCY_THRESHOLD = 200; // How close to white to make transparent

async function removeWhiteBackground(inputPath) {
    try {
        const image = sharp(inputPath);
        const metadata = await image.metadata();

        // Get raw pixel data
        const { data, info } = await image
            .ensureAlpha()
            .raw()
            .toBuffer({ resolveWithObject: true });

        const pixels = new Uint8Array(data);
        const { width, height, channels } = info;

        // Process each pixel
        for (let i = 0; i < pixels.length; i += channels) {
            const r = pixels[i];
            const g = pixels[i + 1];
            const b = pixels[i + 2];

            // Check if pixel is white/near-white
            if (r > TRANSPARENCY_THRESHOLD && g > TRANSPARENCY_THRESHOLD && b > TRANSPARENCY_THRESHOLD) {
                // Make it transparent
                pixels[i + 3] = 0; // Set alpha to 0
            }
        }

        // Save the processed image
        await sharp(pixels, {
            raw: {
                width,
                height,
                channels
            }
        })
            .png()
            .toFile(inputPath + '.tmp');

        // Replace original
        fs.renameSync(inputPath + '.tmp', inputPath);

        console.log(`  Fixed: ${path.basename(inputPath)}`);
        return true;
    } catch (err) {
        console.error(`  Error processing ${inputPath}: ${err.message}`);
        return false;
    }
}

async function processDirectory(dirPath) {
    const fullPath = path.join(STATIC_DIR, dirPath);

    if (!fs.existsSync(fullPath)) {
        console.log(`Directory not found: ${dirPath}`);
        return;
    }

    console.log(`\nProcessing: ${dirPath}/`);

    const files = fs.readdirSync(fullPath);

    for (const file of files) {
        if (file.endsWith('.png')) {
            await removeWhiteBackground(path.join(fullPath, file));
        }
    }
}

// Create simple colored tiles for fallback
async function createSimpleTiles() {
    console.log('\nCreating simple fallback tiles...');

    const tiles = {
        'tiles/terrain/grass.png': { color: '#3d7a37', pattern: 'grass' },
        'tiles/terrain/stone.png': { color: '#4a4a4a', pattern: 'stone' },
        'tiles/floor.png': { color: '#5a5a5a', pattern: 'floor' },
        'tiles/void.png': { color: '#0a0a14', pattern: 'solid' },
        'tiles/wall.png': { color: '#3a3a3a', pattern: 'brick' },
    };

    for (const [tilePath, config] of Object.entries(tiles)) {
        const fullPath = path.join(STATIC_DIR, tilePath);
        const dir = path.dirname(fullPath);

        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }

        try {
            // Create a 32x32 tile with the specified color
            const size = 32;
            const channels = 4;
            const pixels = Buffer.alloc(size * size * channels);

            // Parse color
            const hex = config.color.replace('#', '');
            const r = parseInt(hex.substring(0, 2), 16);
            const g = parseInt(hex.substring(2, 4), 16);
            const b = parseInt(hex.substring(4, 6), 16);

            // Fill with color and add some variation for texture
            for (let y = 0; y < size; y++) {
                for (let x = 0; x < size; x++) {
                    const i = (y * size + x) * channels;

                    let variation = 0;
                    if (config.pattern === 'grass') {
                        // Add grass-like variation
                        variation = Math.random() * 20 - 10;
                    } else if (config.pattern === 'stone' || config.pattern === 'brick') {
                        // Add stone texture variation
                        variation = Math.random() * 15 - 7;
                        // Add grid lines for bricks
                        if (config.pattern === 'brick' && (x % 8 === 0 || y % 8 === 0)) {
                            variation -= 20;
                        }
                    }

                    pixels[i] = Math.max(0, Math.min(255, r + variation));
                    pixels[i + 1] = Math.max(0, Math.min(255, g + variation));
                    pixels[i + 2] = Math.max(0, Math.min(255, b + variation));
                    pixels[i + 3] = 255; // Fully opaque
                }
            }

            await sharp(pixels, {
                raw: { width: size, height: size, channels }
            })
                .png()
                .toFile(fullPath);

            console.log(`  Created: ${tilePath}`);
        } catch (err) {
            console.error(`  Error creating ${tilePath}: ${err.message}`);
        }
    }
}

async function main() {
    console.log('Fixing sprite transparency...\n');

    // Process all sprite directories
    for (const dir of SPRITE_DIRS) {
        await processDirectory(dir);
    }

    // Create simple fallback tiles
    await createSimpleTiles();

    console.log('\nDone!');
}

main().catch(console.error);
